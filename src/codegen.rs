use crate::ast::*;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum CodegenError {}

impl Error for CodegenError {}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            "
            codegen error
            "
        )
    }
}

pub type CodegenResult<T> = Result<T, CodegenError>;

pub struct Codegen;

//impl PhoronAstVisitor for Codegen {
//    type Result = CodegenResult<()>;
//
//    fn visit_program(&mut self, program: &PhoronProgram) -> Self::Result {
//        todo!()
//    }
//
//    fn visit_sourcefile_def(&mut self, sourcefile_def: &PhoronSourceFileDef) -> Self::Result {
//        todo!()
//    }
//}
