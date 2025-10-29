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

pub struct FunctionDeclaration {
    name: String,
    function_contents: Vec<ParFunction>
}

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

/// See `expect()`. Same thing, but has multiple possible expectations with a `&[LexSymbol]`
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
                    panic!("Conditionals not implemented!")
                }

                // Function declaration
                else if lexeme.peek().unwrap().value == "fn" {
                    lexeme.next();
                    let functionname = 
                        expect(LexSymbol::Identifier, &mut lexeme);
                    expect(LexSymbol::OpeningBracket, &mut lexeme);
                    // TODO: take in arguments in functions
                    expect(LexSymbol::ClosingBracket, &mut lexeme);
                    println!("FUN DEC | {}", functionname);
                    parfunctions.push(ParFunction::FunctionDeclaration(
                        FunctionDeclaration {
                            name: functionname,
                            function_contents: vec![]
                        }   
                    // TODO: function contents
                    // We need to insert the function contents (likely as a Vec<ParFn>),
                    // into the FunctionDeclaration so that we know what's inside the function
                    // We'll likely do this by turning on some in_function bool variable and
                    // then just waiting for closing bracket. Then dump everything to contents.
                    // This does however mean that no functions inside functions. (idc about that tbh)
                    ));
                    lexeme.next();
                    continue
                    // STOP
                }

                // Variable declaration
                else if lexeme.peek().unwrap().value == "let" {
                    lexeme.next();
                    let varname = 
                        expect(LexSymbol::Identifier, &mut lexeme);
                    expect(LexSymbol::EqualSign, &mut lexeme);
                    let value = 
                        multi_expect(&[LexSymbol::Integer, LexSymbol::String, LexSymbol::Identifier], &mut lexeme);
                    // FIXME: dotting exists !!
                    // FIXME: take in functions
                    println!("VAR DEC | {}: {}", varname, value); // DEBUG
                    parfunctions.push(ParFunction::VariableDeclaration(
                        VariableDeclaration {name: varname, value: value}
                    ));
                    expect(LexSymbol::EndLine, &mut lexeme);
                    continue
                    // STOP
                }

                // Invalid (shouldn't happen unless Lexer messed up or we did lexeme.next() incorrectly)
                else {return Err(format!("Unknown Keyword: {}", lexeme.peek().unwrap().value))}
            }
            LexSymbol::Identifier => {
                lexeme.next();

                // We're running a function
                if lexeme.peek().unwrap().symbol == LexSymbol::OpeningBracket {
                    lexeme.next();
                    // TODO: take in arguments
                    expect(LexSymbol::ClosingBracket, &mut lexeme);
                    expect(LexSymbol::EndLine, &mut lexeme);
                    continue;
                    // STOP
                }

                // We're dotting something
                else if lexeme.peek().unwrap().symbol == LexSymbol::Dot {
                    panic!("Dotting Identifiers not implemented!");
                }

                // Incorrect Identifier
                else {
                    return Err(format!("Expected Dot or Function, not {:?}", lexeme.peek().unwrap().symbol))
                }
            }

            // Incorrect start symbols
            LexSymbol::EndLine => {continue} // technically correct but idc
            token => {return Err(format!("Expected statement, not {:?}", token))}
        }
    }

    if PAR_DEBUG_PRINTS {println!("- Parser done!")}
    return Ok(parfunctions)
}