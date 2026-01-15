
//
// Main
// 

pub const SRC_FILE: &str = "sourcefile";    // Name of the sourcefile (move to args later)

//
// Lexer
//

// TODO: make settings better
// // There's a ton of hardcoded values in the lexer, bring them here.
// At this rate I might as well hardcode the rest
pub const LEX_DEBUG_PRINTS: bool = true;
pub const KEYWORDS: [&str; 6] = 
    ["let", "if", "function", "call", "return", "while"];
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