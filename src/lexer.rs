use crate::compiler_settings::{CLOSED_BRACES, KEYWORDS, LEX_DEBUG_PRINTS, LINE_SPLITTER, OPEN_BRACES, WHITESPACE};
use std::iter::Peekable;


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum LexSymbol {
    Keyword,
    Identifier,
    String,
    Integer,
    GenericOpeningBracket, // TODO: Compress brackets to one (or two) symbols
    GenericClosingBracket, // ^ Open/close-bracket with value "(" or ")" etc...
    FunctionOpeningBracket,
    FunctionClosingBracket,
    OperationalSymbol,
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
    pub value: String,
    pub location: (usize, usize), // LINE : CHARACTER
}
impl Lexeme {
    pub fn new(symbol: LexSymbol, value: String, location: (usize, usize)) -> Self {Lexeme{symbol:symbol, value:value, location: location}}
}
// TODO: add line and character locations in Lexer
// add them to Lexeme to help debug code (for users)

/// Takes in a peekable chars iterator, returns with the next possible Lexeme.
/// Keep running it until iterator runs out to get out all Lexemes.
fn lex_token(chars: &mut Peekable<impl Iterator<Item = char>>, loc: &mut (usize, usize)) -> Option<Lexeme> {
    while let Some(&c) = chars.peek() {
        // Token is whitespace, ignore
        if WHITESPACE.contains(&c) { 
            if &c == &'\n' {*loc = (loc.0 + 1, 1 as usize)} 
            else {*loc = (loc.0, loc.1 + 1)}
            chars.next(); // Go to next char
            continue;
        }

        // Identifier or keyword
        if c.is_ascii_alphabetic() || c == '_' {
            let mut ident = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    ident.push(ch);
                    *loc = (loc.0, loc.1 + 1);
                    chars.next();
                } else {
                    break;
                }
            }
            if KEYWORDS.contains(&ident.as_str()) {
                return Some(Lexeme::new(LexSymbol::Keyword, ident, *loc));
            } else {
                return Some(Lexeme::new(LexSymbol::Identifier, ident, *loc));
            }
        }

        // Integer
        if c.is_ascii_digit() {
            let mut num = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_digit() {
                    num.push(ch);
                    *loc = (loc.0, loc.1 + 1);
                    chars.next();
                } else if ch == '.' {
                    num.push(ch);
                    *loc = (loc.0, loc.1 + 1);
                    chars.next();
                } else {
                    break;
                }
            }
            return Some(Lexeme::new(LexSymbol::Integer, num, *loc));
        }

        // String literal
        if c == '"' {
            *loc = (loc.0, loc.1 + 1);
            chars.next(); // consume opening quote
            let mut val = String::new();
            while let Some(ch) = chars.next() {
                *loc = (loc.0, loc.1 + 1);
                if ch == '"' {
                    break;
                }
                val.push(ch);
            }
            return Some(Lexeme::new(LexSymbol::String, val, *loc));
        }
    
        // Braces
        if OPEN_BRACES.contains(&c) { // TODO: This brace setup is stupid, make it better
            if c == '(' {chars.next(); *loc = (loc.0, loc.1 + 1); return Some(Lexeme::new(LexSymbol::GenericOpeningBracket, c.to_string(), *loc));}
            else if c == '{' {chars.next(); *loc = (loc.0, loc.1 + 1); return Some(Lexeme::new(LexSymbol::FunctionOpeningBracket, c.to_string(), *loc));}
        }
        if CLOSED_BRACES.contains(&c) {
            if c == ')' {chars.next(); *loc = (loc.0, loc.1 + 1); return Some(Lexeme::new(LexSymbol::GenericClosingBracket, c.to_string(), *loc));}
            else if c == '}' {chars.next(); *loc = (loc.0, loc.1 + 1); return Some(Lexeme::new(LexSymbol::FunctionClosingBracket, c.to_string(), *loc));}
        } 
    
        // Line splitter
        if c == LINE_SPLITTER {
            chars.next();
            *loc = (loc.0, loc.1 + 1);
            return Some(Lexeme::new(LexSymbol::EndLine, LINE_SPLITTER.to_string(), *loc))
        }

        // Operational Symbols and GT/LT Math Symbols
        if c == '=' || c == '!' || c == '<' || c == '>' {
            // I LOVE MASSIVE READ TABLES MMMMMMMMM
            match c {
                '=' => {
                    chars.next();
                    *loc = (loc.0, loc.1 + 1);
                    let c = chars.peek().unwrap(); // FIXME: Unwrap :(
                    match c {                             // Fix the ones below too
                        '=' => {
                            chars.next();
                            *loc = (loc.0, loc.1 + 1);
                            return Some(Lexeme::new(LexSymbol::OperationalSymbol, "==".to_string(), *loc))
                        }
                        '>' => {
                            chars.next();
                            *loc = (loc.0, loc.1 + 1);
                            return Some(Lexeme::new(LexSymbol::OperationalSymbol, ">=".to_string(), *loc))
                        }
                        '<' => {
                            chars.next();
                            *loc = (loc.0, loc.1 + 1);
                            return Some(Lexeme::new(LexSymbol::OperationalSymbol, "<=".to_string(), *loc))
                        }
                        '!' => {
                            chars.next();
                            *loc = (loc.0, loc.1 + 1);
                            return Some(Lexeme::new(LexSymbol::OperationalSymbol, "!=".to_string(), *loc))
                        }
                        _ => {
                            return Some(Lexeme::new(LexSymbol::EqualSign, "=".to_string(), *loc))
                        }
                    }
                },
                '!' => {
                    chars.next();
                    *loc = (loc.0, loc.1 + 1);
                    let c = chars.peek().unwrap();
                    match c {
                        '=' => {
                            chars.next();
                            *loc = (loc.0, loc.1 + 1);
                            return Some(Lexeme::new(LexSymbol::OperationalSymbol, "!=".to_string(), *loc))
                        }
                        _ => {
                            continue;
                        }
                        
                    }
                },
                '<' => {
                    chars.next();
                    *loc = (loc.0, loc.1 + 1);
                    let c = chars.peek().unwrap();
                    match c {
                        '=' => {
                            chars.next();
                            *loc = (loc.0, loc.1 + 1);
                            return Some(Lexeme::new(LexSymbol::OperationalSymbol, "<=".to_string(), *loc))
                        }
                        _ => {
                            return Some(Lexeme::new(LexSymbol::OperationalSymbol, "<".to_string(), *loc))
                        }
                    }
                },
                '>' => {
                    chars.next();
                    *loc = (loc.0, loc.1 + 1);
                    let c = chars.peek().unwrap();
                    match c {
                        '=' => {
                            chars.next();
                            *loc = (loc.0, loc.1 + 1);
                            return Some(Lexeme::new(LexSymbol::OperationalSymbol, ">=".to_string(), *loc))
                        }
                        _ => {
                            return Some(Lexeme::new(LexSymbol::OperationalSymbol, ">".to_string(), *loc))
                        }
                    }
                },
                _ => {chars.next(); *loc = (loc.0, loc.1 + 1); continue;} // Should never happen
            }
        }

        // Rest of the OperationalSymbols
        if c == '+' || c == '-' || c == '*' || c == '/' {
            chars.next();
            *loc = (loc.0, loc.1 + 1);
            return Some(Lexeme::new(LexSymbol::OperationalSymbol, c.to_string(), *loc))
        }

        // Dot
        if c == '.' {
            chars.next();
            *loc = (loc.0, loc.1 + 1);
            return Some(Lexeme::new(LexSymbol::Dot, '.'.to_string(), *loc))
        }

        // Comma
        if c == ',' {
            chars.next();
            *loc = (loc.0, loc.1 + 1);
            return Some(Lexeme::new(LexSymbol::Comma, ','.to_string(), *loc))
        }

        // Double dot ( : )
        if c == ':' {
            chars.next();
            *loc = (loc.0, loc.1 + 1);
            return Some(Lexeme::new(LexSymbol::DoubleDot, ":".to_string(), *loc))
        }

        // Unrecognized: skip
        chars.next();
        *loc = (loc.0, loc.1 + 1);
    }

    None
}

/// Takes string, returns Vec<LexSm>
pub fn lexer(content: &str) -> Vec<Lexeme> {
    if LEX_DEBUG_PRINTS {println!("- - - LEXER")}

    // Main lexer loop
    let mut chars = content.chars().peekable();
    let mut tokens = Vec::new();
    let mut location_tracker: (usize, usize) = (0, 1);
    while let Some(token) = lex_token(&mut chars, &mut location_tracker) {
        tokens.push(token);
    }

    if LEX_DEBUG_PRINTS {println!("LEXED TOKENS:\n{:#?}", tokens)}

    if LEX_DEBUG_PRINTS {println!("- - - Lexer done!")}
    return tokens;
}