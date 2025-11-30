use std::collections::VecDeque;

use atoi_simd::parse;
use hashbrown::HashMap;
use nohash::BuildNoHashHasher;

#[derive(Debug, PartialEq, Eq)]
pub enum ParameterMode {
    Direct,
    Immediate,
    Relative,
}

impl ParameterMode {
    pub fn new(code: &str) -> Self {
        match code {
            "0" => ParameterMode::Direct,
            "1" => ParameterMode::Immediate,
            "2" => ParameterMode::Relative,
            _ => panic!("Not a valid paramter {code}"),
        }
    }
}

impl From<i64> for ParameterMode {
    fn from(value: i64) -> Self {
        match value {
            0 => ParameterMode::Direct,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Not a valid paramter {value}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntCode {
    pub data: HashMap<usize, i64, BuildNoHashHasher<usize>>,
    pub current_pos: usize,
    pub output: Vec<i64>,
    pub input: VecDeque<i64>,
    pub quit: bool,
    pub relative_base: i64,
    pub waiting_for_input: Option<i64>,
}
impl IntCode {
    pub fn new(data: &str) -> Self {
        let data = data
            .trim()
            .split(',')
            .filter_map(|s| parse(s.as_bytes()).ok())
            .enumerate()
            .collect();

        IntCode {
            data,
            current_pos: 0,
            output: Vec::new(),
            input: VecDeque::new(),
            quit: false,
            waiting_for_input: None,
            relative_base: 0,
        }
    }

    pub fn add_input(&mut self, input: i64) {
        if let Some(value_a) = self.waiting_for_input {
            self.waiting_for_input = None;
            *self.data.entry(value_a as usize).or_default() = input;
            self.current_pos += 2;
        } else {
            self.input.push_back(input);
        }
    }
    pub fn get_last_output(&self) -> i64 {
        *self.output.iter().last().unwrap()
    }

    pub fn get_value_at(&mut self, idx: usize, parameter_mode: ParameterMode) -> i64 {
        let base: i64 = *self.data.entry(idx).or_default();
        match parameter_mode {
            ParameterMode::Direct => self.get_value_at(base as usize, ParameterMode::Immediate),
            ParameterMode::Immediate => base,
            ParameterMode::Relative => self.get_value_at(
                (base + self.relative_base) as usize,
                ParameterMode::Immediate,
            ),
        }
    }
    pub fn get_literal_value_at(&mut self, idx: usize, parameter_mode: ParameterMode) -> i64 {
        let base = self.data.entry(idx).or_default();
        match parameter_mode {
            ParameterMode::Direct | ParameterMode::Immediate => *base,
            ParameterMode::Relative => *base + self.relative_base,
        }
    }

    pub fn is_quit(&self) -> bool {
        self.quit
    }

    pub fn process_step(&mut self, stop_on_output: bool) -> bool {
        let mut keep_going = true;

        while keep_going {
            let (opcode, first_parameter_mode, second_parameter_mode, third_parameter_mode) =
                parse_opcode(self.data.get(&self.current_pos).unwrap());
            let current_pos = self.current_pos;

            match opcode {
                1 => {
                    let value_a = self.get_value_at(current_pos + 1, first_parameter_mode);
                    let value_b = self.get_value_at(current_pos + 2, second_parameter_mode);
                    let value_idx_c =
                        self.get_literal_value_at(current_pos + 3, third_parameter_mode);
                    self.data.insert(value_idx_c as usize, value_a + value_b);
                    self.current_pos += 4;
                }
                2 => {
                    let value_a = self.get_value_at(current_pos + 1, first_parameter_mode);
                    let value_b = self.get_value_at(current_pos + 2, second_parameter_mode);
                    let value_idx_c =
                        self.get_literal_value_at(current_pos + 3, third_parameter_mode);
                    self.data.insert(value_idx_c as usize, value_a * value_b);
                    self.current_pos += 4;
                }
                3 => {
                    let value_a = self.get_literal_value_at(current_pos + 1, first_parameter_mode);
                    if let Some(input) = self.input.pop_front() {
                        self.data.insert(value_a as usize, input);
                        self.current_pos += 2;
                    } else {
                        keep_going = false;
                        self.waiting_for_input = Some(value_a);
                    }
                }
                4 => {
                    let value_a = self.get_value_at(current_pos + 1, first_parameter_mode);
                    self.output.push(value_a);
                    self.current_pos += 2;
                    if stop_on_output {
                        keep_going = false;
                    }
                }
                // jump-if-true - if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
                5 => {
                    let value_a = self.get_value_at(current_pos + 1, first_parameter_mode);
                    if value_a != 0 {
                        self.current_pos =
                            self.get_value_at(current_pos + 2, second_parameter_mode) as usize;
                    } else {
                        self.current_pos += 3;
                    }
                }
                // jump-if-false
                6 => {
                    let value_a = self.get_value_at(current_pos + 1, first_parameter_mode);
                    if value_a == 0 {
                        self.current_pos =
                            self.get_value_at(current_pos + 2, second_parameter_mode) as usize;
                    } else {
                        self.current_pos += 3;
                    }
                }
                // less than
                7 => {
                    let value_a = self.get_value_at(current_pos + 1, first_parameter_mode);
                    let value_b = self.get_value_at(current_pos + 2, second_parameter_mode);
                    let value_c = self.get_literal_value_at(current_pos + 3, third_parameter_mode);
                    let value = if value_a < value_b { 1 } else { 0 };
                    self.data.insert(value_c as usize, value);
                    self.current_pos += 4;
                }
                8 => {
                    let value_a = self.get_value_at(current_pos + 1, first_parameter_mode);
                    let value_b = self.get_value_at(current_pos + 2, second_parameter_mode);
                    let value_c = self.get_literal_value_at(current_pos + 3, third_parameter_mode);
                    let value = if value_a == value_b { 1 } else { 0 };
                    self.data.insert(value_c as usize, value);
                    self.current_pos += 4;
                }
                9 => {
                    let value_a = self.get_value_at(current_pos + 1, first_parameter_mode);
                    self.relative_base += value_a;
                    self.current_pos += 2;
                }
                99 => {
                    self.quit = true;
                    keep_going = false;
                }
                _ => {
                    unreachable!("Should not have hit here")
                }
            }
        }

        keep_going
    }

    pub fn process(&mut self, stop_on_output: bool) {
        let mut keep_going = true;

        while keep_going {
            keep_going = self.process_step(stop_on_output)
        }
    }
}

fn parse_opcode(n: &i64) -> (usize, ParameterMode, ParameterMode, ParameterMode) {
    let opcode = (n % 100) as usize;

    let first_mode_val = (n / 100) % 10;
    let first_parameter_mode = ParameterMode::from(first_mode_val);

    let second_mode_val = (n / 1000) % 10;
    let second_parameter_mode = ParameterMode::from(second_mode_val);

    let third_mode_val = (n / 10000) % 10;
    let third_parameter_mode = ParameterMode::from(third_mode_val);

    (
        opcode,
        first_parameter_mode,
        second_parameter_mode,
        third_parameter_mode,
    )
}

pub fn intcode(data: &str) -> Vec<i64> {
    let mut ic = IntCode::new(data);

    ic.process(false);
    let max = *ic.data.keys().max().unwrap();
    (0..=max)
        .map(|i| ic.data.get(&i).unwrap_or(&0))
        .copied()
        .collect()
}

/// Small builder for better ergonomics around setting up an intcode computer
#[derive(Debug, Default)]
pub struct IntCodeBuilder {
    pub input: VecDeque<i64>,
    pub quit: bool,
}
impl IntCodeBuilder {
    /// set an input item. Each time this is called, it is added to the back of the list
    pub fn input(mut self, input: i64) -> Self {
        self.input.push_back(input);
        self
    }

    /// Pushes an input to the front of the queue
    pub fn input_prepend(mut self, input: i64) -> Self {
        self.input.push_front(input);
        self
    }

    /// Final build step to create the IntCode computer. Takes the string input for the intcode
    /// computer as its only parameter and returns an `IntCode` instance.
    pub fn build(self, data: &str) -> IntCode {
        let mut ic = IntCode::new(data);
        ic.quit = self.quit;
        ic.input = self.input;
        ic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a1() {
        assert_eq!(
            intcode("1,9,10,3,2,3,11,0,99,30,40,50"),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }
    #[test]
    fn test_a2() {
        assert_eq!(intcode("1,0,0,0,99"), vec![2, 0, 0, 0, 99]);
    }
    #[test]
    fn test_a3() {
        assert_eq!(intcode("2,3,0,3,99"), vec![2, 3, 0, 6, 99]);
    }
    #[test]
    fn test_a4() {
        assert_eq!(intcode("2,4,4,5,99,0"), vec![2, 4, 4, 5, 99, 9801]);
    }
    #[test]
    fn test_a5() {
        assert_eq!(
            intcode("1,1,1,4,99,5,6,0,99"),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
    #[test]
    fn test_1() {
        let data = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let ic = IntCode::new(data);

        let mut ic1 = ic.clone();
        ic1.add_input(1);
        println!("{:#?}", ic1.input);
        ic1.process(false);
        assert_eq!(ic1.output[0], 999);

        let mut ic2 = ic.clone();
        ic2.add_input(8);
        ic2.process(false);
        assert_eq!(ic2.output[0], 1000);

        let mut ic3 = ic.clone();
        ic3.add_input(9);
        ic3.process(false);
        assert_eq!(ic3.output[0], 1001);
    }

    #[test]
    fn test_opcode_9() {
        let data = "109,19,204,-34";

        let mut ic = IntCode::new(data);
        *ic.data.entry(1984).or_default() = 2323;
        *ic.data.entry(1985).or_default() = 3232;
        ic.relative_base = 2000;

        ic.process(true);
        assert_eq!(ic.relative_base, 2019);
        // ic.process_step(true);
        assert_eq!(ic.output.first().unwrap(), &3232);
    }

    #[test]
    fn test_opcode_9_example_1() {
        let mut ic = IntCode::new("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");

        ic.process(false);

        assert_eq!(
            ic.output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn test_opcode_9_example_2() {
        let mut ic = IntCode::new("1102,34915192,34915192,7,4,7,99,0");

        ic.process(true);

        assert_eq!(ic.output.first().unwrap().to_string().len(), 16);
    }

    #[test]
    fn test_opcode_9_example_3() {
        let data = "104,1125899906842624,99";

        let mut ic = IntCode::new(data);

        ic.process(true);
        assert_eq!(*ic.output.first().unwrap(), 1125899906842624);
    }

    #[test]
    fn reddit_test_case_1() {
        let mut ic = IntCode::new("109,-1,4,1,99");
        ic.process(false);
        assert_eq!(*ic.output.first().unwrap(), -1);
    }
    #[test]
    fn reddit_test_case_2() {
        let mut ic = IntCode::new("109,-1,104,1,99");
        ic.process(false);
        assert_eq!(*ic.output.first().unwrap(), 1);
    }
    #[test]
    fn reddit_test_case_3() {
        let mut ic = IntCode::new("109,-1,204,1,99");
        ic.process(false);
        assert_eq!(*ic.output.first().unwrap(), 109);
    }
    #[test]
    fn reddit_test_case_4() {
        let mut ic = IntCode::new("109,1,9,2,204,-6,99");
        ic.process(false);
        assert_eq!(*ic.output.first().unwrap(), 204);
    }
    #[test]
    fn reddit_test_case_5() {
        let mut ic = IntCode::new("109,1,109,9,204,-6,99");
        ic.process(false);
        assert_eq!(*ic.output.first().unwrap(), 204);
    }
    #[test]
    fn reddit_test_case_6() {
        let mut ic = IntCode::new("109,1,209,-1,204,-106,99");
        ic.process(false);
        assert_eq!(*ic.output.first().unwrap(), 204);
    }
    #[test]
    fn reddit_test_case_7() {
        let mut ic = IntCode::new("109,1,3,3,204,2,99");
        ic.add_input(32);
        ic.process(false);
        assert_eq!(*ic.output.first().unwrap(), 32);
    }
    #[test]
    fn test_parse_opcode() {
        assert_eq!(
            parse_opcode(&1002),
            (
                2,
                ParameterMode::Direct,
                ParameterMode::Immediate,
                ParameterMode::Direct
            )
        );
        assert_eq!(
            parse_opcode(&1101),
            (
                1,
                ParameterMode::Immediate,
                ParameterMode::Immediate,
                ParameterMode::Direct
            )
        );
    }
}
