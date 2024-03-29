//! A Phoron abstraction of the JVM's  Constant Pool (arguably the most important aspect of the JVM
//! bytecode format).
//!
//! The indxeing of the Constant Pool elements is deterministic (the ordering is left unspecified
//! in the JVM specification) and follows a top-down recursive approach.
//!
use crate::ast::{attributes::*, *};

pub mod constant_pool;

use constant_pool::*;

use std::fmt;

#[derive(Debug)]
pub enum ConstantPoolAnalyzerError {
    IndexNotAvailable { component: &'static str },
}

impl std::error::Error for ConstantPoolAnalyzerError {}

impl fmt::Display for ConstantPoolAnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ConstantPoolAnalyzerError::*;

        write!(
            f,
            "{}",
            match *self {
                IndexNotAvailable { ref component } => {
                    format!("Constant Pool index not available for {component}")
                }
            },
        )
    }
}

pub type ConstantPoolAnalyzerResult<T> = Result<T, ConstantPoolAnalyzerError>;

pub struct ConstantPoolAnalyzer {
    cp_index: u16,
}

impl ConstantPoolAnalyzer {
    pub fn new() -> Self {
        ConstantPoolAnalyzer { cp_index: 1 } // index 0 is not allowed
    }

    /// check if the name (Utf8) is already in the constant pool, and if not, insert it.
    /// Update the Constant Pool index accordingly.
    fn analyze_name(
        &mut self,
        name: &str,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<u16> {
        Ok(*cp
            .0
            .entry(PhoronConstantPoolKind::Utf8(name.to_owned()))
            .or_insert_with(|| {
                let curr_cp_index = self.cp_index;
                self.cp_index += 1;
                curr_cp_index
            }))
    }

    /// check if the name (Utf8) is already in the constant pool, and if not, insert it.
    /// Update the Constant Pool index accordingly.
    fn analyze_string(
        &mut self,
        string_index: u16,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<u16> {
        Ok(*cp
            .0
            .entry(PhoronConstantPoolKind::String { string_index })
            .or_insert_with(|| {
                let curr_cp_index = self.cp_index;
                self.cp_index += 1;
                curr_cp_index
            }))
    }

    /// check if the int (i32) is already in the constant pool, and if not, insert it.
    /// Update the Constant Pool index accordingly.
    fn analyze_integer(
        &mut self,
        int: i32,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<u16> {
        Ok(*cp
            .0
            .entry(PhoronConstantPoolKind::Integer(int.to_be_bytes()))
            .or_insert_with(|| {
                let curr_cp_index = self.cp_index;
                self.cp_index += 1;
                curr_cp_index
            }))
    }

    /// check if the long (i64) is already in the constant pool, and if not, insert it.
    /// Update the Constant Pool index accordingly.
    fn analyze_long(
        &mut self,
        long: i64,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<u16> {
        Ok(*cp
            .0
            .entry(PhoronConstantPoolKind::Long(long.to_be_bytes()))
            .or_insert_with(|| {
                let curr_cp_index = self.cp_index;
                self.cp_index += 2; // extra index slot for LongInfo
                curr_cp_index
            }))
    }

    /// check if the float (f32) is already in the constant pool, and if not, insert it.
    /// Update the Constant Pool index accordingly.
    fn analyze_float(
        &mut self,
        float: f32,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<u16> {
        Ok(*cp
            .0
            .entry(PhoronConstantPoolKind::Float(float.to_be_bytes()))
            .or_insert_with(|| {
                let curr_cp_index = self.cp_index;
                self.cp_index += 1;
                curr_cp_index
            }))
    }
    /// check if the double (f64) is already in the constant pool, and if not, insert it.
    /// Update the Constant Pool index accordingly.
    fn analyze_double(
        &mut self,
        double: f64,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<u16> {
        Ok(*cp
            .0
            .entry(PhoronConstantPoolKind::Double(double.to_be_bytes()))
            .or_insert_with(|| {
                let curr_cp_index = self.cp_index;
                self.cp_index += 2; // extra index slot for DoubleInfo
                curr_cp_index
            }))
    }

    /// check if the NameAndType is already in the constant pool, and if not, insert it.
    /// Update the Constant Pool index accordingly.
    fn analyze_name_and_type(
        &mut self,
        name_index: u16,
        descriptor_index: u16,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<u16> {
        Ok(*cp
            .0
            .entry(PhoronConstantPoolKind::NameAndType {
                name_index,
                descriptor_index,
            })
            .or_insert_with(|| {
                let curr_cp_index = self.cp_index;
                self.cp_index += 1;
                curr_cp_index
            }))
    }

    /// check if the Fieldref is already in the constant pool, and if not, insert it.
    /// Update the Constant Pool index accordingly.
    fn analyze_field_ref(
        &mut self,
        class_index: u16,
        name_and_type_index: u16,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<u16> {
        Ok(*cp
            .0
            .entry(PhoronConstantPoolKind::Fieldref {
                class_index,
                name_and_type_index,
            })
            .or_insert_with(|| {
                let curr_cp_index = self.cp_index;
                self.cp_index += 1;
                curr_cp_index
            }))
    }

    /// check if the Methodref is already in the constant pool, and if not, insert it.
    /// Update the Constant Pool index accordingly.
    fn analyze_method_ref(
        &mut self,
        class_index: u16,
        name_and_type_index: u16,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<u16> {
        Ok(*cp
            .0
            .entry(PhoronConstantPoolKind::Methodref {
                class_index,
                name_and_type_index,
            })
            .or_insert_with(|| {
                let curr_cp_index = self.cp_index;
                self.cp_index += 1;
                curr_cp_index
            }))
    }

    /// check if the Class is already in the constant pool, and if not, insert it.
    /// Update the Constant Pool index accordingly.
    fn analyze_class(
        &mut self,
        name_index: u16,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<u16> {
        Ok(*cp
            .0
            .entry(PhoronConstantPoolKind::Class { name_index })
            .or_insert_with(|| {
                let curr_cp_index = self.cp_index;
                self.cp_index += 1;
                curr_cp_index
            }))
    }

    /// check if the class represented by the type descriptor is already in the constant pool,
    /// and if not, insert it. Update the Constant Pool index accordingly.
    fn analyze_field_descriptor(
        &mut self,
        field_desc: &PhoronFieldDescriptor,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<()> {
        let component_name = field_desc.to_string();

        match *field_desc {
            PhoronFieldDescriptor::BaseType(..) => {}

            PhoronFieldDescriptor::ObjectType { ref class_name } => {
                let class_name_idx = self.analyze_name(class_name, cp)?;
                self.analyze_class(class_name_idx, cp)?;
            }

            PhoronFieldDescriptor::ArrayType { .. } => {
                //self.analyze_field_descriptor(component_type, cp)?;
                let array_name_idx = self.analyze_name(&component_name, cp)?;
                self.analyze_class(array_name_idx, cp)?;
            }
        }

        Ok(())
    }

    /// check if the Class or interface represented by the type descriptor is already in the constant pool,
    /// and if not, insert it. Update the Constant Pool index accordingly.
    fn analyze_class_or_interface_type_descriptor(
        &mut self,
        component_type: &PhoronFieldDescriptor,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<()> {
        let component_name = component_type.to_string();

        match *component_type {
            PhoronFieldDescriptor::BaseType(..) => {}

            PhoronFieldDescriptor::ObjectType { ref class_name } => {
                // check
                let class_name_idx = self.analyze_name(class_name, cp)?;
                self.analyze_class(class_name_idx, cp)?;
            }

            PhoronFieldDescriptor::ArrayType { .. } => {
                // self.analyze_class_or_interface_type_descriptor(component_type, cp)?;
                let array_name_idx = self.analyze_name(&component_name, cp)?;
                self.analyze_class(array_name_idx, cp)?;
            }
        }

        Ok(())
    }

    pub fn analyze(
        &mut self,
        program: &PhoronProgram,
    ) -> ConstantPoolAnalyzerResult<PhoronConstantPool> {
        let mut cp = PhoronConstantPool::new();
        self.visit_program(program, &mut cp)?;

        Ok(cp)
    }
}

impl<'a> PhoronAstVisitor<'a> for ConstantPoolAnalyzer {
    type Input = &'a mut PhoronConstantPool;
    type Result = ConstantPoolAnalyzerResult<()>;

    fn visit_program(&mut self, program: &PhoronProgram, cp: Self::Input) -> Self::Result {
        self.visit_header(&program.header, cp)?;
        self.visit_body(&program.body, cp)?;

        Ok(())
    }

    fn visit_header(&mut self, header: &PhoronHeader, cp: Self::Input) -> Self::Result {
        self.visit_sourcefile_def(&header.sourcefile_def, cp)?;

        match header.class_or_interface_def {
            PhoronClassOrInterface::Class(ref class_def) => self.visit_class_def(class_def, cp)?,
            PhoronClassOrInterface::Interface(ref interface_def) => {
                self.visit_interface_def(interface_def, cp)?
            }
        }

        self.visit_super_def(&header.super_def, cp)?;

        header
            .implements_defs
            .iter()
            .try_for_each(|impl_def| self.visit_implements_def(impl_def, cp))?;

        Ok(())
    }

    fn visit_sourcefile_def(
        &mut self,
        sourcefile_def: &PhoronSourceFileDef,
        cp: Self::Input,
    ) -> Self::Result {
        self.analyze_name(PHORON_SOURCE_FILE, cp)?;
        self.analyze_name(&sourcefile_def.source_file, cp)?;

        Ok(())
    }

    fn visit_class_def(&mut self, class_def: &PhoronClassDef, cp: Self::Input) -> Self::Result {
        let name_index = self.analyze_name(&class_def.name, cp)?;
        self.analyze_class(name_index, cp)?;

        Ok(())
    }

    fn visit_interface_def(
        &mut self,
        interface_def: &PhoronInterfaceDef,
        cp: Self::Input,
    ) -> Self::Result {
        let name_index = self.analyze_name(&interface_def.name, cp)?;
        self.analyze_class(name_index, cp)?;

        Ok(())
    }

    fn visit_super_def(&mut self, super_def: &PhoronSuperDef, cp: Self::Input) -> Self::Result {
        let name_index = self.analyze_name(&super_def.super_class_name, cp)?;
        self.analyze_class(name_index, cp)?;

        Ok(())
    }

    fn visit_implements_def(
        &mut self,
        impl_def: &PhoronImplementsDef,
        cp: Self::Input,
    ) -> Self::Result {
        let name_index = self.analyze_name(&impl_def.class_name, cp)?;
        self.analyze_class(name_index, cp)?;

        Ok(())
    }

    fn visit_body(&mut self, body: &PhoronBody, cp: Self::Input) -> Self::Result {
        body.field_defs
            .iter()
            .try_for_each(|field_def| self.visit_field_def(field_def, cp))?;
        body.method_defs
            .iter()
            .try_for_each(|method_def| self.visit_method_def(method_def, cp))?;

        Ok(())
    }

    fn visit_field_def(&mut self, field_def: &PhoronFieldDef, cp: Self::Input) -> Self::Result {
        self.analyze_name(&field_def.name, cp)?;
        self.analyze_name(&field_def.field_descriptor.to_string(), cp)?;

        if let Some(field_init_val) = &field_def.init_val {
            match field_init_val {
                PhoronFieldInitValue::Integer(int) => {
                    self.analyze_integer(*int as i32, cp)?;
                }
                PhoronFieldInitValue::Double(double) => {
                    self.analyze_double(*double, cp)?;
                }
                PhoronFieldInitValue::QuotedString(ref s) => {
                    let string_index = self.analyze_name(s, cp)?;
                    self.analyze_string(string_index, cp)?;
                }
            }
        }

        Ok(())
    }

    fn visit_method_def(&mut self, method_def: &PhoronMethodDef, cp: Self::Input) -> Self::Result {
        self.analyze_name(&method_def.name, cp)?;
        self.analyze_name(&method_def.method_descriptor.to_string(), cp)?;

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
                PhoronInstruction::PhoronLabel(..) => Ok(()),
            })?;

        Ok(())
    }

    fn visit_directive(&mut self, directive: &PhoronDirective, cp: Self::Input) -> Self::Result {
        match directive {
            PhoronDirective::LimitStack { .. } => {} // do nothing

            PhoronDirective::LimitLocals { .. } => {} // do nothing

            PhoronDirective::Throws { ref class_name, .. } => {
                self.analyze_name(PHORON_EXCEPTIONS, cp)?;

                let name_index = self.analyze_name(class_name, cp)?;
                self.analyze_class(name_index, cp)?;
            }

            PhoronDirective::LineNumber { .. } => {
                self.analyze_name(PHORON_LINE_NUMBER_TABLE, cp)?;
            }

            PhoronDirective::Var {
                ref name,
                ref field_descriptor,
                ..
            } => {
                self.analyze_name(PHORON_LOCAL_VARIABLE_TABLE, cp)?;
                self.analyze_name(name, cp)?;
                self.analyze_name(&field_descriptor.to_string(), cp)?;
                self.analyze_field_descriptor(field_descriptor, cp)?;
            }

            PhoronDirective::Catch { ref class_name, .. } => {
                if class_name != "all" {
                    let name_index = self.analyze_name(class_name, cp)?;
                    self.analyze_class(name_index, cp)?;
                }
            }
        }
        Ok(())
    }

    fn visit_jvm_instruction(&mut self, instr: &JvmInstruction, cp: Self::Input) -> Self::Result {
        use JvmInstruction::*;

        // if we have reached this stage, then we know that there is a `Code` attribute for sure.
        self.analyze_name(PHORON_CODE, cp)?;

        match instr {
            Anewarray { ref component_type } => {
                self.analyze_class_or_interface_type_descriptor(component_type, cp)?
            }

            Checkcast { ref cast_type } => {
                self.analyze_class_or_interface_type_descriptor(&cast_type, cp)?
            }

            Getstatic {
                ref class_name,
                ref field_name,
                ref field_descriptor,
                ..
            } => {
                let class_name_index = self.analyze_name(class_name, cp)?;
                let class_index = self.analyze_class(class_name_index, cp)?;

                let field_name_index = self.analyze_name(field_name, cp)?;
                let field_descriptor_index =
                    self.analyze_name(&field_descriptor.to_string(), cp)?;

                let field_name_and_type_index =
                    self.analyze_name_and_type(field_name_index, field_descriptor_index, cp)?;

                self.analyze_field_ref(class_index, field_name_and_type_index, cp)?;
            }

            Getfield {
                ref class_name,
                ref field_name,
                ref field_descriptor,
                ..
            } => {
                let class_name_index = self.analyze_name(class_name, cp)?;
                let class_index = self.analyze_class(class_name_index, cp)?;

                let field_name_index = self.analyze_name(field_name, cp)?;
                let field_descriptor_index =
                    self.analyze_name(&field_descriptor.to_string(), cp)?;

                let field_name_and_type_index =
                    self.analyze_name_and_type(field_name_index, field_descriptor_index, cp)?;

                self.analyze_field_ref(class_index, field_name_and_type_index, cp)?;
            }

            Instanceof { ref check_type } => {
                self.analyze_class_or_interface_type_descriptor(check_type, cp)?;
            }

            Invokeinterface {
                ref interface_name,
                ref method_name,
                ref method_descriptor,
                ..
            } => {
                let class_name_index = self.analyze_name(interface_name, cp)?;
                let class_index = self.analyze_class(class_name_index, cp)?;

                let method_name_index = self.analyze_name(method_name, cp)?;
                let method_descriptor_index =
                    self.analyze_name(&method_descriptor.to_string(), cp)?;

                let method_name_and_type_index =
                    self.analyze_name_and_type(method_name_index, method_descriptor_index, cp)?;

                self.analyze_method_ref(class_index, method_name_and_type_index, cp)?;
            }

            Invokespecial {
                ref class_name,
                ref method_name,
                ref method_descriptor,
                ..
            } => {
                let class_name_index = self.analyze_name(class_name, cp)?;
                let class_index = self.analyze_class(class_name_index, cp)?;

                let method_name_index = self.analyze_name(method_name, cp)?;
                let method_descriptor_index =
                    self.analyze_name(&method_descriptor.to_string(), cp)?;

                let method_name_and_type_index =
                    self.analyze_name_and_type(method_name_index, method_descriptor_index, cp)?;

                self.analyze_method_ref(class_index, method_name_and_type_index, cp)?;
            }

            Invokestatic {
                ref class_name,
                ref method_name,
                ref method_descriptor,
                ..
            } => {
                let class_name_index = self.analyze_name(class_name, cp)?;
                let class_index = self.analyze_class(class_name_index, cp)?;

                let method_name_index = self.analyze_name(method_name, cp)?;
                let method_descriptor_index =
                    self.analyze_name(&method_descriptor.to_string(), cp)?;

                let method_name_and_type_index =
                    self.analyze_name_and_type(method_name_index, method_descriptor_index, cp)?;

                self.analyze_method_ref(class_index, method_name_and_type_index, cp)?;
            }

            Invokevirtual {
                ref class_name,
                ref method_name,
                ref method_descriptor,
                ..
            } => {
                let class_name_index = self.analyze_name(class_name, cp)?;
                let class_index = self.analyze_class(class_name_index, cp)?;

                let method_name_index = self.analyze_name(method_name, cp)?;
                let method_descriptor_index =
                    self.analyze_name(&method_descriptor.to_string(), cp)?;

                let method_name_and_type_index =
                    self.analyze_name_and_type(method_name_index, method_descriptor_index, cp)?;

                self.analyze_method_ref(class_index, method_name_and_type_index, cp)?;
            }

            Ldc(ref ldc_val) => match ldc_val {
                LdcValue::Integer(int) => {
                    self.analyze_integer(*int, cp)?;
                }

                LdcValue::Float(float) => {
                    self.analyze_float(*float, cp)?;
                }

                LdcValue::QuotedString(string) => {
                    let string_index = self.analyze_name(string, cp)?;
                    self.analyze_string(string_index, cp)?;
                }
            },

            Ldcw(ref ldcw_val) => match ldcw_val {
                LdcwValue::Integer(int) => {
                    self.analyze_integer(*int, cp)?;
                }

                LdcwValue::Float(float) => {
                    self.analyze_float(*float, cp)?;
                }

                LdcwValue::QuotedString(string) => {
                    let string_index = self.analyze_name(string, cp)?;
                    self.analyze_string(string_index, cp)?;
                }
            },

            Ldc2w(ref ldc2w_val) => match ldc2w_val {
                Ldc2wValue::Long(long) => {
                    self.analyze_long(*long, cp)?;
                }
                Ldc2wValue::Double(double) => {
                    self.analyze_double(*double, cp)?;
                }
            },

            Multianewarray {
                ref component_type, ..
            } => self.analyze_field_descriptor(component_type, cp)?,

            Newarray { ref component_type } => {
                self.analyze_name(&component_type.to_string(), cp)?;
            }

            New { ref class_name } => {
                let name_index = self.analyze_name(class_name, cp)?;
                self.analyze_class(name_index, cp)?;
            }

            Putfield {
                ref class_name,
                ref field_name,
                ref field_descriptor,
                ..
            } => {
                let class_name_index = self.analyze_name(class_name, cp)?;
                let class_index = self.analyze_class(class_name_index, cp)?;

                let field_name_index = self.analyze_name(field_name, cp)?;
                let field_descriptor_index =
                    self.analyze_name(&field_descriptor.to_string(), cp)?;

                let field_name_and_type_index =
                    self.analyze_name_and_type(field_name_index, field_descriptor_index, cp)?;

                self.analyze_field_ref(class_index, field_name_and_type_index, cp)?;
            }

            Putstatic {
                ref class_name,
                ref field_name,
                ref field_descriptor,
                ..
            } => {
                let class_name_index = self.analyze_name(class_name, cp)?;
                let class_index = self.analyze_class(class_name_index, cp)?;

                let field_name_index = self.analyze_name(field_name, cp)?;
                let field_descriptor_index =
                    self.analyze_name(&field_descriptor.to_string(), cp)?;

                let field_name_and_type_index =
                    self.analyze_name_and_type(field_name_index, field_descriptor_index, cp)?;

                self.analyze_field_ref(class_index, field_name_and_type_index, cp)?;
            }

            _ => {} // no analysis needed
        }

        Ok(())
    }
}
