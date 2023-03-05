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
#[derive(Debug, PartialEq)]
pub struct PhoronConstantPool(HashMap<PhoronConstantPoolKind, u16>);

impl PhoronConstantPool {
    pub fn new() -> Self {
        PhoronConstantPool(HashMap::new())
    }

    pub fn insert(&mut self, cp_key: PhoronConstantPoolKind, cp_val: u16) {
        self.0.insert(cp_key, cp_val);
    }

    /// Return the size of the Constant Pool
    pub fn len(&self) -> usize {
        let mut len = 0;

        for (cp_kind, _) in &self.0 {
            match cp_kind {
                PhoronConstantPoolKind::Long(_) | PhoronConstantPoolKind::Double(_) => len += 2,
                _ => len += 1,
            }
        }
        len
    }

    /// Return an iterator over the contents of the Constant Pool.
    pub fn iter(&self) -> Iter<'_, PhoronConstantPoolKind, u16> {
        self.0.iter()
    }

    /// Retrieve the index in the Constant Pool, if present, of the
    /// given double.
    pub fn get_double(&self, double: f64) -> Option<&u16> {
        self.0
            .get(&PhoronConstantPoolKind::Double(double.to_be_bytes()))
    }

    /// Retrieve the index in the Constant Pool, if present, of the
    /// given float.
    pub fn get_float(&self, float: f32) -> Option<&u16> {
        self.0
            .get(&PhoronConstantPoolKind::Float(float.to_be_bytes()))
    }

    /// Retrieve the index in the Constant Pool, if present, of the
    /// given long.
    pub fn get_long(&self, long: i64) -> Option<&u16> {
        self.0
            .get(&PhoronConstantPoolKind::Long(long.to_be_bytes()))
    }

    /// Retrieve the index in the Constant Pool, if present, of the
    /// given integer.
    pub fn get_integer(&self, int: i32) -> Option<&u16> {
        self.0
            .get(&PhoronConstantPoolKind::Integer(int.to_be_bytes()))
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
                println!(
                    "inserting class {:#?} at index {:#?}",
                    name_index, curr_cp_index
                );
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

            PhoronFieldDescriptor::ArrayType { ref component_type } => {
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
        component_type: &ClassOrArrayTypeDescriptor,
        cp: &mut PhoronConstantPool,
    ) -> ConstantPoolAnalyzerResult<()> {
        let component_name = component_type.to_string();

        match *component_type {
            ClassOrArrayTypeDescriptor::BaseType(..) => {}

            ClassOrArrayTypeDescriptor::ClassType { ref class_name } => {
                // check
                let class_name_idx = self.analyze_name(class_name, cp)?;
                self.analyze_class(class_name_idx, cp)?;
            }

            ClassOrArrayTypeDescriptor::ArrayType { ref component_type } => {
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
                PhoronInstruction::PhoronLabel(ref label) => Ok(()),
            })?;

        Ok(())
    }

    fn visit_directive(&mut self, directive: &PhoronDirective, cp: Self::Input) -> Self::Result {
        match directive {
            PhoronDirective::Catch {
                ref class_name,
                ref from_label,
                ref to_label,
                ref handler_label,
            } => {
                let name_index = self.analyze_name(class_name, cp)?;
                self.analyze_class(name_index, cp)?;
            }

            _ => {}
        }

        Ok(())
    }

    fn visit_jvm_instruction(&mut self, instr: &JvmInstruction, cp: Self::Input) -> Self::Result {
        use JvmInstruction::*;

        // if we have reached this stage, then we know that there is a `Code` attribute for sure.
        self.analyze_name(PHORON_CODE, cp)?;

        match instr {
            Anewarray { ref component_type } => {
                self.analyze_class_or_interface_type_descriptor(component_type, cp)?;
            }

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

            Instanceof { ref check_type } => {
                self.analyze_class_or_interface_type_descriptor(check_type, cp)?;
            }

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
            },

            Ldc2w(ref ldc2w_val) => match ldc2w_val {
                Ldc2wValue::Long(long) => {
                    self.analyze_long(*long, cp)?;
                }
                Ldc2wValue::Double(double) => {
                    self.analyze_double(*double, cp)?;
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
            } => self.analyze_field_descriptor(component_type, cp)?,

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
