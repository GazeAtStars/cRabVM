#[macro_use]
extern crate nom;

pub mod vm;
pub mod instructions;
pub mod repl;
pub mod asm;

fn main() {
    let vm = vm::VM::new();
    let mut repl = repl::REPL::new(vm);
    repl.run();
}
