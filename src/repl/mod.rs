pub mod parser;

use std::{
    io::{self, Write},
    num::ParseIntError,
};

use nom::types::CompleteStr;

use crate::vm::VM;
use crate::repl::parser::Parser;
use crate::asm::program_parser::parse_program;

pub static BANNER: &'static str = "Hello welcome to the incomplete lang REPL owo";
pub static PROMPT: &'static str = "> ";
pub static PREFIX: &'static str = ".";

#[derive(Default)]
pub struct REPL {
    vm: VM,
    buffer: Vec<String>,
    hex_mode: bool,
}

impl REPL {
    pub fn new(vm: VM) -> Self {
        Self {
            vm,
            buffer: Vec::new(),
            hex_mode: false,
        }
    }
    pub fn message(&mut self, msg: String) {
        println!("{}", msg);
    }
    pub fn prompt(&mut self) {
        print!("{} ", PROMPT);
        io::stdout().flush().expect("Unable to flush stdout");
    }

    #[allow(dead_code)]
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(' ').collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }

    fn execute(&mut self, input: &str) {
        let args = Parser::tokenize(input);
        match args[0] {
            ".exit" | ".quit" => self.quit(&args[1..]),
            ".history" => self.history(&args[1..]),
            ".program" => self.program(&args[1..]),
            ".clear_program" => self.clear_program(&args[1..]),
            ".clear_registers" => self.clear_registers(&args[1..]),
            ".registers" => self.registers(&args[1..]),
            ".register" => self.register(&args[1..]),
            ".hex_mode" => self.hex_mode(&args[1..]),
            _ => {
                self.message("Invalid command!".to_string());
            }
        };
    }

    fn quit(&mut self, _args: &[&str]) {
        self.message("Quiting...".to_string());
        std::process::exit(0);
    }

    fn history(&mut self, _args: &[&str]) {
        let mut results = vec![];
        for command in &self.buffer {
            results.push(command.clone());
        }
        self.message(format!("{:#?}", results));
    }
    fn program(&mut self, _args: &[&str]) {
        self.message("Listing instructions currently in VM's program vector: ".to_string());
        let mut results = vec![];
        for instruction in &self.vm.program {
            results.push(instruction.clone())
        }
        self.message(format!("{:#?}", results));
        self.message("End of Program Listing".to_string());
    }

    fn clear_program(&mut self, _args: &[&str]) {
        self.vm.program.clear();
    }

    fn clear_registers(&mut self, _args: &[&str]) {
        self.message("Setting all registers to 0".to_string());
        for i in 0..self.vm.registers.len() {
            self.vm.registers[i] = 0;
        }
        self.message("Done!".to_string());
    }

    fn registers(&mut self, _args: &[&str]) {
        self.message("Listing registers and all contents:".to_string());
        let mut results = vec![];
        for register in &self.vm.registers {
            results.push(register.clone());
        }
        self.message(format!("{:#?}", results));
        self.message("End of Register Listing".to_string());
    }

    fn register(&mut self, args: &[&str]) {
        if args.len() != 1 {
            self.message(format!("Register 0 contains the value {}", self.vm.registers[0]));
            return;
        }
        let register_index = args[0].parse::<usize>().unwrap();
        if register_index >= self.vm.registers.len() {
            self.message("Invalid register index".to_string());
            return;
        }
        self.message(format!("Register {} contains the value {}", register_index, self.vm.registers[register_index]));
    }

    fn hex_mode(&mut self, args: &[&str]) {
        if args.len() != 1 {
            self.message("Entering hex mode".to_string());
            self.hex_mode = true;
        } else if args[0] == "disable" || args[0] == "off" {
            self.message("Exiting hex mode".to_string());
            self.hex_mode = false;
        }
    }

    pub fn run(&mut self) {
        self.message(BANNER.to_string());
        self.prompt();
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut buffer).expect("Failed to read line");
            let copy = buffer.clone();
            self.buffer.push(copy);
            if buffer.starts_with(PREFIX) {
                self.execute(&buffer);
                self.prompt();
            } else {
                if self.hex_mode{
                        let program = self.parse_hex(CompleteStr(&buffer).trim());
                        match program {
                            Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte)
                            }
                        }
                        Err(_e) => {
                            println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.")
                        }
                    }
                } else {

                    let program = match parse_program(CompleteStr(&buffer)) {
                      Ok((_rest,  program)) => program,
                      Err(_) => {
                          self.message("Unable to parse instruction.".to_string());
                          continue;
                      }
                  };
                  self.vm.program.append(&mut program.to_bytes());
                }
                self.vm.run_once();
                self.prompt();
            }
        }
    }
}
