use nom::{types::CompleteStr, digit, alpha1};
use crate::asm::Token;
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

named!(pub integer <CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >> // Integer's use !
            num: digit >>
            (Token::Integer{num: num.parse::<i32>().unwrap()})
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
        let mut result = integer(CompleteStr("#0"));
        assert_eq!(result.is_ok(), true);
        result = integer(CompleteStr("0"));
        assert_eq!(result.is_err(), true);
        result = integer(CompleteStr("#A"));
        assert_eq!(result.is_err(), true);
        result = integer(CompleteStr("#"));
        assert_eq!(result.is_err(), true);
    }

}
