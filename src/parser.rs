use std::fmt::format;

use crate::{compiler_settings::PAR_DEBUG_PRINTS, lexer::{LexSymbol, Lexeme}};

//
// STRUCTS
//

#[derive(Debug)]
enum Expression {
    Number(i64),
    String(String),
    Variable(String),
    Operation(Operation),
    FunctionCall {target: String, args: Vec<Expression>}
}

#[derive(Debug)]
enum Operation {
    Left(Box<Expression>),
    Operator(Operator),
    Right(Box<Expression>)
}

#[derive(Debug)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LesserThan,
    GreaterThan
}

#[derive(Debug)]
enum Statement {
    ExpressionStatement(Expression),
    VariableAssignment {name: String, value: Expression},
    FunctionAssignment {name: String, arguments: Vec<String>, body: Vec<Statement>},
    While {condition: Expression, body: Vec<Statement>}

}

//
// FUNCTIONS
//

// Recursive parse functions
fn parse_expression(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Expression, String> {
    Err("Not implemented".to_string())
}
fn parse_statement(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Statement, String> {
    Err("Not implemented".to_string())
}
fn parse_operator(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Operator, String> {
    Err("Not implemented".to_string())
}
fn parse_term(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Expression, String> {
    match lexeme.peek().unwrap().symbol {
        LexSymbol::Integer => {
            let result = expect(LexSymbol::Integer, lexeme);
            if result.is_err() {return Err("Found invalid Integer".to_string())}
            let int_value: i64 = result?.parse().unwrap();
            return Ok(Expression::Number(int_value));
        },
        LexSymbol::Identifier => {
            // TODO: identifiers are ass dude
            return Err("Not implemented".to_string())
        },
        LexSymbol::String => {
            let result = expect(LexSymbol::String, lexeme);
            if result.is_err() {return Err("Found invalid String".to_string())}
            return Ok(Expression::String(result?));
        },
        _ => {
            return Err(format!(
                "Expected term, not {:?}", 
                lexeme.peek().unwrap().symbol
            ))
        }
    }
}

/// Expects a certain type of `LexSymbol`. 
/// 
/// Returns ´Err(String)´ if not expected, Ok(lexeme.value) if is
fn expect(expectation: LexSymbol, lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<String, String> {
    if lexeme.peek().unwrap().symbol == expectation {
        let returnable = Ok(lexeme.peek().unwrap().value.clone());
        lexeme.next();
        return returnable;
    } else {
        return Err(format!("Expected {:?}, not {:?}", expectation, lexeme.peek().unwrap().symbol))
    }
}

/// Parser entrypoint, turns a Vec<Lexeme> to Vec<ParFn>
pub fn parser(lexemevector: Vec<Lexeme>) -> Result<Vec<String>, String> {
    if PAR_DEBUG_PRINTS {println!("- - - PARSER")}

    let mut outtokens: Vec<Statement> = vec![];
    let mut lexeme = lexemevector.iter().peekable();
    loop {
        if lexeme.peek() == None {break}
        match lexeme.peek().unwrap().symbol {

            LexSymbol::Keyword => {
                // Defining a variable
                if lexeme.peek().unwrap().value == "let" {
                    lexeme.next();
                    let varname = 
                        expect(LexSymbol::Identifier, &mut lexeme)?;
                    expect(LexSymbol::EqualSign, &mut lexeme)?;
                    
                    // We need to figure out what is it
                    // It could be a number, calculation, function or variable
                    // We do it in this inline function that does stuff
                    let unsure_expression: Result<Expression, String> = {
                    if lexeme.peek().unwrap().symbol == LexSymbol::Identifier {
                        // Could be a function or a variable
                        Err("Identifiers not implemented".to_string())
                    }
                    else if lexeme.peek().unwrap().symbol == LexSymbol::Integer {
                        // It's a number, it could be a calculation though.
                        let expression = ();
                        // TODO: recursion
                        // Remember that thing about functions and recursion?
                        // It could probably be used here. Make this (selection)
                        // into a function, then recurse through that.
                        // No idea how it'd work in practice though.
                        Err("Integers not implemented".to_string())
                    }
                    else if lexeme.peek().unwrap().symbol == LexSymbol::String {
                        // It's a string, easy as.
                        let result = expect(LexSymbol::String, &mut lexeme);
                        if result.is_err() {return Err("Found invalid String".to_string())}
                        Ok(Expression::String(result?))
                    }
                    else {Err(format!(
                        "Expected String, Integer or Identifier, not {:?}", 
                        lexeme.peek().unwrap().symbol
                    ))}
                    };

                    if unsure_expression.is_err() {return Err(unsure_expression.unwrap_err())}
                    let expression = unsure_expression?;
                    outtokens.push(Statement::VariableAssignment {
                        name: varname,
                        value: expression
                    })
                }
            }

            LexSymbol::EndLine => {continue}
            invalid => {return Err(format!("Expected _, not {:?}", invalid))}
        }
    }

    if PAR_DEBUG_PRINTS {println!("- Parser done!")}
    return Ok(vec![])
}