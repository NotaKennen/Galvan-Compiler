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
    FunctionCall {target: String, args: Vec<Expression>},
    ReturnValue {value: Box<Expression>} // FIXME: probably not correct type
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
    GreaterThan,
    EqualLesserThan,
    EqualGreaterThan,
    EqualTo,
    Inequal,
}

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expression),
    VariableAssignment {name: String, value: Expression},
    FunctionAssignment {name: String, arguments: Vec<Expression>, body: Vec<Statement>},
    While {condition: Expression, body: Vec<Statement>},
    ConditionalStatement {condition: Expression, body: Vec<Statement>},
}

//
// FUNCTIONS
//

/// Peeks the lexeme, and handles unwrap.
/// 
/// Returns EOF Lexeme if it hits a None
fn peek_lexeme(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Lexeme {
    let lexeme: Option<&&Lexeme> = lexeme.peek(); 
    if lexeme.is_none() {
        return Lexeme { symbol: LexSymbol::EOF, value: String::new() }
    } else {
        return lexeme.unwrap().to_owned().clone() // Peak programming
    }
}

/// Expects the next lexeme to be a valid expression, gets it, then sorts it into a valid Expression.
/// Runs `lexeme.next()` once.
/// 
/// Expects format `(Expr)`
fn parse_single_expression(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Expression, String> {
    match peek_lexeme(lexeme).symbol {
        LexSymbol::String => {
            let content = peek_lexeme(lexeme).value.clone();
            lexeme.next();
            Ok(Expression::String(content))
        }
        LexSymbol::Integer => {
            let strint = &peek_lexeme(lexeme).value;
            let int = strint.parse::<i64>();
            if int.is_err() {return Err("Invalid integer".to_string())}
            lexeme.next();
            Ok(Expression::Number(int.unwrap()))
        }
        LexSymbol::Identifier => {
            // Check if it's a function or a variable (check for braces)
            let idname = peek_lexeme(lexeme).value.clone();
            lexeme.next();
            if peek_lexeme(lexeme).symbol == LexSymbol::GenericOpeningBracket {
                // We're inside the first bracket, cursor pointing at it.
                lexeme.next();
                let args = parse_arguments(lexeme)?;
                lexeme.next(); // Get over last endbracket
                Ok(Expression::FunctionCall { target: idname, args: args })
            }
            else {Ok(Expression::Variable(idname))}
        }
        symbol => {return Err(format!("Expected expression, not {:?}", symbol))}
    }
}

/// Takes in lexeme, recursively gets all the next expressions all the way until an invalid symbol. 
/// Stops on anything that's not a mathsymbol.
/// 
/// Expects format `(Expr) [MathSymbol] [Expr] ...`
fn parse_expression(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Expression, String> {
    let res_leftexpr = parse_single_expression(lexeme);
    if res_leftexpr.is_err() {return Err(res_leftexpr.unwrap_err())}
    let leftexpr = res_leftexpr?;

    let nextsymbol = peek_lexeme(lexeme).symbol;
    if nextsymbol == LexSymbol::MathSymbol {
        let operator = {
            // TODO: Use to Match here instead
            // All peek_lexeme()'s in this function
            if peek_lexeme(lexeme).value == "+" {Operator::Addition}
            else if peek_lexeme(lexeme).value == "-" {Operator::Subtraction}
            else if peek_lexeme(lexeme).value == "*" {Operator::Multiplication}
            else if peek_lexeme(lexeme).value == "/" {Operator::Division}
            else if peek_lexeme(lexeme).value == ">" {Operator::GreaterThan}
            else if peek_lexeme(lexeme).value == "<" {Operator::LesserThan}
            else {return Err(format!("{:?} is not considered a valid operator", peek_lexeme(lexeme).value))}
        };
        lexeme.next();
        return Ok(Expression::Operation(
            Operation { 
                left: Box::new(leftexpr),
                operator: operator, 
                right: Box::new(parse_expression(lexeme)?),
            }
        ))
    } else if nextsymbol == LexSymbol::OperationalSymbol {
        let operator: Operator = {
            if peek_lexeme(lexeme).value == "==" {Operator::EqualTo}
            else if peek_lexeme(lexeme).value == ">=" {Operator::EqualGreaterThan}
            else if peek_lexeme(lexeme).value == "<=" {Operator::EqualLesserThan}
            else if peek_lexeme(lexeme).value == "!=" {Operator::Inequal}
            else {return Err(format!("{:?} is not considered a valid operator", peek_lexeme(lexeme).value))}
        };
        lexeme.next();
        return Ok(Expression::Operation(
            Operation {
                left: Box::new(leftexpr),
                operator: operator,
                right: Box::new(parse_expression(lexeme)?)
            }
        ))
    } else {return Ok(leftexpr)}
}

/// Get all following arguments for a function. Runs `lexeme.next` until closing bracket,
/// (cursor to closebracket)
/// 
/// Expects format `[Expr] [Comma] [Expr] [Comma] [Expr] ... (ClosingBracket)`. 
fn parse_arguments(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Vec<Expression>, String> {
    if peek_lexeme(lexeme).symbol == LexSymbol::GenericClosingBracket {return Ok(vec![])}

    // Get the first argument
    let mut args: Vec<Expression> = vec![];
    let expr = parse_expression(lexeme);
    if expr.is_err() {return Err(expr.unwrap_err())}
    args.push(expr?);
    // TODO: add types to argument parsing
    // (and to the rest of the lang as well ig)

    // Use recursion to get the rest of the arguments
    if peek_lexeme(lexeme).symbol == LexSymbol::Comma {
        lexeme.next();
        let res_args = parse_arguments(lexeme);
        if res_args.is_err() {return Err(res_args.unwrap_err())}
        args.append(&mut res_args.unwrap());
    }

    Ok(args)
}

/// Parses a singular "line", basically anything until `LexSymbol::EndLine`.
/// Unlike `parse_single_expression()`, this one includes keywords and such.
fn parse_single(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Option<Statement>, String> { 
    let mut outtoken: Option<Statement> = None;
    match peek_lexeme(lexeme).symbol {
        // Keywords, see compiler_settings.rs for specifics
        LexSymbol::Keyword => {
            // TODO: Automatically fetch keywords from settings or something
            // TODO: Use match here instead

            // Defining a variable
            if peek_lexeme(lexeme).value == "let" {
                lexeme.next();
                let variablename = expect(LexSymbol::Identifier, lexeme)?;
                expect(LexSymbol::EqualSign, lexeme)?;
                let expression = {
                    parse_expression(lexeme)
                }?;
                outtoken = Some(Statement::VariableAssignment { 
                    name: variablename,
                    value: expression 
                });
                expect(LexSymbol::EndLine, lexeme)?;
                // STOP
            }
        
            // Defining function
            else if peek_lexeme(lexeme).value == "function" {
                lexeme.next();
                let functionname = expect(LexSymbol::Identifier, lexeme)?;
                expect(LexSymbol::GenericOpeningBracket, lexeme)?;
                let arguments = parse_arguments(lexeme)?;
                // FIXME: You can pass function calls etc as function arguments
                lexeme.next(); // Jump over ending bracket
                expect(LexSymbol::FunctionOpeningBracket, lexeme)?;
                let internals = parse_until_symbol(LexSymbol::FunctionClosingBracket, lexeme)?;
                lexeme.next();

                outtoken = Some(Statement::FunctionAssignment {
                    name: functionname, 
                    arguments: arguments, 
                    body: internals, 
                });
            }

            // Calling function
            else if peek_lexeme(lexeme).value == "call" {
                lexeme.next();
                let target = expect(LexSymbol::Identifier, lexeme)?;
                expect(LexSymbol::GenericOpeningBracket, lexeme)?;
                let arguments = parse_arguments(lexeme)?;
                expect(LexSymbol::GenericClosingBracket, lexeme)?;
                outtoken = Some(Statement::ExpressionStatement(
                        Expression::FunctionCall { 
                        target: target,
                        args: arguments,
                    }
                ));
            }

            // Function returns
            else if peek_lexeme(lexeme).value == "return" {
                lexeme.next();
                let returning = parse_expression(lexeme)?;
                outtoken = Some(Statement::ExpressionStatement(
                    Expression::ReturnValue { 
                        value: Box::new(returning)
                    }
                ))
            }

            // Conditionals
            else if peek_lexeme(lexeme).value == "if" {
                lexeme.next();
                expect(LexSymbol::GenericOpeningBracket, lexeme)?;
                let condition = parse_expression(lexeme)?;
                expect(LexSymbol::GenericClosingBracket, lexeme)?;
                expect(LexSymbol::FunctionOpeningBracket, lexeme)?;
                let body = parse_until_symbol(LexSymbol::FunctionClosingBracket, lexeme)?;
                lexeme.next();

                outtoken = Some(Statement::ConditionalStatement {
                    condition, 
                    body, 
                })
            }
        }

        // "Breaking symbols"
        LexSymbol::EndLine => {lexeme.next();}
        invalid => {return Err(format!("Expected keyword, not {:?}", invalid))}
    }
    return Ok(outtoken);
}

/// Keeps parsing the statements until it hits a specified symbol.
/// This is basically just `parser()`, just that the EOF check is changed to `stopsymbol`
/// 
/// Expects format `[Expr] (anything) [EndLine]...`
fn parse_until_symbol(stopsymbol: LexSymbol, lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Vec<Statement>, String> {
    let mut outtokens: Vec<Statement> = vec![];  
    loop {
        if peek_lexeme(lexeme).symbol == stopsymbol || peek_lexeme(lexeme).symbol == LexSymbol::EOF
            {break}
        let statement = parse_single(lexeme)?;
        if statement.is_none() {continue}
        else {outtokens.push(statement.unwrap());}
    }
    return Ok(outtokens);
}

/// Expects a certain type of `LexSymbol`. 
/// 
/// Returns `Err(String)` if not expected, `Ok(lexeme.value)` if is. 
/// Returned string is "Expected \[Expectation\], not \[Found symbol\]" 
/// (user-facing text)
/// 
/// Moves to the next Lexeme when done
fn expect(expectation: LexSymbol, lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<String, String> {
    if peek_lexeme(lexeme).symbol == expectation {
        let returnable = Ok(peek_lexeme(lexeme).value.clone());
        lexeme.next();
        return returnable;
    } else {
        return Err(format!("Expected {:?}, not {:?}", expectation, peek_lexeme(lexeme).symbol))
    }
}

/// Parser entrypoint, turns a `Vec<Lexeme>` to `Vec<Statement>`
pub fn parser(mut lexeme: std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Vec<Statement>, String> {
    if PAR_DEBUG_PRINTS {println!("- - - PARSER")}

    let mut outtokens: Vec<Statement> = vec![];  
    loop {
        if peek_lexeme(&mut lexeme).symbol == LexSymbol::EOF {break}
        let statement = parse_single(&mut lexeme)?;
        if statement.is_none() {continue}
        else {outtokens.push(statement.unwrap());}
    }

    if PAR_DEBUG_PRINTS {println!("\nStatement dump:\n{:#?}\n", outtokens)}
    if PAR_DEBUG_PRINTS {println!("- - - Parser done!")}
    return Ok(vec![])
}