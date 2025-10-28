use crate::compiler_settings::{CLOSED_BRACES, KEYWORDS, LEX_DEBUG_PRINTS, LINE_SPLITTER, MATH_SYMBOLS, OPEN_BRACES};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum LexSymbol {
    Keyword,
    Identifier,
    String,
    Integer,
    OpeningBracket,
    ClosingBracket,
    MathSymbol,
    EndLine,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Lexeme {
    pub symbol: LexSymbol,
    pub value: String
}

// Lexes one whitespace split "token"
fn lex_token(token: &str) -> Vec<Lexeme> {

    // Check for small writing quirks
    if token.ends_with(LINE_SPLITTER) { // Line splitter
        let token = token.strip_suffix(";").unwrap();
        let mut lexeme = lex_token(token);
        let endline = Lexeme {
            symbol: LexSymbol::EndLine,
            value: LINE_SPLITTER.to_string()
        }; // FIXME: this is pretty awful
        println!("- {}\n^ (Line Splitter)\n", LINE_SPLITTER);
        lexeme.push(endline);
        return lexeme
    }

    // Form it into a token
    if LEX_DEBUG_PRINTS {println!("- {}", token)} // DEBUG
    let result_symbol = match token {
        token if KEYWORDS.contains(&token) => {LexSymbol::Keyword},
        token if MATH_SYMBOLS.contains(&token) => {LexSymbol::MathSymbol},
        token if OPEN_BRACES.contains(&token) => {LexSymbol::OpeningBracket},
        token if CLOSED_BRACES.contains(&token) => {LexSymbol::ClosingBracket},
        token if token.parse::<f64>().is_ok() => {LexSymbol::Integer},
        token if token.starts_with("\"") && token.ends_with("\"")
            => {LexSymbol::String},
        _ => {LexSymbol::Identifier}
    };

    // TODO: Check for braces and dotting (Maybe here??)
    // For example, something.value would be considered one identifier.
    // Similarly somefunc("invalue") would also be one (according to current system)
    // Don't check at writing quirks, it could influence floats (11.6 isn't a dotted value)
    // Thus check if it's an identifier and *then* run it (?)

    // Form into a struct Lexeme and return
    if LEX_DEBUG_PRINTS {println!("^ ({:?})\n", result_symbol)} // DEBUG
    let lexeme = Lexeme {
        symbol: result_symbol,
        value: token.to_string()
    };
    return vec![lexeme]
}

// Takes string, returns Vec<LexSm>
pub fn lexer(content: &str) -> Vec<Lexeme> {
    let mut tokens = Vec::new();

    if LEX_DEBUG_PRINTS {println!("- - - LEXER")}
    if LEX_DEBUG_PRINTS {println!("[DEBUG] Dumping tokens:")}
    for token in content.split_whitespace() {
        let mut lexeme = lex_token(token);
        tokens.append(&mut lexeme)
    }
    println!("- Lexer done!");

    return tokens
}