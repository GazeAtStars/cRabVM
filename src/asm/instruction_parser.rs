use crate::asm::*;
use crate::asm::parser::*;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AsmInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
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
        if let Some(ref token) = self.opcode  {
            match token {
                Token::Opcode { code } => {
                    let b: u8 = (*code).into();
                    bytes.push(b);
                },
                _ => panic!("Invalid opcode"),
            }
        }
        for arg in [&self.arg1, &self.arg2, &self.arg3].into_iter().flatten() {
            AsmInstruction::extract_arg(arg, &mut bytes);
        }
        while bytes.len() < 4 {
            bytes.push(0);
        }
        bytes
    }
}

named!(instruction<CompleteStr, AsmInstruction>,
    do_parse!(
        opcode: opcode >>
        label: opt!(label) >>
        arg1: opt!(arg) >>
        arg2: opt!(arg) >>
        arg3: opt!(arg) >>
        (AsmInstruction{opcode: Some(opcode), label, directive: None, arg1, arg2, arg3})
    )
);

named!(pub  parse_instruction<CompleteStr, AsmInstruction>,
    do_parse!(
        instruction: alt!(instruction) >>
        (instruction)
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm::Opcode;

    #[test]
    fn test_parse_instruction1() {
        let input = CompleteStr("set $0 #10\n");
        let expected = AsmInstruction {
            opcode: Some(Token::Opcode{code: Opcode::SET}),
            label: None,
            directive: None,
            arg1: Some(Token::Register{num: 0}),
            arg2: Some(Token::Integer{num: 10}),
            arg3: None,
        };
        assert_eq!(parse_instruction(input), Ok((CompleteStr(""), expected)));
    }

    #[test]
    fn test_parse_instruction2() {
        let input = CompleteStr("hlt");
        let expected = AsmInstruction {
            opcode: Some(Token::Opcode{code: Opcode::HLT}),
            label: None,
            directive: None,
            arg1: None,
            arg2: None,
            arg3: None,
        };
        assert_eq!(parse_instruction(input), Ok((CompleteStr(""), expected)));
    }
}
