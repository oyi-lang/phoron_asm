use crate::{ast::*, cp_analyzer::*};
use phoron_core::{
    error::SerializeError,
    model::{
        access_flags::*,
        attributes::AttributeInfo,
        constant_pool::{tags::*, types::CpInfo},
        *,
    },
    rw::writer::Writer,
    serializer::Serializer,
};
use std::{error::Error, fmt, io::Write};

#[derive(Debug)]
pub enum CodegenError {
    Missing {
        component: &'static str,
    },
    OpcodeError {
        opcode: &'static str,
        details: &'static str,
    },
    Unknown,
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
                OpcodeError {
                    ref opcode,
                    ref details,
                } => format!("Malformed or Invalid opcode {opcode} : {details}"),
                Unknown => "an unknown error occurred during code generation".into(),
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

impl PhoronClassOrInterfaceAccessFlag {
    fn to_u16(&self) -> u16 {
        match self {
            PhoronClassOrInterfaceAccessFlag::AccPublic => ACC_PUBLIC,
            PhoronClassOrInterfaceAccessFlag::AccFinal => ACC_FINAL,
            PhoronClassOrInterfaceAccessFlag::AccSuper => ACC_SUPER,
            PhoronClassOrInterfaceAccessFlag::AccInterface => ACC_INTERFACE,
            PhoronClassOrInterfaceAccessFlag::AccAbstract => ACC_ABSTRACT,
            PhoronClassOrInterfaceAccessFlag::AccSynthetic => ACC_SYNTHETIC,
            PhoronClassOrInterfaceAccessFlag::AccAnnotation => ACC_ANNOTATION,
            PhoronClassOrInterfaceAccessFlag::AccEnum => ACC_ENUM,
            PhoronClassOrInterfaceAccessFlag::AccModule => ACC_MODULE,
        }
    }
}
impl PhoronFieldAccessFlag {
    fn to_u16(&self) -> u16 {
        match self {
            PhoronFieldAccessFlag::AccPublic => ACC_PUBLIC,
            PhoronFieldAccessFlag::AccPrivate => ACC_PRIVATE,
            PhoronFieldAccessFlag::AccProtected => ACC_PROTECTED,
            PhoronFieldAccessFlag::AccStatic => ACC_STATIC,
            PhoronFieldAccessFlag::AccFinal => ACC_FINAL,
            PhoronFieldAccessFlag::AccVolatile => ACC_VOLATILE,
            PhoronFieldAccessFlag::AccTransient => ACC_TRANSIENT,
            PhoronFieldAccessFlag::AccSynthetic => ACC_SYNTHETIC,
            PhoronFieldAccessFlag::AccEnum => ACC_ENUM,
        }
    }
}

impl PhoronMethodAccessFlag {
    fn to_u16(&self) -> u16 {
        match self {
            PhoronMethodAccessFlag::AccPublic => ACC_PUBLIC,
            PhoronMethodAccessFlag::AccPrivate => ACC_PRIVATE,
            PhoronMethodAccessFlag::AccProtected => ACC_PROTECTED,
            PhoronMethodAccessFlag::AccStatic => ACC_STATIC,
            PhoronMethodAccessFlag::AccFinal => ACC_FINAL,
            PhoronMethodAccessFlag::AccSynchronized => ACC_SYNCHRONIZED,
            PhoronMethodAccessFlag::AccBridge => ACC_BRIDGE,
            PhoronMethodAccessFlag::AccVarargs => ACC_VARARGS,
            PhoronMethodAccessFlag::AccNative => ACC_NATIVE,
            PhoronMethodAccessFlag::AccAbstract => ACC_ABSTRACT,
            PhoronMethodAccessFlag::AccStrict => ACC_STRICT,
            PhoronMethodAccessFlag::AccSynthetic => ACC_SYNTHETIC,
        }
    }
}

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

        let mut constant_pool = vec![None; constant_pool_count + 1];

