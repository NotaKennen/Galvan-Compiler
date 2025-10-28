use crate::{compiler_settings::PAR_DEBUG_PRINTS, lexer::{LexSymbol, Lexeme}};

//
// NOTES
//

//
// STRUCTS
//

pub enum ParFunction {
    VariableDeclaration,
    FunctionDeclaration,
    LiteralCalculation,
    FunctionCall,
}

//
// FUNCTIONS
//

/// Parser entrypoint, turns a Vec<Lexeme> to Vec<ParFn>
pub fn parser(lexemevector: Vec<Lexeme>) {
    if PAR_DEBUG_PRINTS {println!("- - - PARSER")}

    let mut lexeme = lexemevector.iter().peekable();
    loop {
        if lexeme.peek() == None {break}
        match lexeme.peek().unwrap().symbol {
            // Correct start symbols
            LexSymbol::Keyword => {lexeme.next();}
            LexSymbol::Identifier => {lexeme.next();}
            LexSymbol::String => {lexeme.next();}

            // Incorrect start symbols
            LexSymbol::EndLine => {continue} // technically correct but idc
            LexSymbol::MathSymbol => {panic!("Expected statement, not MathSymbol")}
            LexSymbol::ClosingBracket => {panic!("Expected statement, not ClosingBracket")}
            LexSymbol::OpeningBracket => {panic!("Expected statement, not OpeningBracket")}
            LexSymbol::Dot => {panic!("Expected statement, not Dot")}
            LexSymbol::Integer => {panic!("Expected statement, not Integer")}
        }
    }

    if PAR_DEBUG_PRINTS {println!("- Parser done!")}
}