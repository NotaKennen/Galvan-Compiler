use crate::{compiler_settings::PAR_DEBUG_PRINTS, lexer::{LexSymbol, Lexeme}};

// TODO: Custom ParserError type
// Include position information, expected symbol and actual symbol

//
// STRUCTS
//

#[derive(Debug)]
#[derive(Clone)]
pub enum Expression {
    Number(i64),
    String(String),
    Variable(String),
    Operation(Operation),
    FunctionCall {target: String, args: Vec<Expression>},
    ReturnValue {value: Box<Expression>}
    // ^ // TODO: Make return into a Statement instead 
         // Will require extra work ughhh
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Operation {
    left: Box<Expression>,
    operator: Operator,
    right: Box<Expression>
}

#[derive(Debug)]
#[derive(Clone)]
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
/// Returns `Lexeme::EOF` Lexeme if it hits a None
fn peek_lexeme(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Lexeme {
    let lexeme: Option<&&Lexeme> = lexeme.peek(); 
    if lexeme.is_none() { // TODO: Remember to change EOF lexeme location at some point
        return Lexeme { symbol: LexSymbol::EOF, value: String::new(), location: (0, 0) }
    } else { // FIXME: Don't clone the lexeme on peeking
        return lexeme.unwrap().to_owned().clone() // Peak programming
    }
}

/// Expects the next lexeme to be a valid expression, gets it, then sorts it into a valid Expression.
/// Runs `lexeme.next()` once.
/// 
/// Expects format `[Expr]`
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

/// Parses an expression. Returns the expression or a parse error.
/// 
/// Expects `[expr] (OperationalSymbol) (expr)...`
/// 
/// Returns cursor at `expr + 1`
fn parse_expression(lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Expression, String> {
    let min_importance = 0;
    println!("Entered head_parse");
    let expr = better_parse(parse_single_expression(lexeme)?, min_importance, lexeme);
    return expr
}

/// Used for better_parse() only. Returns the precedence for symbol in a 
/// operator-precedence parse system. 
fn precedence(strin: &str) -> i64 {
    if [">", "<", "=", "==", "!=", ">=", "<="].contains(&strin.as_ref()) {0}
    else if ["+", "-"].contains(&strin.as_ref()) {1}
    else if ["/", "*"].contains(&strin.as_ref()) {2}
    else {-1}
}

/// "main" parser sub-function for `parse_expression()`, use it instead, do not use this.
fn better_parse(mut left: Expression, min_importance: i64, lexeme: &mut std::iter::Peekable<std::slice::Iter<'_, Lexeme>>) -> Result<Expression, String> {
    // Get operator and ensure it's good (Initial error checking)
    if peek_lexeme(lexeme).symbol != LexSymbol::OperationalSymbol {
        println!("Isn't operationalsymbol");
        return Ok(left)
    }
    
    // Fetch Lookahead ; Outer loop
    println!("Doing loop of better_parse");
    let mut lookahead = peek_lexeme(lexeme);
    while precedence(&lookahead.value) >= min_importance {

        // op -> lookahead ; advance token
        println!("Hit 1st internal loop");
        let op = lookahead.clone();
        lexeme.next();

        // Parse RHS ; Lookahead -> peek next
        let mut right = parse_single_expression(lexeme)?; // implicit lexeme.next()
        lookahead = peek_lexeme(lexeme);                           // ^ Because of that, points to OpSymbol
        println!("\nLeft: ########\n{:#?}", left);
        println!("Right: ########\n{:#?}", right);
        println!("Lookahead: ########\n{:#?}\n", lookahead);

        // Inner loop
        while precedence(&lookahead.value) > precedence(&op.value) {
            println!("Hit 2nd internal loop");
            println!("Lookahead value: {:#?}", lookahead);
            let add_importance = if precedence(&lookahead.value) > min_importance {1} else {0};
            right = better_parse(right.clone(), precedence(&op.value) + add_importance, lexeme)?;
            lookahead = peek_lexeme(lexeme)
        }

        // Left side to left+right
        let operator = {match op.value.as_ref() { 
            "+" => Operator::Addition, 
            "-" => Operator::Subtraction, 
            "/" => Operator::Division, 
            "*" => Operator::Multiplication, 
            ">" => Operator::GreaterThan, 
            "<" => Operator::LesserThan, 
            "<=" => Operator::EqualLesserThan, 
            ">=" => Operator::EqualGreaterThan, 
            "!=" => Operator::Inequal, 
            "==" => Operator::EqualTo, 
            _ => return Err("Expected OperationalSymbol".to_string()) // TODO: change error to something good
        }};
        left = Expression::Operation(Operation {
            left: Box::new(left), 
            operator: operator, 
            right: Box::new(right) 
        });
        println!("Left changed to {:#?}", left);
    }

    println!("Returning with {:#?}", left);
    return Ok(left)
}

/// Get all following arguments for a function. Runs `lexeme.next()` until closing bracket,
/// (cursor to closebracket). Can handle no arguments as well.
/// 
/// Expects format `(Expr) (Comma) (Expr) (Comma) (Expr) ... (ClosingBracket)`. 
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
            // TODO: Use match here instead

            // Defining a variable
            if peek_lexeme(lexeme).value == "let" {
                lexeme.next();
                let variablename = expect(LexSymbol::Identifier, lexeme)?;
                expect(LexSymbol::EqualSign, lexeme)?;
                let expression = {
                    parse_expression(lexeme)?
                };
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
                expect(LexSymbol::EndLine, lexeme)?;
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
                expect(LexSymbol::EndLine, lexeme)?;
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

            // While loops
            else if peek_lexeme(lexeme).value == "while" {
                // Surprisingly similar to IFs hmmm...
                lexeme.next();
                expect(LexSymbol::GenericOpeningBracket, lexeme)?;
                let condition = parse_expression(lexeme)?;
                expect(LexSymbol::GenericClosingBracket, lexeme)?;
                expect(LexSymbol::FunctionOpeningBracket, lexeme)?;
                let body = parse_until_symbol(LexSymbol::FunctionClosingBracket, lexeme)?;
                lexeme.next();

                outtoken = Some(Statement::While {
                    condition, 
                    body 
                })
            }

            else {return Err(format!("Unexpected keyword '{}', non-matching Lexer-Parser versions?", peek_lexeme(lexeme).value))}
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
        let symbol = peek_lexeme(lexeme).symbol;
        let position = peek_lexeme(lexeme).location;
        return Err(format!("Expected {:?}, not {:?} at position {}:{}", expectation, symbol, position.0, position.1))
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
    return Ok(outtokens)
}