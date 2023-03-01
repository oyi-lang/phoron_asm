use crate::{ast::*, attributes::*};

use std::{
    collections::{hash_map::Iter, HashMap},
    fmt,
};

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum PhoronConstantPoolKind {
    Class {
        name_index: u16,
    },
    Double([u8; 8]),
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Float([u8; 4]),
    Integer([u8; 4]),
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Long([u8; 8]),
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    String {
        string_index: u16,
    },
    Utf8(String),
}

// Map from Constant Pool entries to indices
#[derive(Debug)]
pub struct PhoronConstantPool(HashMap<PhoronConstantPoolKind, u16>);

impl PhoronConstantPool {
    /// Return the size of the Constant Pool
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return an iterator over the contents of the Constant Pool.
    pub fn iter(&self) -> Iter<'_, PhoronConstantPoolKind, u16> {
        self.0.iter()
    }

    /// Retrueve the index in the Constant Pool, if present, of the
    /// given String.
    pub fn get_string(&self, string: &str) -> Option<&u16> {
        self.get_name(string).and_then(|string_index| {
            self.0.get(&PhoronConstantPoolKind::String {
                string_index: *string_index,
            })
        })
    }

    /// Retrieve the index in the Constant Pool, if present, of the
    /// given name.
    pub fn get_name(&self, name: &str) -> Option<&u16> {
        self.0.get(&PhoronConstantPoolKind::Utf8(name.to_string()))
    }

    /// Retrieve the index in the Constant Pool, if present,  of the given class.
    pub fn get_class(&self, class_name: &str) -> Option<&u16> {
        self.get_name(class_name).and_then(|name_index| {
            self.0.get(&PhoronConstantPoolKind::Class {
                name_index: *name_index,
            })
        })
    }

    /// Retrieve the index in the Constant Pool, if present, of the given NamdAndType.
    pub fn get_name_and_type(&self, name: &str, descriptor: &str) -> Option<&u16> {
        self.get_name(name).and_then(|name_index| {
            self.get_name(descriptor).and_then(|descriptor_index| {
                self.0.get(&PhoronConstantPoolKind::NameAndType {
                    name_index: *name_index,
                    descriptor_index: *descriptor_index,
                })
            })
        })
    }

    /// Retrieve the index in the Constant Pool, if present, of the given Fieldref.
    pub fn get_fieldref(
        &self,
        class_name: &str,
        field_name: &str,
        field_descriptor: &str,
    ) -> Option<&u16> {
        self.get_class(class_name).and_then(|class_index| {
            self.get_name_and_type(field_name, field_descriptor)
                .and_then(|name_and_type_index| {
                    self.0.get(&PhoronConstantPoolKind::Fieldref {
                        class_index: *class_index,
                        name_and_type_index: *name_and_type_index,
                    })
                })
        })
    }

    /// Retrieve the index in the Constant Pool, if present, of the given Methodref.
    pub fn get_methodref(
        &self,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
    ) -> Option<&u16> {
        self.get_class(class_name).and_then(|class_index| {
            self.get_name_and_type(method_name, method_descriptor)
                .and_then(|name_and_type_index| {
                    self.0.get(&PhoronConstantPoolKind::Methodref {
                        class_index: *class_index,
                        name_and_type_index: *name_and_type_index,
                    })
                })
        })
    }
}

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
    fn analyze_int(
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
                self.cp_index += 1;
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
                self.cp_index += 1;
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

    /// check if the InterfaceMethodref is already in the constant pool, and if not, insert it.
    /// Update the Constant Pool index accordingly.
    fn analyze_interface_method_ref(
        &mut self,
        class_index: u16,
        name_and_type_index: u16,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<u16> {
        Ok(*cp
            .0
            .entry(PhoronConstantPoolKind::InterfaceMethodref {
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
                println!("inserting class with index {:#?}", curr_cp_index);
                self.cp_index += 1;
                curr_cp_index
            }))
    }

    pub fn analyze(
        &mut self,
        program: &PhoronProgram,
    ) -> ConstantPoolAnalyzerResult<PhoronConstantPool> {
        let mut cp = PhoronConstantPool(HashMap::new());
        self.visit_program(program, &mut cp)?;

        println!("cp = {cp:#?}");

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

    fn visit_body(&mut self, body: &PhoronBody, cp: Self::Input) -> Self::Result {
        body.field_defs
            .iter()
            .try_for_each(|field_def| self.visit_field_def(field_def, cp))?;
        body.method_defs
            .iter()
            .try_for_each(|method_def| self.visit_method_def(method_def, cp))?;

        Ok(())
    }

    // pub name: String,
    // pub access_flags: Vec<PhoronFieldAccessFlag>,
    // pub field_descriptor: PhoronFieldDescriptor,
    // pub init_val: Option<PhoronFieldInitValue>,

    fn visit_field_def(&mut self, field_def: &PhoronFieldDef, cp: Self::Input) -> Self::Result {
        todo!()
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
                PhoronInstruction::PhoronLabel(ref label) => Ok(()),
            })?;

        Ok(())
    }

    // todo
    fn visit_directive(&mut self, directive: &PhoronDirective, cp: Self::Input) -> Self::Result {
        Ok(())
    }

    fn visit_jvm_instruction(&mut self, instr: &JvmInstruction, cp: Self::Input) -> Self::Result {
        use JvmInstruction::*;

        // if we have reached this stage, then we know that there is a `Code` attribute for sure.
        self.analyze_name(PHORON_CODE, cp)?;

        match instr {
            Anewarray { ref component_type } => {}
            Aload { ref varnum } => {}
            Arraylength => {}
            Astore { ref varnum } => {}
            Bipush(ref sb) => {}
            Caload => {}
            Checkcast { ref cast_type } => {}
            Dload { ref varnum } => {}
            Dstore { ref varnum } => {}
            Fload { ref varnum } => {}
            Fstore { ref varnum } => {}

            Getstatic {
                ref class_name,
                ref field_name,
                ref field_descriptor,
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

            Goto { ref label } => {}
            Gotow { ref label } => {}
            Ifacmpeq { ref label } => {}
            Ifacmpne { ref label } => {}
            Ificmpeq { ref label } => {}
            Ificmpge { ref label } => {}
            Ificmpgt { ref label } => {}
            Ificmple { ref label } => {}
            Ificmplt { ref label } => {}
            Ificmpne { ref label } => {}
            Ifeq { ref label } => {}
            Ifge { ref label } => {}
            Ifgt { ref label } => {}
            Ifle { ref label } => {}
            Iflt { ref label } => {}
            Ifne { ref label } => {}
            Ifnonnull { ref label } => {}
            Ifnull { ref label } => {}
            Iinc {
                ref varnum,
                ref delta,
            } => {}
            Iload { ref varnum } => {}
            Instanceof { ref check_type } => {}
            Invokeinterface {
                ref interface_name,
                ref method_name,
                ref method_descriptor,
                ref ub,
            } => {}

            Invokespecial {
                ref class_name,
                ref method_name,
                ref method_descriptor,
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
            } => {}

            Invokevirtual {
                ref class_name,
                ref method_name,
                ref method_descriptor,
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

            Istore { ref varnum } => {}
            Jsrw { ref label } => {}
            Jsr { ref label } => {}
            Ldc2w(ref ldc2w_val) => {}
            Ldcw(ref ldc_val) => {}

            Ldc(ref ldc_val) => match ldc_val {
                LdcValue::Double(double) => {}
                LdcValue::Integer(int) => {}
                LdcValue::QuotedString(string) => {
                    let string_index = self.analyze_name(string, cp)?;
                    self.analyze_string(string_index, cp)?;
                }
            },

            Ldiv => {}
            Lload { ref varnum } => {}
            Lookupswitch {
                ref switches,
                ref default,
            } => {}
            Lstore { ref varnum } => {}
            Multianewarray {
                ref component_type,
                ref dimensions,
            } => {}
            Newarray { ref component_type } => {}
            New { ref class_name } => {}
            Putfield {
                ref class_name,
                ref field_name,
                ref field_descriptor,
            } => {}
            Putstatic {
                ref class_name,
                ref field_name,
                ref field_descriptor,
            } => {}
            Ret { ref varnum } => {}
            Sipush(ref ss) => {}
            Tableswitch {
                ref low,
                ref high,
                ref switches,
                ref default,
            } => {}
            _ => {}
        }

        Ok(())
    }
}
