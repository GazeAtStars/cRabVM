use crate::instructions::Opcode;
#[derive(Debug, Default)]
pub struct VM {
    pub registers: [i32; 32],
    pub pcounter: usize,
    pub program: Vec<u8>,
    pub remainder: u32,
    pub is_equal: bool,
    pub is_greater: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 32],
            pcounter: 0,
            program: vec![],
            remainder: 0,
            is_equal: false,
            is_greater: false,
        }
    }
    fn get_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pcounter]);
        self.pcounter += 1;
        return opcode;
    }
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }
    pub fn run_once(&mut self) -> bool {
        self.execute_instruction()
    }
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pcounter];
        self.pcounter += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result =
            ((self.program[self.pcounter] as u16) << 8) | self.program[self.pcounter + 1] as u16;
        self.pcounter += 2;
        return result;
    }
    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);
    }
    pub fn add_bytes(&mut self, mut b: Vec<u8>) {
        self.program.append(&mut b);
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pcounter >= self.program.len() {
            // If this happens, something broke
            return false;
        }
        match self.get_opcode() {
            Opcode::SET => {
                let register = self.next_8_bits() as usize;
                let number = i32::from(self.next_16_bits());
                self.registers[register] = number;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            },
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            },
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            },
            Opcode::JMP => {
                self.pcounter = self.registers[self.next_8_bits() as usize] as usize;
            },
            Opcode::JMPF => {
                self.pcounter += self.registers[self.next_8_bits() as usize] as usize;
            },
            Opcode::JMPB => {
                self.pcounter -= self.registers[self.next_8_bits() as usize] as usize;
            },
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 == register2 {
                    self.is_equal = true
                } else  {
                    self.is_equal = false
                }
                self.next_8_bits();
            },
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 != register2 {
                    self.is_equal = true
                } else  {
                    self.is_equal = false
                }
                self.next_8_bits();
            },
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 > register2 {
                    self.is_greater = true
                } else  {
                    self.is_greater = false
                }
                self.next_8_bits();
            },
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 < register2 {
                    self.is_greater = true
                } else  {
                    self.is_greater = false
                }
                self.next_8_bits();
            },
            Opcode::GTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 >= register2 {
                    self.is_equal = true;
                    self.is_greater = true;
                } else  {
                    self.is_equal = false;
                    self.is_greater = false;
                }
                self.next_8_bits();
            },
            Opcode::LTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 <= register2 {
                    self.is_equal = true;
                    self.is_greater = true;
                } else  {
                    self.is_equal = false;
                    self.is_greater = false;
                }
                self.next_8_bits();
            },
            Opcode::JEQ => {
                if self.is_equal {
                    self.pcounter = self.registers[self.next_8_bits() as usize] as usize;
                }
            },
            Opcode::JNEQ => {
                if !self.is_equal {
                    self.pcounter = self.registers[self.next_8_bits() as usize] as usize;
                }
            },
            Opcode::IGL => {
                let opcode = self.get_opcode();
                panic!("Illegal instruction encountered\nOpcode: {:?}, No. {}", opcode, opcode as u8);
            }
        }
        return true;
    }
}
pub fn get_test_vm() -> VM {
    let mut test_vm = VM::new();
    test_vm.registers[0] = 5;
    test_vm.registers[1] = 1;
    test_vm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_hlt_opcode() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pcounter, 1);
    }

    /*
    #[test]
    fn test_igl_opcode() {
        let mut test_vm = VM::new();
        let test_bytes = vec![120, 123, 12, 12];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pcounter, 1);
    }*/
    // This would always fail, since igl is a illegal instruction

    #[test]
    fn test_set_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![1, 1, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 2);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![2, 1, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 0);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![3, 1, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 1);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![4, 1, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 0);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[1] = 1;
        test_vm.program = vec![6, 1, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pcounter, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pcounter, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[1] = 6;
        test_vm.program = vec![0, 0, 0, 10, 8, 1, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pcounter, 4);
    }
    #[test]
    fn test_eq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.is_equal, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.is_equal, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.is_equal, false);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.is_equal, true);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.is_greater, false);
        test_vm.registers[1] = 9;
        test_vm.run_once();
        assert_eq!(test_vm.is_greater, true);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.is_greater, false);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.is_greater, true);
    }

    #[test]
    fn test_gtq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.is_greater, true);
        assert_eq!(test_vm.is_equal, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.is_greater, false);
        assert_eq!(test_vm.is_equal, false);
    }

    #[test]
    fn test_ltq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.is_greater, true);
        assert_eq!(test_vm.is_equal, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.is_greater, true);
        assert_eq!(test_vm.is_equal, true);
    }
    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.is_equal = true;
        test_vm.program = vec![15, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pcounter, 7);
    }

    #[test]
    fn test_jneq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.is_equal = false;
        test_vm.program = vec![16, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pcounter, 7);
    }
}
