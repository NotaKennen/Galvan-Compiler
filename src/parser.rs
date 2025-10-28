use crate::{compiler_settings::PAR_DEBUG_PRINTS, lexer::{LexSymbol, Lexeme}};
use std::iter::Peekable;

//
// NOTES
//

//
// STRUCTS
//

pub enum ParFunction {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    LiteralCalculation(LiteralCalculation),
    FunctionCall(FunctionCall),
}

pub struct VariableDeclaration {
    pub name: String,
    pub value: String
}

pub struct FunctionDeclaration {}

pub struct LiteralCalculation {}

pub struct FunctionCall {}

//
// FUNCTIONS
//

/// A parser help function to expect a certain value, panics on failure.
/// 
/// Takes in a &Lexeme, returns Lexeme.value
fn expect(expectation: LexSymbol, lexeme: &mut Peekable<std::slice::Iter<'_, Lexeme>>) -> String {
    if lexeme.peek().unwrap().symbol == expectation {
        let value = lexeme.peek().unwrap().value.clone();
        lexeme.next();
        return value
    } else { // TODO: don't panic
        panic!("Expected {:?}, not {:?}", expectation, lexeme.peek().unwrap().symbol);
    }
}

/// See `expect()`, same thing, but has multiple possible expectations with a `&[LexSymbol]`
fn multi_expect(expectations: &[LexSymbol], lexeme: &mut Peekable<std::slice::Iter<'_, Lexeme>>) -> String {
    if expectations.contains(&lexeme.peek().unwrap().symbol) {
        let value = lexeme.peek().unwrap().value.clone();
        lexeme.next();
        return value
    } else { // TODO: don't panic
        panic!("Expected one of following {:?}; not {:?}", expectations, lexeme.peek().unwrap().symbol);
    }
}

/// Parser entrypoint, turns a Vec<Lexeme> to Vec<ParFn>
pub fn parser(lexemevector: Vec<Lexeme>) -> Result<Vec<ParFunction>, String> {
    if PAR_DEBUG_PRINTS {println!("- - - PARSER")}

    let mut parfunctions: Vec<ParFunction> = Vec::new();
    let mut lexeme = lexemevector.iter().peekable();
    loop {
        if lexeme.peek() == None {break}
        match lexeme.peek().unwrap().symbol {

            // Correct start symbols
            LexSymbol::Keyword => {
                // Conditionals
                if lexeme.peek().unwrap().value == "if" {

                }

                // Function declaration
                if lexeme.peek().unwrap().value == "fn" {
                    lexeme.next();
                }

                // Variable declaration
                if lexeme.peek().unwrap().value == "let" {
                    // We're declaring a variable
                    lexeme.next();
                    let varname = expect(LexSymbol::Identifier, &mut lexeme);
                    expect(LexSymbol::EqualSign, &mut lexeme);
                    let value = multi_expect(
                    &[LexSymbol::Integer, LexSymbol::String, LexSymbol::Identifier], &mut lexeme);
                    // FIXME: dotting exists !!
                    parfunctions.push(ParFunction::VariableDeclaration(
                        VariableDeclaration {name: varname, value: value}
                    ));
                    expect(LexSymbol::EndLine, &mut lexeme);
                    lexeme.next();
                    // STOP
                }

                // Invalid (shouldn't happen unless Lexer messed up)
                else {return Err(format!("Unknown Keyword: {}", lexeme.peek().unwrap().value))}
            }
            LexSymbol::Identifier => {
                panic!("yuh uh (identifier)")
            }

            // Incorrect start symbols
            LexSymbol::EndLine => {continue} // technically correct but idc
            token => {return Err(format!("Expected statement, not {:?}", token))}
        }
    }

    if PAR_DEBUG_PRINTS {println!("- Parser done!")}
    return Ok(parfunctions)
}