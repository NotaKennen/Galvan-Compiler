use crate::compiler_settings::{CLOSED_BRACES, KEYWORDS, LEX_DEBUG_PRINTS, LINE_SPLITTER, MATH_SYMBOLS, OPEN_BRACES, WHITESPACE};
use std::iter::Peekable;

// TODO: add line and character locations in Lexer
// add them to LexSymbol to help debug code (for users)

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum LexSymbol {
    Keyword,
    Identifier,
    String,
    Integer,
    GenericOpeningBracket,
    GenericClosingBracket,
    FunctionOpeningBracket,
    FunctionClosingBracket,
    MathSymbol,
    EqualSign,
    EndLine,
    Dot,
    DoubleDot,
    Comma,
    EOF,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Lexeme {
    pub symbol: LexSymbol,
    pub value: String
}
impl Lexeme {
    pub fn new(symbol: LexSymbol, value: String) -> Self {Lexeme{symbol:symbol, value:value}}
}

/// Takes in a peekable chars iterator, returns with the next possible Lexeme.
/// Keep running it until iterator runs out to get out all Lexemes.
fn lex_token(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<Lexeme> {
    while let Some(&c) = chars.peek() {
        // Token is whitespace, ignore
        if WHITESPACE.contains(&c) { 
            chars.next(); // Go to next char
            continue;
        }

        // Identifier or keyword
        if c.is_ascii_alphabetic() || c == '_' {
            let mut ident = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    ident.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            if KEYWORDS.contains(&ident.as_str()) {
                return Some(Lexeme::new(LexSymbol::Keyword, ident));
            } else {
                return Some(Lexeme::new(LexSymbol::Identifier, ident));
            }
        }

        // Integer
        if c.is_ascii_digit() {
            let mut num = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_digit() {
                    num.push(ch);
                    chars.next();
                } else if ch == '.' {
                    num.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            return Some(Lexeme::new(LexSymbol::Integer, num));
        }

        // String literal
        if c == '"' {
            chars.next(); // consume opening quote
            let mut val = String::new();
            while let Some(ch) = chars.next() {
                if ch == '"' {
                    break;
                }
                val.push(ch);
            }
            return Some(Lexeme::new(LexSymbol::String, val));
        }
    
        // Braces
        if OPEN_BRACES.contains(&c) {
            if c == '(' {chars.next(); return Some(Lexeme::new(LexSymbol::GenericOpeningBracket, c.to_string()));}
            else if c == '{' {chars.next(); return Some(Lexeme::new(LexSymbol::FunctionOpeningBracket, c.to_string()));}
        }
        if CLOSED_BRACES.contains(&c) {
            if c == ')' {chars.next(); return Some(Lexeme::new(LexSymbol::GenericClosingBracket, c.to_string()));}
            else if c == '}' {chars.next(); return Some(Lexeme::new(LexSymbol::FunctionClosingBracket, c.to_string()));}
        }
    
        // Math symbols
        if MATH_SYMBOLS.contains(&c) {
            chars.next();
            return Some(Lexeme::new(LexSymbol::MathSymbol, c.to_string()));
        }

        // Line splitter
        if c == LINE_SPLITTER {
            chars.next();
            return Some(Lexeme::new(LexSymbol::EndLine, LINE_SPLITTER.to_string()))
        }

        // Dot
        if c == '.' {
            chars.next();
            return Some(Lexeme::new(LexSymbol::Dot, '.'.to_string()))
        }

        // Comma
        if c == ',' {
            chars.next();
            return Some(Lexeme::new(LexSymbol::Comma, ','.to_string()))
        }

        // Equal Sign
        if c == '=' {
            chars.next();
            return Some(Lexeme::new(LexSymbol::EqualSign, '='.to_string()))
        }

        // Double dot ( : )
        if c == ':' {
            chars.next();
            return Some(Lexeme::new(LexSymbol::DoubleDot, ":".to_string()))
        }

        // Unrecognized: skip
        chars.next();
    }

    None
}

/// Takes string, returns Vec<LexSm>
pub fn lexer(content: &str) -> Vec<Lexeme> {
    if LEX_DEBUG_PRINTS {println!("- - - LEXER")}

    // Main lexer loop
    let mut chars = content.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(token) = lex_token(&mut chars) {
        tokens.push(token);
    }

    if LEX_DEBUG_PRINTS {println!("LEXED TOKENS:\n{:#?}", tokens)}

    if LEX_DEBUG_PRINTS {println!("- - - Lexer done!")}
    return tokens;
}