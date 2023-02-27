use crate::{ast::*, cp_analyzer::*};
use phoron_core::{rw::writer::Writer, serializer::Serializer};
use std::{
    error::Error,
    fmt,
    io::{BufWriter, Write},
};

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

pub struct Codegen<'c, W>
where
    W: Write,
{
    outfile: Serializer<'c, W>,
}

impl<'c, W> Codegen<'c, W>
where
    W: Write,
{
    pub fn new(outfile: &'c mut W) -> Self {
        Codegen {
            outfile: Serializer::new(Writer::new(outfile)),
        }
    }

    pub fn gen_bytecode(
        &mut self,
        program: &PhoronProgram,
        cp: &PhoronConstantPool,
    ) -> CodegenResult<()> {
        self.visit_program(&program, &cp)?;
        Ok(())
    }
}

impl<'a, 'c, W> PhoronAstVisitor<'a> for Codegen<'c, W>
where
    W: Write,
{
    type Input = &'a PhoronConstantPool;
    type Result = CodegenResult<()>;

    fn visit_program(&mut self, program: &PhoronProgram, input: Self::Input) -> Self::Result {
        todo!()
    }

    fn visit_header(&mut self, header: &PhoronHeader, input: Self::Input) -> Self::Result {
        todo!()
    }

    fn visit_sourcefile_def(
        &mut self,
        sourcefile_def: &PhoronSourceFileDef,
        input: Self::Input,
    ) -> Self::Result {
        todo!()
    }

    fn visit_class_def(&mut self, class_def: &PhoronClassDef, input: Self::Input) -> Self::Result {
        todo!()
    }

    fn visit_interface_def(
        &mut self,
        class_def: &PhoronInterfaceDef,
        input: Self::Input,
    ) -> Self::Result {
        todo!()
    }

    fn visit_super_def(&mut self, super_def: &PhoronSuperDef, input: Self::Input) -> Self::Result {
        todo!()
    }

    fn visit_body(&mut self, body: &PhoronBody, input: Self::Input) -> Self::Result {
        todo!()
    }

    fn visit_field_def(&mut self, field_def: &PhoronFieldDef, input: Self::Input) -> Self::Result {
        todo!()
    }

    fn visit_method_def(
        &mut self,
        method_def: &PhoronMethodDef,
        input: Self::Input,
    ) -> Self::Result {
        todo!()
    }

    fn visit_directive(&mut self, directive: &PhoronDirective, input: Self::Input) -> Self::Result {
        todo!()
    }

    fn visit_jvm_instruction(
        &mut self,
        instr: &JvmInstruction,
        input: Self::Input,
    ) -> Self::Result {
        todo!()
    }
}
