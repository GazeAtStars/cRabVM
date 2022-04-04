use nom::types::CompleteStr;
use crate::asm::instruction_parser::{AsmInstruction, parse_instruction};

#[derive(Debug, PartialEq)]
pub struct AsmProgram {
    pub instructions: Vec<AsmInstruction>,
}

impl AsmProgram {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for instruction in &self.instructions {
            bytes.append(&mut instruction.to_bytes());
        }
        bytes
    }
}

named!(pub parse_program<CompleteStr, AsmProgram>,
    do_parse!(
        instructions: many1!(parse_instruction) >>
        (AsmProgram { instructions })
    )
);

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::instructions::Opcode;
    #[test]
    fn test_parse_program() {
        let program = parse_program(CompleteStr("set $0 #100\n"));
        assert_eq!(program.is_ok(), true);
        let (rest, instruction) = program.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(instruction.instructions.len(), 1);
        assert_eq!(instruction.instructions[0].opcode, crate::asm::Token::Opcode { code:  Opcode::SET });
    }

    #[test]
    fn test_program_to_bytes() {
        let program = parse_program(CompleteStr("set $0 #100\n"));
        assert_eq!(program.is_ok(), true);
        let (_, program) = program.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }
}
