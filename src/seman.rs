use crate::parser::Statement;
use crate::compiler_settings::*;


/// Semantic analysis
pub fn analyze(statements: Vec<Statement>) {
    if SEMAN_DEBUG_PRINTS {println!("- - - SEMAN")}

    if SEMAN_DEBUG_PRINTS {println!("- - - Sem Analysis done!")}
}