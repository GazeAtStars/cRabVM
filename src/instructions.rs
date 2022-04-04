use nom::types::CompleteStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Opcode {
    SET,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,
    JNEQ,
    IGL
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::SET,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            5 => Opcode::HLT,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTQ,
            14 => Opcode::LTQ,
            15 => Opcode::JEQ,
            16 => Opcode::JNEQ,
            100 => Opcode::IGL,
            _ => Opcode::IGL
        }
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> Self {
        match op {
            Opcode::SET => 0,
            Opcode::ADD => 1,
            Opcode::SUB => 2,
            Opcode::MUL => 3,
            Opcode::DIV => 4,
            Opcode::HLT => 5,
            Opcode::JMP => 6,
            Opcode::JMPF => 7,
            Opcode::JMPB => 8,
            Opcode::EQ => 9,
            Opcode::NEQ => 10,
            Opcode::GT => 11,
            Opcode::LT => 12,
            Opcode::GTQ => 13,
            Opcode::LTQ => 14,
            Opcode::JEQ => 15,
            Opcode::JNEQ => 16,
            Opcode::IGL => 100,
        }
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(s: CompleteStr<'a>) -> Self {
        let lowercase = s.to_lowercase();
        match CompleteStr(&lowercase) {
            CompleteStr("set") => Opcode::SET,
            CompleteStr("add") => Opcode::ADD,
            CompleteStr("sub") => Opcode::SUB,
            CompleteStr("mul") => Opcode::MUL,
            CompleteStr("div") => Opcode::DIV,
            CompleteStr("hlt") => Opcode::HLT,
            CompleteStr("jmp") => Opcode::JMP,
            CompleteStr("jmpf") => Opcode::JMPF,
            CompleteStr("jmpb") => Opcode::JMPB,
            CompleteStr("eq") => Opcode::EQ,
            CompleteStr("neq") => Opcode::NEQ,
            CompleteStr("gt") => Opcode::GT,
            CompleteStr("lt") => Opcode::LT,
            CompleteStr("gtq") => Opcode::GTQ,
            CompleteStr("ltq") => Opcode::LTQ,
            CompleteStr("jeq") => Opcode::JEQ,
            CompleteStr("jneq") => Opcode::JNEQ,
            _ => Opcode::IGL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
      let instruction = Instruction::new(Opcode::HLT);
      assert_eq!(instruction.opcode, Opcode::HLT);
    }

    #[test]
    fn test_int_to_opcode() {
        let opcode = Opcode::from(0);
        assert_eq!(opcode, Opcode::SET);
    }

    #[test]
    fn test_opcode_to_int() {
        let opcode = Opcode::SET;
        assert_eq!(opcode as u8, 0);
    }

    #[test]
    fn test_str_to_opcode() {
        let opcode = Opcode::from(CompleteStr("set"));
        assert_eq!(opcode, Opcode::SET);
        let opcode = Opcode::from(CompleteStr("SET"));
        assert_eq!(opcode, Opcode::SET);
    }
}
