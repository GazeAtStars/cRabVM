mod parser;
mod instruction_parser;
pub mod program_parser;
use crate::instructions::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Opcode{code: Opcode},
    Register{num: u8},
    Integer{num: i32},
    Label{name: String},
    LabelUsage{name: String},
    Directive{name: String},
}
