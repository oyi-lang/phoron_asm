use crate::{codegen, cp_analyzer, lexer, parser};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum PhoronError {
    LexerError(lexer::LexerError),
    ParserError(parser::ParserError),
    ConstantPoolAnalyzerError(cp_analyzer::ConstantPoolAnalyzerError),
    CodegenError(codegen::CodegenError),
}

//impl From<lexer::LexerError> for PhoronError {
//    fn from(lex_err: lexer::LexerError) -> Self {
//        PhoronError::LexerError(lex_err)
//    }
//}

impl Error for PhoronError {}

impl fmt::Display for PhoronError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PhoronError::*;
        write!(
            f,
            "{}",
            match *self {
                LexerError(ref lex_err) => lex_err.to_string(),
                ParserError(ref parse_err) => parse_err.to_string(),
                ConstantPoolAnalyzerError(ref cp_err) => cp_err.to_string(),
                CodegenError(ref codegen_err) => codegen_err.to_string(),
            }
        )
    }
}

pub type PhoronResult<T> = Result<T, PhoronError>;
