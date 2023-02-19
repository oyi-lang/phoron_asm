use crate::{ast::PhoronProgram, lexer::Lexer};

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {}

impl std::error::Error for ParserError {}

use std::fmt;
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "parser error")
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser { lexer }
    }

    pub fn parse(&mut self) -> ParserResult<PhoronProgram> {
        todo!()
    }
}
