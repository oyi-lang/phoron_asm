use crate::{ast::*, cp_analyzer::*};
use phoron_core::{
    error::SerializeError,
    model::{
        access_flags::*,
        constant_pool::{tags::*, types::CpInfo},
        *,
    },
    rw::writer::Writer,
    serializer::Serializer,
};
use std::{
    error::Error,
    fmt,
    io::{BufWriter, Write},
};

#[derive(Debug)]
pub enum CodegenError {
    Missing { component: &'static str },

    SerializeError(SerializeError),
}

impl Error for CodegenError {}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CodegenError::*;

        write!(
            f,
            "{}",
            match *self {
                Missing { ref component } => format!("Missing : {component}"),
                SerializeError(ref ser_err) => ser_err.to_string(),
            }
        )
    }
}

impl From<SerializeError> for CodegenError {
    fn from(ser_err: SerializeError) -> Self {
        CodegenError::SerializeError(ser_err)
    }
}

pub type CodegenResult<T> = Result<T, CodegenError>;

const MAGIC: u32 = 0xcafebabe;
const MAJOR_VERSION: u16 = 45;
const MINOR_VERSION: u16 = 3;

pub struct Codegen<'c, W>
where
    W: Write,
{
    outfile: Serializer<'c, W>,
    classfile: ClassFile,
}

impl<'c, W> Codegen<'c, W>
where
    W: Write,
{
    pub fn new(outfile: &'c mut W) -> Self {
        Codegen {
            outfile: Serializer::new(Writer::new(outfile)),
            classfile: ClassFile::default(),
        }
    }
    //pub const CONSTANT_INVALID_DEFAULT: u8 = 255;
    //pub const CONSTANT_CLASS: u8 = 7;
    //pub const CONSTANT_FIELD_REF: u8 = 9;
    //pub const CONSTANT_METHOD_REF: u8 = 10;
    //pub const CONSTANT_INTERFACE_METHOD_REF: u8 = 11;
    //pub const CONSTANT_STRING: u8 = 8;
    //pub const CONSTANT_INTEGER: u8 = 3;
    //pub const CONSTANT_FLOAT: u8 = 4;
    //pub const CONSTANT_LONG: u8 = 5;
    //pub const CONSTANT_DOUBLE: u8 = 6;
    //pub const CONSTANT_NAME_AND_TYPE: u8 = 12;
    //pub const CONSTANT_UTF8: u8 = 1;
    //pub const CONSTANT_METHOD_HANDLE: u8 = 15;
    //pub const CONSTANT_METHOD_TYPE: u8 = 16;
    //pub const CONSTANT_DYNAMIC: u8 = 17;
    //pub const CONSTANT_INVOKE_DYNAMIC: u8 = 18;
    //pub const CONSTANT_MODULE: u8 = 19;
    //pub const CONSTANT_PACKAGE: u8 = 20;

    fn gen_constant_pool(&mut self, cp: &PhoronConstantPool) -> CodegenResult<()> {
        let constant_pool_count = cp.len();
        self.classfile.constant_pool_count = constant_pool_count as u16;

        let mut constant_pool = Vec::with_capacity(constant_pool_count);
        constant_pool.push(None); // index is not used

        for (cp_key, cp_idx) in cp.iter() {
            match cp_key {
                PhoronConstantPoolKind::Class { ref name_index } => {
                    constant_pool.push(Some(CpInfo::ConstantClassInfo {
                        tag: CONSTANT_CLASS,
                        name_index: *name_index,
                    }))
                }

                PhoronConstantPoolKind::Fieldref {
                    ref class_index,
                    ref name_and_type_index,
                } => constant_pool.push(Some(CpInfo::ConstantFieldrefInfo {
                    tag: CONSTANT_FIELD_REF,
                    class_index: *class_index,
                    name_and_type_index: *name_and_type_index,
                })),

                PhoronConstantPoolKind::Methodref {
                    ref class_index,
                    ref name_and_type_index,
                } => constant_pool.push(Some(CpInfo::ConstantMethodrefInfo {
                    tag: CONSTANT_METHOD_REF,
                    class_index: *class_index,
                    name_and_type_index: *name_and_type_index,
                })),

                PhoronConstantPoolKind::InterfaceMethodref {
                    ref class_index,
                    ref name_and_type_index,
                } => constant_pool.push(Some(CpInfo::ConstantInterfaceMethodrefInfo {
                    tag: CONSTANT_INTERFACE_METHOD_REF,
                    class_index: *class_index,
                    name_and_type_index: *name_and_type_index,
                })),

                PhoronConstantPoolKind::String { ref string_index } => {
                    constant_pool.push(Some(CpInfo::ConstantStringInfo {
                        tag: CONSTANT_STRING,
                        string_index: *string_index,
                    }))
                }

                PhoronConstantPoolKind::Integer(int_bytes) => {
                    todo!()
                }

                PhoronConstantPoolKind::Float(float_bytes) => {
                    todo!()
                }
                PhoronConstantPoolKind::Long(long_bytes) => {
                    todo!()
                }
                PhoronConstantPoolKind::Double(double_bytes) => {
                    todo!()
                }

                PhoronConstantPoolKind::NameAndType {
                    ref name_index,
                    ref descriptor_index,
                } => constant_pool.push(Some(CpInfo::ConstantNameAndTypeInfo {
                    tag: CONSTANT_NAME_AND_TYPE,
                    name_index: *name_index,
                    descriptor_index: *descriptor_index,
                })),

                PhoronConstantPoolKind::Utf8(ref s) => {
                    constant_pool.push(Some(CpInfo::ConstantUtf8Info {
                        tag: CONSTANT_UTF8,
                        length: s.len() as u16,
                        bytes: s.bytes().collect::<_>(),
                    }))
                }
            }
        }

        self.classfile.constant_pool = constant_pool;

        Ok(())
    }

    fn gen_class_or_interface_access_flags(
        &mut self,
        access_flags: &[PhoronClassOrInterfaceAccessFlag],
    ) -> CodegenResult<()> {
        let class_or_interface_af_to_val = |af| match af {
            &PhoronClassOrInterfaceAccessFlag::AccPublic => ACC_PUBLIC,
            &PhoronClassOrInterfaceAccessFlag::AccFinal => ACC_FINAL,
            &PhoronClassOrInterfaceAccessFlag::AccSuper => ACC_SUPER,
            &PhoronClassOrInterfaceAccessFlag::AccInterface => ACC_INTERFACE,
            &PhoronClassOrInterfaceAccessFlag::AccAbstract => ACC_ABSTRACT,
            &PhoronClassOrInterfaceAccessFlag::AccSynthetic => ACC_SYNTHETIC,
            &PhoronClassOrInterfaceAccessFlag::AccAnnotation => ACC_ANNOTATION,
            &PhoronClassOrInterfaceAccessFlag::AccEnum => ACC_ENUM,
            &PhoronClassOrInterfaceAccessFlag::AccModule => ACC_MODULE,
        };

        self.classfile.access_flags = access_flags
            .iter()
            .fold(0u16, |acc, af| acc | class_or_interface_af_to_val(af));
        Ok(())
    }

    fn gen_classfile_headers(&mut self) -> CodegenResult<()> {
        self.classfile.magic = MAGIC;
        self.classfile.minor_version = MINOR_VERSION;
        self.classfile.major_version = MAJOR_VERSION;

        Ok(())
    }

    pub fn gen_bytecode(
        &mut self,
        program: &PhoronProgram,
        cp: &PhoronConstantPool,
    ) -> CodegenResult<()> {
        self.gen_classfile_headers()?;
        self.gen_constant_pool(&cp)?;
        self.visit_program(&program, &cp)?;

        println!("classfile = {:#?}", self.classfile);
        self.outfile.serialize(&self.classfile)?;

        Ok(())
    }
}

