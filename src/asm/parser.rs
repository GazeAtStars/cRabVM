use nom::{types::CompleteStr, digit, alpha1, alphanumeric, multispace};
use crate::asm::{Token, instruction_parser::AsmInstruction};
use crate::instructions::Opcode;

named!(pub opcode<CompleteStr, Token>,
  do_parse!(
      opcode: alpha1 >>
      (
        {
            Token::Opcode{code: Opcode::from(opcode)}
        }
      )
  )
);


named!(pub register <CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >> // Register's use $
            num: digit >>
            (Token::Register{num: num.parse::<u8>().unwrap()})
        )
    )
);

named!(integer_arg<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            sign: opt!(tag!("-")) >>
            reg_num: digit >>
            (
                {
                    let mut tmp = String::from("");
                    if sign.is_some() {
                        tmp.push_str("-");
                    }
                    tmp.push_str(&reg_num.to_string());
                    let converted = tmp.parse::<i32>().unwrap();
                    Token::Integer{num: converted}
                }
            )
        )
    )
);

named!(pub arg<CompleteStr, Token>,
    alt!(
        integer_arg |
        register
    )
);

named!(pub label<CompleteStr, Token>,
    ws!(
        do_parse!(
            name: alphanumeric >>
            tag!(":") >>
            opt!(multispace) >>
            (Token::Label{name: name.to_string()})
        )
    )
);

named!(pub label_usage<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("@") >>
            name: alphanumeric >>
            opt!(multispace) >>
            (Token::LabelUsage{name: name.to_string()})
        )
    )
);

named!(directive_dec<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!(".") >>
            name: alphanumeric >>
            opt!(multispace) >>
            (Token::Directive{name: name.to_string()})
        )
    )
);

named!(pub directive<CompleteStr, AsmInstruction>,
    ws!(
        do_parse!(
            tag!(".") >>
            name: directive_dec >>
            arg1: opt!(arg) >>
            arg2: opt!(arg) >>
            arg3: opt!(arg) >>
            (AsmInstruction{
                opcode: None,
                directive: Some(name),
                label: None,
                arg1: arg1,
                arg2: arg2,
                arg3: arg3
            })
        )
    )
);

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse_opcode() {
        let mut result = opcode(CompleteStr("set"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(token, Token::Opcode{code: Opcode::SET});
        result = opcode(CompleteStr("aet"));
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Opcode{code: Opcode::IGL});
    }

    #[test]
    fn test_parse_register() {
        let mut result = register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        result = register(CompleteStr("0"));
        assert_eq!(result.is_err(), true);
        result = register(CompleteStr("$A"));
        assert_eq!(result.is_err(), true);
        result = register(CompleteStr("$"));
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_parse_integer() {
        let mut result = integer_arg(CompleteStr("#0"));
        assert_eq!(result.is_ok(), true);
        result = integer_arg(CompleteStr("0"));
        assert_eq!(result.is_err(), true);
        result = integer_arg(CompleteStr("#A"));
        assert_eq!(result.is_err(), true);
        result = integer_arg(CompleteStr("#"));
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_parse_label() {
        let result = label(CompleteStr("test:"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Label { name: "test".to_string() });
        let result = label(CompleteStr("test"));
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_label_usage() {
        let result = label_usage(CompleteStr("@test"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::LabelUsage { name: "test".to_string() });
        let result = label_usage(CompleteStr("test"));
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parser_directive() {
        let result = directive_dec(CompleteStr(".data"));
        assert_eq!(result.is_ok(), true);
        let (_, directive) = result.unwrap();
        assert_eq!(directive, Token::Directive { name: "data".to_string() })
    }

}
