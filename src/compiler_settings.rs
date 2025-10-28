
//
// Main
// 

pub const SRC_FILE: &str = "sourcefile";    // Name of the sourcefile (move to args later)

//
// Lexer
//
pub const LEX_DEBUG_PRINTS: bool = true;
pub const KEYWORDS: [&str; 3] = 
    ["let", "if", "fn"];
pub const MATH_SYMBOLS: [char; 7] = 
    ['+', '-', '*', '/', '<', '>', '='];
pub const WHITESPACE: [char; 4] = 
    [' ', '\n', '\t', '\r'];
pub const OPEN_BRACES: [char; 3] = 
    ['(', '[', '{'];
pub const CLOSED_BRACES: [char; 3] = 
    [')', ']', '}'];

// 
// Parser
//
pub const PAR_DEBUG_PRINTS: bool = true;
pub const LINE_SPLITTER: char = ';';