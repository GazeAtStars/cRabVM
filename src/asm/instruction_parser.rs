use crate::asm::*;
use crate::asm::parser::*;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AsmInstruction {
    pub opcode: Token,
    pub arg1: Option<Token>,
    pub arg2: Option<Token>,
    pub arg3: Option<Token>,
}

impl AsmInstruction {
    fn extract_arg(token: &Token, bytes: &mut Vec<u8>) {
        match token {
            Token::Register { num } => bytes.push(*num),
            Token::Integer { num } => {
                let c = *num as u16;
                bytes.push((c >> 8) as u8);
                bytes.push(c as u8);
            }
            _ => panic!("Invalid argument type"),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        match self.opcode {
            Token::Opcode { code } => bytes.push(code as u8),
            _ => panic!("Invalid opcode"),
        };
        for arg in &[&self.arg1, &self.arg2, &self.arg3] {
            if let Some(token) = arg { AsmInstruction::extract_arg(&token, &mut bytes) }
        }
        return bytes;
    }
}

named!(pub parse_instruction<CompleteStr, AsmInstruction>,
    do_parse!(
        opcode: opcode >>
        register: register >>
        integer: integer >>
        (AsmInstruction{opcode: opcode, arg1: Some(register), arg2: Some(integer), arg3: None})
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::Opcode;

    #[test]
    fn test_parse_instruction() {
        let input = CompleteStr("set $0 #10\n");
        let expected = AsmInstruction {
            opcode: Token::Opcode{code: Opcode::SET},
            arg1: Some(Token::Register{num: 0}),
            arg2: Some(Token::Integer{num: 10}),
            arg3: None,
        };
        assert_eq!(parse_instruction(input), Ok((CompleteStr(""), expected)));
    }
}
