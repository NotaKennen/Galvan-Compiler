use crate::{compiler_settings::PAR_DEBUG_PRINTS, lexer::{LexSymbol, Lexeme}};

//
// STRUCTS
//

#[derive(Debug)]
pub enum Expression {
    Number(i64),
    String(String),
    Variable(String),
    Operation(Operation),
    FunctionCall {target: String, args: Vec<Expression>}
}

#[derive(Debug)]
pub struct Operation {
    left: Box<Expression>,
    operator: Operator,
    right: Box<Expression>
}

#[derive(Debug)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LesserThan,
    GreaterThan
}

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expression),
    VariableAssignment {name: String, value: Expression},
    FunctionAssignment {name: String, arguments: Vec<String>, body: Vec<Statement>},
    While {condition: Expression, body: Vec<Statement>}

}

//
// FUNCTIONS
//

// Recursive parse functions
/// Takes in lexeme, recursively gets all the next expressions all the way until a `LINE_SPLITTER`,
/// or until error 
fn parse_expression(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Expression, String> {
    let leftexpr = match lexeme.peek().unwrap().symbol {
        LexSymbol::String => {
            let content = lexeme.peek().unwrap().value.clone();
            lexeme.next();
            Expression::String(content)
        }
        LexSymbol::Integer => {
            let strint = &lexeme.peek().unwrap().value;
            let int = strint.parse::<i64>();
            if int.is_err() {return Err("Invalid integer".to_string())}
            lexeme.next();
            Expression::Number(int.unwrap())

        }
        LexSymbol::Identifier => {
            // Check if it's a function or a variable (check for braces)
            let idname = lexeme.peek().unwrap().value.clone();
            lexeme.next();
            if lexeme.peek().unwrap().symbol == LexSymbol::GenericOpeningBracket {
                // TODO: get args for functions in id parsing
                lexeme.next();
                lexeme.next();
                Expression::FunctionCall { target: idname, args: vec![] }
            }
            else {Expression::Variable(idname)}
        }
        symbol => {return Err(format!("Expected expression, not {:?}", symbol))}
    };

    // Note: we run lexeme.next() in the previous set
    let nextsymbol = lexeme.peek().unwrap().symbol;
    if nextsymbol == LexSymbol::EndLine {return Ok(leftexpr)}
    else if nextsymbol == LexSymbol::MathSymbol {
        let operator = {
            if lexeme.peek().unwrap().value == "+" {Operator::Addition}
            else if lexeme.peek().unwrap().value == "-" {Operator::Subtraction}
            else if lexeme.peek().unwrap().value == "*" {Operator::Multiplication}
            else if lexeme.peek().unwrap().value == "/" {Operator::Division}
            else if lexeme.peek().unwrap().value == ">" {Operator::GreaterThan}
            else if lexeme.peek().unwrap().value == "<" {Operator::LesserThan}
            else {return Err(format!("{:?} is not considered a valid operator", lexeme.peek().unwrap().value))}
        };
        lexeme.next();
        return Ok(Expression::Operation(
            Operation { 
                left: Box::new(leftexpr),
                operator: operator, 
                right: Box::new(parse_expression(lexeme)?),
            }
        ))
    }
    else {return Err(format!("Expected Endline or MathSymbol, not {:?}", nextsymbol))}
}
fn parse_statement(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Statement, String> {
    Err("Not implemented".to_string())
}

/// Expects a certain type of `LexSymbol`. 
/// 
/// Returns `Err(String)` if not expected, `Ok(lexeme.value)` if is
fn expect(expectation: LexSymbol, lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<String, String> {
    if lexeme.peek().unwrap().symbol == expectation {
        let returnable = Ok(lexeme.peek().unwrap().value.clone());
        lexeme.next();
        return returnable;
    } else {
        return Err(format!("Expected {:?}, not {:?}", expectation, lexeme.peek().unwrap().symbol))
    }
}

/// Parser entrypoint, turns a `Vec<Lexeme>` to `Vec<Statement>`
pub fn parser(lexemevector: Vec<Lexeme>) -> Result<Vec<Statement>, String> {
    if PAR_DEBUG_PRINTS {println!("- - - PARSER")}

    let mut outtokens: Vec<Statement> = vec![];
    let mut lexeme = lexemevector.iter().peekable();
    loop {
        if lexeme.peek() == None {break}
        match lexeme.peek().unwrap().symbol {

            // Keywords, see compiler_settings.rs for specifics
            LexSymbol::Keyword => {
                // Defining a variable
                if lexeme.peek().unwrap().value == "let" {
                    // TODO: Error handling
                    lexeme.next();
                    let variablename = expect(LexSymbol::Identifier, &mut lexeme)?;
                    expect(LexSymbol::EqualSign, &mut lexeme)?;
                    let expression = {
                        parse_expression(&mut lexeme)
                    }?;
                    if PAR_DEBUG_PRINTS {println!("Parser got expression: {:?}", expression)};
                    outtokens.push(Statement::VariableAssignment { 
                        name: variablename,
                        value: expression 
                    });
                    lexeme.next();
                    continue;
                    // STOP
                }
            }

            // "Breaking symbols"
            LexSymbol::EndLine => {lexeme.next(); continue} //  V //TODO: fill that up
            invalid => {return Err(format!("Expected _, not {:?}", invalid))}
        }
    }

    if PAR_DEBUG_PRINTS {println!("\nStatement dump:\n{:#?}\n\n", outtokens)}
    if PAR_DEBUG_PRINTS {println!("- - - Parser done!")}
    return Ok(vec![])
}