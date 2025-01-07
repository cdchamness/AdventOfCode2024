use anyhow::Result;
use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    // let input_file = fs::File::open("puzzle_input_example.txt")?;
    // let input_file = fs::File::open("puzzle_input_example2.txt")?;
    let input_file = fs::File::open("puzzle_input.txt")?;
    let input_lines = BufReader::new(input_file).lines();

    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut program = Vec::new();

    for line in input_lines {
        match line {
            Ok(input_string) => {
                if input_string.starts_with("Register A: ") {
                    register_a = input_string
                        .trim_start_matches("Register A: ")
                        .parse::<usize>()
                        .expect("Unable to parse Register A");
                } else if input_string.starts_with("Register B: ") {
                    register_b = input_string
                        .trim_start_matches("Register B: ")
                        .parse::<usize>()
                        .expect("Unable to parse Register B");
                } else if input_string.starts_with("Register C: ") {
                    register_c = input_string
                        .trim_start_matches("Register C: ")
                        .parse::<usize>()
                        .expect("Unable to parse Register C");
                } else if input_string.starts_with("Program: ") {
                    program = input_string
                        .trim_start_matches("Program: ")
                        .split(',')
                        .filter_map(|v| v.parse::<u8>().ok())
                        .collect::<Vec<u8>>();
                }
            }
            Err(e) => {
                return Err(anyhow::Error::new(e));
            }
        }
    }
    let mut computer = ThreeBitComputer::new(register_a, register_b, register_c, program);
    computer.run();
    computer.print_ouput();

    let mut potential_vals = vec![0];
    'a: while let Some(val) = potential_vals.pop() {
        for i in 0..8 {
            let next_val = val + i;
            computer.reset(next_val);
            computer.run();
            if computer.program == computer.output_stream {
                println!("Solution Found: {}", next_val);
                break 'a;
            } else if computer.program.ends_with(&computer.output_stream) {
                let shifted = next_val << 3;
                potential_vals.push(shifted);
            }
        }
    }

    Ok(())
}

struct ThreeBitComputer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    instruction_pointer: usize,
    program: Vec<u8>,
    output_stream: Vec<u8>,
    exit_code: Option<u8>,
}

impl ThreeBitComputer {
    fn reset(&mut self, val: usize) {
        self.register_a = val;
        self.register_b = 0;
        self.register_c = 0;
        self.instruction_pointer = 0;
        self.output_stream.clear();
        self.exit_code = None;
    }

    fn new(
        register_a: usize,
        register_b: usize,
        register_c: usize,
        program: Vec<u8>,
    ) -> ThreeBitComputer {
        ThreeBitComputer {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
            program,
            output_stream: Vec::new(),
            exit_code: None,
        }
    }

    fn run(&mut self) {
        while self.exit_code.is_none() {
            self.update();
        }
    }

    fn print_ouput(&self) {
        // display output
        let mut output = String::new();
        for val in &self.output_stream {
            if !output.is_empty() {
                output += ",";
            }
            output += val.to_string().as_str();
        }
        println!("Program Output: {:?}", output);
    }

    fn halt(&mut self, code: u8) {
        self.exit_code = Some(code)
    }

    fn update(&mut self) {
        if let Some(opcode) = self.program.get(self.instruction_pointer) {
            if let Some(operand) = self.program.get(self.instruction_pointer + 1) {
                // self.display();
                // println!("Running: {} {}\n", opcode, operand);
                match opcode {
                    0 => self.adv(*operand),
                    1 => self.bxl(*operand),
                    2 => self.bst(*operand),
                    3 => self.jnz(*operand),
                    4 => self.bxc(*operand),
                    5 => self.out(*operand),
                    6 => self.bdv(*operand),
                    7 => self.cdv(*operand),
                    _ => {
                        eprintln!("Unrecognized operand!!");
                        self.halt(1);
                    }
                }
            }
        } else {
            // cannot read opcode -> program halts
            self.halt(0);
        }
    }

    fn combo_operand(&mut self, operand: u8) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7..=u8::MAX => {
                eprintln!("Invalid operand! Program Halt!");
                self.halt(2);
                2
            }
        }
    }

    fn adv(&mut self, operand: u8) {
        self.register_a >>= self.combo_operand(operand);
        self.instruction_pointer += 2;
    }

    fn bxl(&mut self, operand: u8) {
        self.register_b ^= operand as usize;
        self.instruction_pointer += 2;
    }

    fn bst(&mut self, operand: u8) {
        self.register_b = self.combo_operand(operand) % 8;
        self.instruction_pointer += 2;
    }

    fn jnz(&mut self, operand: u8) {
        if self.register_a != 0 {
            self.instruction_pointer = operand as usize;
        } else {
            self.instruction_pointer += 2;
        }
    }

    fn bxc(&mut self, _: u8) {
        self.register_b ^= self.register_c;
        self.instruction_pointer += 2;
    }

    fn out(&mut self, operand: u8) {
        let val = (self.combo_operand(operand) % 8) as u8;
        self.output_stream.push(val);
        self.instruction_pointer += 2;
    }

    fn bdv(&mut self, operand: u8) {
        self.register_b = self.register_a >> self.combo_operand(operand);
        self.instruction_pointer += 2;
    }

    fn cdv(&mut self, operand: u8) {
        self.register_c = self.register_a >> self.combo_operand(operand);
        self.instruction_pointer += 2;
    }
}