        for (cp_key, cp_idx) in cp.iter() {
            let cp_idx = *cp_idx as usize;

            match cp_key {
                PhoronConstantPoolKind::Class { ref name_index } => {
                    constant_pool[cp_idx] = Some(CpInfo::ConstantClassInfo {
                        tag: CONSTANT_CLASS,
                        name_index: *name_index,
                    })
                }

                PhoronConstantPoolKind::Fieldref {
                    ref class_index,
                    ref name_and_type_index,
                } => {
                    constant_pool[cp_idx] = Some(CpInfo::ConstantFieldrefInfo {
                        tag: CONSTANT_FIELD_REF,
                        class_index: *class_index,
                        name_and_type_index: *name_and_type_index,
                    })
                }

                PhoronConstantPoolKind::Methodref {
                    ref class_index,
                    ref name_and_type_index,
                } => {
                    constant_pool[cp_idx] = Some(CpInfo::ConstantMethodrefInfo {
                        tag: CONSTANT_METHOD_REF,
                        class_index: *class_index,
                        name_and_type_index: *name_and_type_index,
                    })
                }

                PhoronConstantPoolKind::InterfaceMethodref {
                    ref class_index,
                    ref name_and_type_index,
                } => {
                    constant_pool[cp_idx] = Some(CpInfo::ConstantInterfaceMethodrefInfo {
                        tag: CONSTANT_INTERFACE_METHOD_REF,
                        class_index: *class_index,
                        name_and_type_index: *name_and_type_index,
                    })
                }

                PhoronConstantPoolKind::String { ref string_index } => {
                    constant_pool[cp_idx] = Some(CpInfo::ConstantStringInfo {
                        tag: CONSTANT_STRING,
                        string_index: *string_index,
                    })
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
                } => {
                    constant_pool[cp_idx] = Some(CpInfo::ConstantNameAndTypeInfo {
                        tag: CONSTANT_NAME_AND_TYPE,
                        name_index: *name_index,
                        descriptor_index: *descriptor_index,
                    })
                }

                PhoronConstantPoolKind::Utf8(ref s) => {
                    constant_pool[cp_idx] = Some(CpInfo::ConstantUtf8Info {
                        tag: CONSTANT_UTF8,
                        length: s.len() as u16,
                        bytes: s.bytes().collect::<_>(),
                    })
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
        self.classfile.access_flags = access_flags.iter().fold(0u16, |acc, af| acc | af.to_u16());
        Ok(())
    }

    fn gen_field_access_flags(
        &mut self,
        field_info: &mut FieldInfo,
        access_flags: &[PhoronFieldAccessFlag],
    ) -> CodegenResult<()> {
        field_info.access_flags = access_flags.iter().fold(0u16, |acc, af| acc | af.to_u16());
        Ok(())
    }

    fn gen_method_access_flags(
        &mut self,
        method_info: &mut MethodInfo,
        access_flags: &[PhoronMethodAccessFlag],
    ) -> CodegenResult<()> {
        method_info.access_flags = access_flags.iter().fold(0u16, |acc, af| acc | af.to_u16());
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

        self.outfile.serialize(&self.classfile)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum CodegenResultType {
    ByteVec(Vec<u8>),
    Empty,
}

impl<'a, 'c, W> PhoronAstVisitor<'a> for Codegen<'c, W>
where
    W: Write,
{
    type Input = &'a PhoronConstantPool;
    type Result = CodegenResult<CodegenResultType>;

    fn visit_program(&mut self, program: &PhoronProgram, cp: Self::Input) -> Self::Result {
        self.visit_header(&program.header, &cp)?;
        self.visit_body(&program.body, &cp)?;

        Ok(CodegenResultType::Empty)
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
        };
        self.visit_super_def(&header.super_def, &cp)?;

        Ok(CodegenResultType::Empty)
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

        Ok(CodegenResultType::Empty)
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

        Ok(CodegenResultType::Empty)
    }

    fn visit_body(&mut self, body: &PhoronBody, cp: Self::Input) -> Self::Result {
        self.classfile.fields_count = body.field_defs.len() as u16;

        for field in &body.field_defs {
            self.visit_field_def(field, cp)?;
        }

        self.classfile.methods_count = body.method_defs.len() as u16;
        for method in &body.method_defs {
            self.visit_method_def(method, cp)?;
        }

        println!("classfile = {:#?}", self.classfile);

        Ok(CodegenResultType::Empty)
    }

    /// Generate JVM bytecode for the field definition
    fn visit_field_def(&mut self, field_def: &PhoronFieldDef, cp: Self::Input) -> Self::Result {
        todo!()
    }

    //pub access_flags: u16,
    //pub name_index: u16,
    //pub descriptor_index: u16,
    //pub attributes_count: u16,
    //pub attributes: Vec<AttributeInfo>,

    //pub name: String,
    //pub access_flags: Vec<PhoronMethodAccessFlag>,
    //pub method_descriptor: PhoronMethodDescriptor,
    //pub instructions: Vec<PhoronInstruction>,

    /// Generate JVM bytecode for the method definition
    fn visit_method_def(&mut self, method_def: &PhoronMethodDef, cp: Self::Input) -> Self::Result {
        let mut method_info = MethodInfo::default();

        self.gen_method_access_flags(&mut method_info, &method_def.access_flags)?;

        method_info.name_index = *cp.get_name(&method_def.name).ok_or(CodegenError::Missing {
            component: "method name",
        })?;

        method_info.descriptor_index = *cp
            .get_name(&method_def.method_descriptor.to_string())
            .ok_or(CodegenError::Missing {
                component: "method descriptor",
            })?;

        // todo - attributes

        //Code {
        //    attribute_name_index: u16,
        //    attribute_length: u32,
        //    max_stack: u16,
        //    max_locals: u16,
        //    code_length: u32,
        //    code: Vec<u8>,
        //    exception_table_length: u16,
        //    exception_table: Vec<ExceptionHandler>,
        //    code_attributes_count: u16,
        //    code_attributes: Vec<AttributeInfo>,
        //},

        method_info.attributes_count = 1;
        //let mut attrs = Vec::new();

        let attribute_name_index = *cp.get_name("Code").ok_or(CodegenError::Missing {
            component: "`Code` attribute",
        })?;

        let attribute_length = 0;
        let mut max_stack = 0;
        let mut max_locals = 0;
        let mut code = Vec::new();
        let exception_table_length = 0;
        let exception_table = vec![];
        let code_attributes_count = 0;
        let code_attributes = vec![];

        for instr in &method_def.instructions {
            match instr {
                PhoronInstruction::PhoronDirective(ref dir) => match dir {
                    PhoronDirective::LimitStack(stack) => {
                        max_stack = *stack;
                    }

                    PhoronDirective::LimitLocals(locals) => {
                        max_locals = *locals;
                    }
                    _ => unreachable!(),
                },

                PhoronInstruction::PhoronLabel(ref label) => {}

                PhoronInstruction::JvmInstruction(ref jvm_instr) => {
                    if let Ok(CodegenResultType::ByteVec(instr_opcodes)) =
                        self.visit_jvm_instruction(jvm_instr, cp)
                    {
                        code.extend_from_slice(&instr_opcodes);
                    } else {
                        return Err(CodegenError::Unknown);
                    }
                }
            }
        }

        let code_length = code.len() as u32;

        method_info.attributes.push(AttributeInfo::Code {
            attribute_name_index,
            attribute_length,
            max_stack,
            max_locals,
            code_length,
            code,
            exception_table_length,
            exception_table,
            code_attributes_count,
            code_attributes,
        });

        self.classfile.methods.push(method_info);

        Ok(CodegenResultType::Empty)
    }

    /// Generate JVM bytecode for Phoron directive
    fn visit_directive(&mut self, directive: &PhoronDirective, cp: Self::Input) -> Self::Result {
        Ok(CodegenResultType::Empty)
    }

    /// Generate JVM bytecode for JVM instruction
    fn visit_jvm_instruction(
        &mut self,
        jvm_instr: &JvmInstruction,
        cp: Self::Input,
    ) -> Self::Result {
        use JvmInstruction::*;

        Ok(match jvm_instr {
            Aaload => {
                todo!()
            }
            Anewarray { ref component_type } => {
                todo!()
            }
            Areturn => {
                todo!()
            }
            Aastore => {
                todo!()
            }
            Aconstnull => {
                todo!()
            }

            Aload0 => CodegenResultType::ByteVec(vec![0x2a]),
            Aload1 => {
                todo!()
            }
            Aload2 => {
                todo!()
            }
            Aload3 => {
                todo!()
            }
            Aload { ref varnum } => {
                todo!()
            }
            Arraylength => {
                todo!()
            }
            Astore0 => {
                todo!()
            }

            Astore1 => {
                todo!()
            }
            Astore2 => {
                todo!()
            }
            Astore3 => {
                todo!()
            }
            Astore { ref varnum } => {
                todo!()
            }
            Athrow => {
                todo!()
            }
            Baload => {
                todo!()
            }
            Bastore => {
                todo!()
            }

            Bipush(sb) => {
                todo!()
            }
            Caload => {
                todo!()
            }
            Castore => {
                todo!()
            }
            Checkcast { ref cast_type } => {
                todo!()
            }
            Dadd => {
                todo!()
            }
            Daload => {
                todo!()
            }
            Dastore => {
                todo!()
            }
            Dcmpg => {
                todo!()
            }
            Dcmpl => {
                todo!()
            }
            Dconst0 => {
                todo!()
            }
            Dconst1 => {
                todo!()
            }
            Ddiv => {
                todo!()
            }
            D2f => {
                todo!()
            }
            D2i => {
                todo!()
            }
            D2l => {
                todo!()
            }
            Dload0 => {
                todo!()
            }
            Dload1 => {
                todo!()
            }
            Dload2 => {
                todo!()
            }
            Dload3 => {
                todo!()
            }
            Dload { ref varnum } => {
                todo!()
            }
            Dmul => {
                todo!()
            }
            Dneg => {
                todo!()
            }
            Drem => {
                todo!()
            }
            Dreturn => {
                todo!()
            }
            Dstore0 => {
                todo!()
            }
            Dstore1 => {
                todo!()
            }
            Dstore2 => {
                todo!()
            }
            Dstore3 => {
                todo!()
            }
            Dstore { ref varnum } => {
                todo!()
            }
            Dsub => {
                todo!()
            }
            Dup2x1 => {
                todo!()
            }
            Dup2x2 => {
                todo!()
            }
            Dup2 => {
                todo!()
            }
            Dupx1 => {
                todo!()
            }
            Dupx2 => {
                todo!()
            }
            Dup => {
                todo!()
            }
            F2d => {
                todo!()
            }
            F2i => {
                todo!()
            }
            F2l => {
                todo!()
            }
            Fadd => {
                todo!()
            }
            Faload => {
                todo!()
            }
            Fastore => {
                todo!()
            }
            Fcmpg => {
                todo!()
            }
            Fcmpl => {
                todo!()
            }
            Fconst0 => {
                todo!()
            }
            Fconst1 => {
                todo!()
            }
            Fconst2 => {
                todo!()
            }
            Fdiv => {
                todo!()
            }
            Fload0 => {
                todo!()
            }
            Fload1 => {
                todo!()
            }
            Fload2 => {
                todo!()
            }
            Fload3 => {
                todo!()
            }
            Fload { ref varnum } => {
                todo!()
            }
            Fmul => {
                todo!()
            }
            Fneg => {
                todo!()
            }
            Frem => {
                todo!()
            }
            Freturn => {
                todo!()
            }
            Fstore0 => {
                todo!()
            }
            Fstore1 => {
                todo!()
            }
            Fstore2 => {
                todo!()
            }
            Fstore3 => {
                todo!()
            }
            Fstore { ref varnum } => {
                todo!()
            }
            Fsub => {
                todo!()
            }

            Getstatic {
                ref class_name,
                ref field_name,
                ref field_descriptor,
            } => {
                let mut opcodes = Vec::new();
                opcodes.push(0xb2);

                let fieldref = *cp
                    .get_fieldref(class_name, field_name, &field_descriptor.to_string())
                    .ok_or(CodegenError::OpcodeError {
                        opcode: "getstatic",
                        details: "missing field reference",
                    })?;
                opcodes.extend_from_slice(&fieldref.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Getfield {
                ref class_name,
                ref field_name,
                ref field_descriptor,
            } => {
                todo!()
            }
            Goto { ref label } => {
                todo!()
            }
            Gotow { ref label } => {
                todo!()
            }
            I2b => {
                todo!()
            }
            I2c => {
                todo!()
            }
            I2d => {
                todo!()
            }
            I2f => {
                todo!()
            }
            I2l => {
                todo!()
            }
            I2s => {
                todo!()
            }
            Iadd => {
                todo!()
            }
            Iaload => {
                todo!()
            }
            Iand => {
                todo!()
            }
            Iastore => {
                todo!()
            }
            Iconstm1 => {
                todo!()
            }
            Iconst0 => {
                todo!()
            }
            Iconst1 => {
                todo!()
            }
            Iconst2 => {
                todo!()
            }
            Iconst3 => {
                todo!()
            }
            Iconst4 => {
                todo!()
            }
            Iconst5 => {
                todo!()
            }
            Idiv => {
                todo!()
            }

            Ifacmpeq { ref label } => {
                todo!()
            }
            Ifacmpne { ref label } => {
                todo!()
            }
            Ificmpeq { ref label } => {
                todo!()
            }
            Ificmpge { ref label } => {
                todo!()
            }
            Ificmpgt { ref label } => {
                todo!()
            }
            Ificmple { ref label } => {
                todo!()
            }
            Ificmplt { ref label } => {
                todo!()
            }
            Ificmpne { ref label } => {
                todo!()
            }
            Ifeq { ref label } => {
                todo!()
            }
            Ifge { ref label } => {
                todo!()
            }
            Ifgt { ref label } => {
                todo!()
            }
            Ifle { ref label } => {
                todo!()
            }
            Iflt { ref label } => {
                todo!()
            }
            Ifne { ref label } => {
                todo!()
            }
            Ifnonnull { ref label } => {
                todo!()
            }
            Ifnull { ref label } => {
                todo!()
            }
            Iinc {
                ref varnum,
                ref delta,
            } => {
                todo!()
            }
            Iload0 => {
                todo!()
            }
            Iload1 => {
                todo!()
            }
            Iload2 => {
                todo!()
            }
            Iload3 => {
                todo!()
            }
            Iload { ref varnum } => {
                todo!()
            }
            Imul => {
                todo!()
            }
            Ineg => {
                todo!()
            }
            Instanceof { ref check_type } => {
                todo!()
            }
            Invokeinterface {
                ref interface_name,
                ref method_name,
                ref method_descriptor,
                ref ub,
            } => {
                todo!()
            }

            Invokespecial {
                ref class_name,
                ref method_name,
                ref method_descriptor,
            } => {
                let mut opcodes = Vec::new();
                opcodes.push(0xb7);

                let methodref_index = *cp
                    .get_methodref(class_name, method_name, &method_descriptor.to_string())
                    .ok_or(CodegenError::OpcodeError {
                        opcode: "invokespecial",
                        details: "missing method reference",
                    })?;
                opcodes.extend_from_slice(&methodref_index.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Invokestatic {
                ref class_name,
                ref method_name,
                ref method_descriptor,
            } => {
                todo!()
            }

            Invokevirtual {
                ref class_name,
                ref method_name,
                ref method_descriptor,
            } => {
                let mut opcodes = Vec::new();
                opcodes.push(0xb6);

                let methodref_index = *cp
                    .get_methodref(class_name, method_name, &method_descriptor.to_string())
                    .ok_or(CodegenError::OpcodeError {
                        opcode: "invokevirtual",
                        details: "missing method reference",
                    })?;

                opcodes.extend_from_slice(&methodref_index.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ior => {
                todo!()
            }
            Irem => {
                todo!()
            }
            Ireturn => {
                todo!()
            }
            Ishl => {
                todo!()
            }
            Ishr => {
                todo!()
            }
            Istore0 => {
                todo!()
            }
            Istore1 => {
                todo!()
            }
            Istore2 => {
                todo!()
            }
            Istore3 => {
                todo!()
            }
            Istore { ref varnum } => {
                todo!()
            }
            Isub => {
                todo!()
            }
            Iushr => {
                todo!()
            }
            Ixor => {
                todo!()
            }
            Jsrw { ref label } => {
                todo!()
            }
            Jsr { ref label } => {
                todo!()
            }
            L2d => {
                todo!()
            }
            L2f => {
                todo!()
            }
            L2i => {
                todo!()
            }
            Ladd => {
                todo!()
            }
            Laload => {
                todo!()
            }
            Land => {
                todo!()
            }
            Lastore => {
                todo!()
            }
            Lcmp => {
                todo!()
            }
            Lconst0 => {
                todo!()
            }
            Lconst1 => {
                todo!()
            }
            Ldc2w(ref ldc2w_val) => {
                todo!()
            }
            Ldcw(ref ldcw_val) => {
                todo!()
            }

            Ldc(ref ldc_val) => {
                let mut opcodes = Vec::new();
                opcodes.push(0x12);

                match ldc_val {
                    LdcValue::QuotedString(ref string) => {
                        let string_index =
                            *cp.get_string(string).ok_or(CodegenError::OpcodeError {
                                opcode: "ldc",
                                details: "missing quoted string",
                            })?;

                        opcodes.extend_from_slice(&string_index.to_be_bytes())
                    }
                    _ => unreachable!(),
                }

                CodegenResultType::ByteVec(opcodes)
            }

            Ldiv => {
                todo!()
            }
            Lload { ref varnum } => {
                todo!()
            }
            Lload0 => {
                todo!()
            }
            Lload1 => {
                todo!()
            }
            Lload2 => {
                todo!()
            }
            Lload3 => {
                todo!()
            }
            Lmul => {
                todo!()
            }
            Lneg => {
                todo!()
            }
            Lookupswitch {
                ref switches,
                ref default,
            } => {
                todo!()
            }
            Lor => {
                todo!()
            }
            Lrem => {
                todo!()
            }
            Lreturn => {
                todo!()
            }
            Lshl => {
                todo!()
            }
            Lshr => {
                todo!()
            }
            Lstore { ref varnum } => {
                todo!()
            }
            Lstore0 => {
                todo!()
            }
            Lstore1 => {
                todo!()
            }
            Lstore2 => {
                todo!()
            }
            Lstore3 => {
                todo!()
            }
            Lsub => {
                todo!()
            }
            Lushr => {
                todo!()
            }
            Lxor => {
                todo!()
            }
            Monitorenter => {
                todo!()
            }
            Monitorexit => {
                todo!()
            }
            Multianewarray {
                ref component_type,
                ref dimensions,
            } => {
                todo!()
            }
            Newarray { ref component_type } => {
                todo!()
            }
            New { ref class_name } => todo!(),
            Nop => {
                todo!()
            }
            Pop2 => {
                todo!()
            }
            Pop => {
                todo!()
            }
            Putfield {
                ref class_name,
                ref field_name,
                ref field_descriptor,
            } => {
                todo!()
            }
            Putstatic {
                ref class_name,
                ref field_name,
                ref field_descriptor,
            } => {
                todo!()
            }

            Return => CodegenResultType::ByteVec(vec![0xb1]),

            Ret { ref varnum } => todo!(),
            Saload => {
                todo!()
            }
            Sastore => {
                todo!()
            }
            Sipush(ref ss) => todo!(),
            Swap => {
                todo!()
            }
            Tableswitch {
                ref low,
                ref high,
                ref switches,
                ref default,
            } => {
                todo!()
            }
            Wide => {
                todo!()
            }
        })
    }
}
