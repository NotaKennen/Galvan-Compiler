mod compiler_settings; use std::fs::read_to_string;
use compiler_settings::*;
mod lexer; use lexer::*;
mod parser; use crate::parser::*;
mod seman; use crate::seman::*;

fn main() {
    // TODO: Get source file via args
    // FIXME: make sure file exists
    let sourcefile = read_to_string(SRC_FILE).unwrap();

    let lexersymbols = lexer(&sourcefile);

    let lexeme = lexersymbols.iter().peekable();
    let statements = parser(lexeme).expect("Failed parsing");

    let _ = analyze(statements);
}
