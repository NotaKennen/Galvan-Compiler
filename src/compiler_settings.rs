
//
// Main
// 

pub const SRC_FILE: &str = "sourcefile";    // Name of the sourcefile (move to args later)

//
// Lexer
//
pub const LEX_DEBUG_PRINTS: bool = true;
pub const KEYWORDS: [&str; 2] = 
    ["let", "if"];
pub const MATH_SYMBOLS: [&str; 9] = 
    ["+", "-", "*", "/", "<", ">", "=>", "<=", "="];
pub const OPEN_BRACES: [&str; 3] = 
    ["(", "[", "{"];
pub const CLOSED_BRACES: [&str; 3] = 
    [")", "]", "}"];

// 
// Parser
//

pub const LINE_SPLITTER: char = ';';