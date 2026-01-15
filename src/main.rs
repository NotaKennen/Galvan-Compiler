mod io; use crate::io::load_file;
mod compiler_settings; use compiler_settings::*;
mod lexer; use lexer::*;
mod parser; use crate::parser::*;

fn main() {
    // We assume the source file exists (fix later)
    // TODO: Get source file via args
    let srccontents = load_file(SRC_FILE).unwrap();

    let lexersymbols = lexer(&srccontents);

    let lexeme = lexersymbols.iter().peekable();
    parser(lexeme).expect("Failed parsing");
}