impl<'a, 'c, W> PhoronAstVisitor<'a> for Codegen<'c, W>
where
    W: Write,
{
    type Input = &'a PhoronConstantPool;
    type Result = CodegenResult<()>;

    fn visit_program(&mut self, program: &PhoronProgram, cp: Self::Input) -> Self::Result {
        self.visit_header(&program.header, &cp)?;
        self.visit_body(&program.body, &cp)?;

        Ok(())
    }

    fn visit_header(&mut self, header: &PhoronHeader, cp: Self::Input) -> Self::Result {
        if let Some(sourcefile_def) = &header.sourcefile_def {
            self.visit_sourcefile_def(sourcefile_def, &cp)?;
        }

        match header.class_or_interface_def {
            PhoronClassOrInterface::Class(ref class_def) => self.visit_class_def(class_def, &cp)?,
            PhoronClassOrInterface::Interface(ref interface_def) => {
                self.visit_interface_def(interface_def, &cp)?
            }
        }
        self.visit_super_def(&header.super_def, &cp)?;

        Ok(())
    }

    fn visit_sourcefile_def(
        &mut self,
        sourcefile_def: &PhoronSourceFileDef,
        input: Self::Input,
    ) -> Self::Result {
        todo!()
    }

    //pub struct ClassFile {
    //    pub interfaces_count: u16,
    //    pub interfaces: Vec<u16>,
    //    pub fields_count: u16,
    //    pub fields: Vec<FieldInfo>,
    //    pub methods_count: u16,
    //    pub methods: Vec<MethodInfo>,
    //    pub attributes_count: u16,
    //    pub attributes: Vec<AttributeInfo>,
    //}

    fn visit_class_def(&mut self, class_def: &PhoronClassDef, cp: Self::Input) -> Self::Result {
        self.gen_class_or_interface_access_flags(&class_def.access_flags)?;

        self.classfile.this_class =
            *cp.get_class(&class_def.name).ok_or(CodegenError::Missing {
                component: "`this` class",
            })?;

        Ok(())
    }

    fn visit_interface_def(
        &mut self,
        class_def: &PhoronInterfaceDef,
        input: Self::Input,
    ) -> Self::Result {
        todo!()
    }

    fn visit_super_def(&mut self, super_def: &PhoronSuperDef, cp: Self::Input) -> Self::Result {
        self.classfile.super_class =
            *cp.get_class(&super_def.super_class_name)
                .ok_or(CodegenError::Missing {
                    component: "`super` class",
                })?;

        Ok(())
    }

    fn visit_body(&mut self, body: &PhoronBody, cp: Self::Input) -> Self::Result {
        self.classfile.fields_count = body.field_defs.len() as u16;
        body.field_defs
            .iter()
            .try_for_each(|field| self.visit_field_def(&field, cp))?;

        self.classfile.methods_count = body.method_defs.len() as u16;
        body.method_defs
            .iter()
            .try_for_each(|method| self.visit_method_def(&method, cp))?;

        Ok(())
    }

    fn visit_field_def(&mut self, field_def: &PhoronFieldDef, cp: Self::Input) -> Self::Result {
        todo!()
    }

    fn visit_method_def(
        &mut self,
        method_def: &PhoronMethodDef,
        input: Self::Input,
    ) -> Self::Result {
        todo!()
    }

    fn visit_directive(&mut self, directive: &PhoronDirective, cp: Self::Input) -> Self::Result {
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
