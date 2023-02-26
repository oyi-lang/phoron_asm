use crate::{
    ast::*,
    error_handler::{PhoronError, PhoronResult},
};

use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum ConstantPoolKind {
    Class {
        name_idx: u16,
    },
    Double([u8; 8]),
    Fieldref {
        name_idx: u16,
        name_and_type_idx: u16,
    },
    Float([u8; 4]),
    Integer([u8; 4]),
    InterfaceMethodref {
        class_idx: u16,
        name_and_type_idx: u16,
    },
    Long([u8; 8]),
    Methodref {
        class_idx: u16,
        name_and_type_idx: u16,
    },
    NameAndType {
        name_idx: u16,
        descriptor_idx: u16,
    },
    String {
        string_idx: u16,
    },
    Utf8(String),
}

// Map from Constant Pool entries to indices
pub type ConstantPool = HashMap<ConstantPoolKind, usize>;

#[derive(Debug)]
pub enum ConstantPoolAnalyzerError {}

impl std::error::Error for ConstantPoolAnalyzerError {}

impl fmt::Display for ConstantPoolAnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ConstantPoolAnalyzerError::*;
        todo!()
    }
}

pub type ConstantPoolAnalyzerResult<T> = Result<T, ConstantPoolAnalyzerError>;

pub struct ConstantPoolAnalyzer {
    cp_idx: usize,
}

impl ConstantPoolAnalyzer {
    pub fn new() -> Self {
        ConstantPoolAnalyzer { cp_idx: 1 } // idx 0 is not allowed
    }

    pub fn analyze(&mut self, program: &PhoronProgram) -> ConstantPoolAnalyzerResult<ConstantPool> {
        let mut cp = ConstantPool::new();
        self.visit_program(program, &mut cp)?;

        Ok(cp)
    }
}

impl<'a> PhoronAstVisitor<'a> for ConstantPoolAnalyzer {
    type Input = &'a mut ConstantPool;
    type Result = ConstantPoolAnalyzerResult<()>;

    fn visit_program(&mut self, program: &PhoronProgram, cp: Self::Input) -> Self::Result {
        self.visit_header(&program.header, cp)?;
        self.visit_body(&program.body, cp)?;

        Ok(())
    }

    fn visit_header(&mut self, header: &PhoronHeader, cp: Self::Input) -> Self::Result {
        if let Some(sourcefile_def) = &header.sourcefile_def {
            self.visit_sourcefile_def(sourcefile_def, cp)?;
        }

        match header.class_or_interface_def {
            PhoronClassOrInterface::Class(ref class_def) => self.visit_class_def(class_def, cp)?,
            PhoronClassOrInterface::Interface(ref interface_def) => {
                self.visit_interface_def(interface_def, cp)?
            }
        }
        self.visit_super_def(&header.super_def, cp)?;

        Ok(())
    }

    fn visit_sourcefile_def(
        &mut self,
        sourcefile_def: &PhoronSourceFileDef,
        cp: Self::Input,
    ) -> Self::Result {
        cp.insert(
            ConstantPoolKind::Utf8(sourcefile_def.source_file.clone()),
            self.cp_idx,
        );
        self.cp_idx += 1;

        Ok(())
    }

    fn visit_class_def(&mut self, class_def: &PhoronClassDef, cp: Self::Input) -> Self::Result {
        cp.insert(ConstantPoolKind::Utf8(class_def.name.clone()), self.cp_idx);
        self.cp_idx += 1;

        Ok(())
    }

    fn visit_interface_def(
        &mut self,
        interface_def: &PhoronInterfaceDef,
        cp: Self::Input,
    ) -> Self::Result {
        cp.insert(
            ConstantPoolKind::Utf8(interface_def.name.clone()),
            self.cp_idx,
        );
        self.cp_idx += 1;

        Ok(())
    }

    fn visit_super_def(&mut self, super_def: &PhoronSuperDef, cp: Self::Input) -> Self::Result {
        cp.insert(
            ConstantPoolKind::Utf8(super_def.super_class_name.clone()),
            self.cp_idx,
        );
        self.cp_idx += 1;

        Ok(())
    }

    fn visit_body(&mut self, body: &PhoronBody, cp: Self::Input) -> Self::Result {
        body.field_defs
            .iter()
            .try_for_each(|field_def| self.visit_field_def(field_def, cp));
        body.method_defs
            .iter()
            .try_for_each(|method_def| self.visit_method_def(method_def, cp));

        Ok(())
    }

    // pub name: String,
    // pub access_flags: Vec<PhoronFieldAccessFlag>,
    // pub field_descriptor: PhoronFieldDescriptor,
    // pub init_val: Option<PhoronFieldInitValue>,

    fn visit_field_def(&mut self, field_def: &PhoronFieldDef, cp: Self::Input) -> Self::Result {
        todo!()
    }

    //pub name: String,
    //pub access_flags: Vec<PhoronMethodAccessFlag>,
    //pub method_descriptor: PhoronMethodDescriptor,
    //pub instructions: Vec<PhoronInstruction>,

    fn visit_method_def(&mut self, method_def: &PhoronMethodDef, cp: Self::Input) -> Self::Result {
        // need to check presence in map
        cp.insert(ConstantPoolKind::Utf8(method_def.name.clone()), self.cp_idx);
        self.cp_idx += 1;

        // need to check presence in map
        cp.entry(ConstantPoolKind::Utf8(
            method_def.method_descriptor.to_string(),
        ))
        .or_insert_with(|| {
            let val = self.cp_idx;
            self.cp_idx += 1;
            val
        });

        // test
        println!("cp = {cp:#?}");

        // visit instructions
        method_def
            .instructions
            .iter()
            .try_for_each(|instr| match instr {
                PhoronInstruction::PhoronDirective(ref directive) => {
                    self.visit_directive(directive, cp)
                }
                PhoronInstruction::JvmInstruction(ref instr) => {
                    self.visit_jvm_instruction(instr, cp)
                }
                PhoronInstruction::PhoronLabel(ref label) => Ok(()),
            });

        Ok(())
    }

    fn visit_directive(&mut self, directive: &PhoronDirective, cp: Self::Input) -> Self::Result {
        todo!()
    }

    fn visit_jvm_instruction(
        &mut self,
        directive: &JvmInstruction,
        cp: Self::Input,
    ) -> Self::Result {
        todo!()
    }
}
