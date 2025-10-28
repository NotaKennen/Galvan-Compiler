use crate::{compiler_settings::PAR_DEBUG_PRINTS, lexer::{LexSymbol, Lexeme}};

//
// NOTES
//


// 
// STRUCTS
//


//
// FUNCTIONS
//

/// Organizes a Vec<Lexeme> into a Vec<Vec<Lexeme>> where each internal
/// vec represents one line indicated by linesplitter
/// FIXME: braces not counted in by this
fn organize_lines(lexersymbols: Vec<Lexeme>) -> Vec<Vec<Lexeme>> {
    let mut returnable: Vec<Vec<Lexeme>> = vec![];
    let mut line: Vec<Lexeme> = vec![];

    for lexeme in lexersymbols {
        line.push(lexeme.clone());
        if lexeme.symbol == LexSymbol::EndLine {
            returnable.push(line.clone());
            line.clear();
        }
    }

    returnable
}

/// Parses one line
fn parse_line(organized_line: Vec<Lexeme>) {

}

///
pub fn parser(lexersymbols: Vec<Lexeme>) {
    if PAR_DEBUG_PRINTS {println!("- - - PARSER")}
    let organized_lines = organize_lines(lexersymbols);
    println!("[DEBUG] Organized lines:\n{:#?}", organized_lines);

    for org_line in organized_lines {
        parse_line(org_line);
    }
    
    if PAR_DEBUG_PRINTS {println!("- Parser done!")}
}