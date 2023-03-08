use crate::{
    ast::*,
    attributes::*,
    cp_analyzer::{constant_pool::*, *},
};
use phoron_core::{
    error::SerializeError,
    model::{
        access_flags::*,
        attributes::{AttributeInfo, *},
        constant_pool::{tags::*, types::CpInfo},
        *,
    },
    rw::writer::Writer,
    serializer::Serializer,
};

use std::{collections::HashMap, error::Error, fmt, io::Write};

#[derive(Debug)]
pub enum CodegenError {
    AttributeError {
        attr: &'static str,
        details: &'static str,
    },

    Missing {
        component: &'static str,
    },
    ConstantPoolError {
        cp_entry: &'static str,
        details: &'static str,
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
                AttributeError {
                    ref attr,
                    ref details,
                } => format!("{attr}: {details}"),
                Missing { ref component } => format!("Missing : {component}"),
                ConstantPoolError {
                    ref cp_entry,
                    ref details,
                } => format!("{cp_entry} : {details}"),
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
    label_mapping: HashMap<String, i16>,
    curr_code_offset: i16,
}

impl<'c, W> Codegen<'c, W>
where
    W: Write,
{
    pub fn new(outfile: &'c mut W) -> Self {
        Codegen {
            outfile: Serializer::new(Writer::new(outfile)),
            classfile: ClassFile::default(),
            label_mapping: HashMap::new(),
            curr_code_offset: 0,
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

        self.classfile.constant_pool_count = constant_pool_count as u16 + 1;

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
                    constant_pool[cp_idx] = Some(CpInfo::ConstantIntegerInfo {
                        tag: CONSTANT_INTEGER,
                        bytes: u32::from_be_bytes(*int_bytes),
                    })
                }

                PhoronConstantPoolKind::Float(float_bytes) => {
                    constant_pool[cp_idx] = Some(CpInfo::ConstantFloatInfo {
                        tag: CONSTANT_FLOAT,
                        bytes: u32::from_be_bytes(*float_bytes),
                    })
                }

                PhoronConstantPoolKind::Long(long_bytes) => {
                    let high_bytes: [u8; 4] = long_bytes[0..4].try_into().map_err(|_| {
                        CodegenError::ConstantPoolError {
                            cp_entry: "long",
                            details: "failed to get high bytes",
                        }
                    })?;

                    let low_bytes: [u8; 4] = long_bytes[4..].try_into().map_err(|_| {
                        CodegenError::ConstantPoolError {
                            cp_entry: "long",
                            details: "failed to get low bytes",
                        }
                    })?;

                    constant_pool[cp_idx] = Some(CpInfo::ConstantLongInfo {
                        tag: CONSTANT_LONG,
                        high_bytes: u32::from_be_bytes(high_bytes),
                        low_bytes: u32::from_be_bytes(low_bytes),
                    })
                }

                PhoronConstantPoolKind::Double(double_bytes) => {
                    let high_bytes: [u8; 4] = double_bytes[0..4].try_into().map_err(|_| {
                        CodegenError::ConstantPoolError {
                            cp_entry: "double",
                            details: "failed to get high bytes",
                        }
                    })?;

                    let low_bytes: [u8; 4] = double_bytes[4..].try_into().map_err(|_| {
                        CodegenError::ConstantPoolError {
                            cp_entry: "double",
                            details: " failed to get low bytes",
                        }
                    })?;

                    constant_pool[cp_idx] = Some(CpInfo::ConstantDoubleInfo {
                        tag: CONSTANT_DOUBLE,
                        high_bytes: u32::from_be_bytes(high_bytes),
                        low_bytes: u32::from_be_bytes(low_bytes),
                    })
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

    fn gen_wide_offset_for_label(&self, label_offset: i16) -> [u8; 4] {
        let label_offset = label_offset as i32;

        todo!()
    }

    fn gen_label_mappings(&mut self, instructions: &[PhoronInstruction]) -> CodegenResult<()> {
        use JvmInstruction::*;

        self.label_mapping.clear();
        let mut curr_code_offset = 0i16;

        for instr in instructions {
            match instr {
                PhoronInstruction::PhoronDirective(ref directive) => {}

                PhoronInstruction::PhoronLabel(ref label) => {
                    self.label_mapping
                        .insert(label.to_string(), curr_code_offset);
                }

                PhoronInstruction::JvmInstruction(ref jvm_instr) => {
                    curr_code_offset += match jvm_instr {
                        // 1-byte instructions
                        Aaload | Aastore | Aconstnull | Aload0 | Aload1 | Aload2 | Aload3
                        | Areturn | Arraylength | Astore0 | Astore1 | Astore2 | Astore3
                        | Athrow | Baload | Bastore | Caload | Castore | D2f | D2i | D2l | Dadd
                        | Daload | Dastore | Dcmpg | Dcmpl | Dconst0 | Dconst1 | Ddiv | Dload0
                        | Dload1 | Dload2 | Dload3 | Dmul | Dneg | Drem | Dreturn | Dstore0
                        | Dstore1 | Dstore2 | Dstore3 | Dsub | Dup | Dupx1 | Dupx2 | Dup2
                        | Dup2x1 | Dup2x2 | F2d | F2i | F2l | Fadd | Faload | Fastore | Fcmpg
                        | Fcmpl | Fconst0 | Fconst1 | Fconst2 | Fdiv | Fload0 | Fload1 | Fload2
                        | Fload3 | Fmul | Fneg | Frem | Freturn | Fstore0 | Fstore1 | Fstore2
                        | Fstore3 | Fsub | I2b | I2c | I2d | I2f | I2l | I2s | Iadd | Iaload
                        | Iand | Iastore | Iconstm1 | Iconst0 | Iconst1 | Iconst2 | Iconst3
                        | Iconst4 | Iconst5 | Idiv | Iload0 | Iload1 | Iload2 | Iload3 | Imul
                        | Ineg | Ior | Irem | Ireturn | Ishl | Ishr | Istore0 | Istore1
                        | Istore2 | Istore3 | Isub | Iushr | Ixor | L2d | L2f | L2i | Ladd
                        | Laload | Land | Lastore | Lcmp | Lconst0 | Lconst1 | Ldiv | Lload0
                        | Lload1 | Lload2 | Lload3 | Lmul | Lneg | Lor | Lrem | Lreturn | Lshl
                        | Lshr | Lstore0 | Lstore1 | Lstore2 | Lstore3 | Lsub | Lushr | Lxor
                        | Monitorenter | Monitorexit | Nop | Pop | Pop2 | Return | Saload
                        | Sastore | Swap => 1,

                        Bipush(..)
                        | Newarray { .. }
                        | Iload { .. }
                        | Fload { .. }
                        | Aload { .. }
                        | Lload { .. }
                        | Dload { .. }
                        | Istore { .. }
                        | Fstore { .. }
                        | Astore { .. }
                        | Lstore { .. }
                        | Dstore { .. }
                        | Ldc(..)
                        | Ret { .. } => 2,

                        Sipush(..)
                        | Anewarray { .. }
                        | Checkcast { .. }
                        | Getstatic { .. }
                        | Getfield { .. }
                        | Iinc { .. }
                        | Invokespecial { .. }
                        | Invokestatic { .. }
                        | Invokevirtual { .. }
                        | Putfield { .. }
                        | Putstatic { .. }
                        | Goto { .. }
                        | Ifacmpeq { .. }
                        | Ifacmpne { .. }
                        | Ificmpeq { .. }
                        | Ificmpge { .. }
                        | Ificmpgt { .. }
                        | Ificmple { .. }
                        | Ificmplt { .. }
                        | Ificmpne { .. }
                        | Ifeq { .. }
                        | Ifge { .. }
                        | Ifgt { .. }
                        | Ifle { .. }
                        | Iflt { .. }
                        | Ifne { .. }
                        | Ifnonnull { .. }
                        | Ifnull { .. }
                        | Instanceof { .. }
                        | Ldcw(..)
                        | Ldc2w(..)
                        | New { .. }
                        | Jsr { .. } => 3,

                        Invokeinterface { .. } | Multianewarray { .. } => 4,

                        Jsrw { .. } | Gotow { .. } => 5,

                        Lookupswitch {
                            ref switches,
                            ref default,
                        } => {
                            let mut opcode_len = 1i16; // for the opcode
                            opcode_len +=
                                switches.len() as i16 * 2i16 * std::mem::size_of::<u32>() as i16; // for the switch pairs

                            let default_offset_padding = (self.curr_code_offset + opcode_len) % 4;
                            opcode_len += default_offset_padding;
                            opcode_len += 2i16 * std::mem::size_of::<u32>() as i16; // for the default pair

                            opcode_len
                        }

                        Tableswitch {
                            ref low,
                            ref high,
                            ref switches,
                            ref default,
                        } => {
                            let mut opcode_len = 1i16; // for the opcode

                            // for `low` and `high`
                            opcode_len += 2i16 * std::mem::size_of::<i32>() as i16;
                            // for the switch offsets
                            opcode_len +=
                                (high - low + 1) as i16 * std::mem::size_of::<i32>() as i16;

                            let default_offset_padding = (self.curr_code_offset + opcode_len) % 4;
                            opcode_len += default_offset_padding;
                            // for the default pair
                            opcode_len += 2i16 & std::mem::size_of::<i32>() as i16;

                            opcode_len
                        }

                        Wide(ref wide_instr) => match wide_instr {
                            WideInstruction::IInc { .. } => 6,
                            _ => 4,
                        },
                    } as i16;
                }
            }
        }

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

        self.visit_program(&program, cp)?;
        println!("classfile = {:#?}", self.classfile);
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
        self.visit_header(&program.header, cp)?;
        self.visit_body(&program.body, cp)?;

        Ok(CodegenResultType::Empty)
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
        };
        self.visit_super_def(&header.super_def, cp)?;

        Ok(CodegenResultType::Empty)
    }

    fn visit_sourcefile_def(
        &mut self,
        sourcefile_def: &PhoronSourceFileDef,
        cp: Self::Input,
    ) -> Self::Result {
        self.classfile.attributes_count += 1;

        let attribute_name_index =
            *cp.get_name(PHORON_SOURCE_FILE)
                .ok_or(CodegenError::Missing {
                    component: "`SourceFile` name attribute",
                })?;
        let attribute_length = 2; // as per the spec
        let sourcefile_index =
            *cp.get_name(&sourcefile_def.source_file)
                .ok_or(CodegenError::Missing {
                    component: "SourceFile class name",
                })?;
        self.classfile.attributes.push(AttributeInfo::SourceFile {
            attribute_name_index,
            attribute_length,
            sourcefile_index,
        });

        Ok(CodegenResultType::Empty)
    }

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
        cp: Self::Input,
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

        Ok(CodegenResultType::Empty)
    }

    //pub init_val: Option<PhoronFieldInitValue>,

    // pub attributes_count: u16,
    // pub attributes: Vec<AttributeInfo>,

    /// Generate JVM bytecode for the field definition
    fn visit_field_def(&mut self, field_def: &PhoronFieldDef, cp: Self::Input) -> Self::Result {
        let mut field_info = FieldInfo::default();

        self.gen_field_access_flags(&mut field_info, &field_def.access_flags)?;

        field_info.name_index = *cp.get_name(&field_def.name).ok_or(CodegenError::Missing {
            component: "field name",
        })?;

        field_info.descriptor_index =
            *cp.get_name(&field_def.field_descriptor.to_string())
                .ok_or(CodegenError::Missing {
                    component: "field descriptor",
                })?;

        // todo - attributes
        field_info.attributes_count = 0;

        self.classfile.fields.push(field_info);

        Ok(CodegenResultType::Empty)
    }

    //pub name: String,
    //pub access_flags: Vec<PhoronMethodAccessFlag>,
    //pub method_descriptor: PhoronMethodDescriptor,
    //pub instructions: Vec<PhoronInstruction>,

    /// Generate JVM bytecode for the method definition
    fn visit_method_def(&mut self, method_def: &PhoronMethodDef, cp: Self::Input) -> Self::Result {
        let mut method_info = MethodInfo::default();

        // todo
        self.gen_method_access_flags(&mut method_info, &method_def.access_flags)?;

        method_info.name_index = *cp.get_name(&method_def.name).ok_or(CodegenError::Missing {
            component: "method name",
        })?;

        method_info.descriptor_index = *cp
            .get_name(&method_def.method_descriptor.to_string())
            .ok_or(CodegenError::Missing {
                component: "method descriptor",
            })?;

        method_info.attributes_count = 1; // for the Code attribute

        // Visit all the JVM instructions first to collect metadata about all the
        // labels used in the Code section. This will be used to calculate the branch
        // offsets of the labels in the actual instructions in the next iteration
        // using the following algorithm:
        //
        // s: i16 = label_offset - curr_code_offset
        // b1: u8 = (s >> 8) as u8
        // b2: u8 = (s ^ (b1 as i16) << 8) as u8
        //
        // This also offsets the need to performa complicated "backpatching" of the
        // offsets after the Code vector/array has already beem generated (as would have
        // been the case in a more traditional compiler).
        self.gen_label_mappings(&method_def.instructions)?;

        // code attributes
        let attribute_name_index = *cp.get_name("Code").ok_or(CodegenError::Missing {
            component: "`Code` attribute name",
        })?;

        let mut code_attributes_length = 12; // default minimum (as per the spec)
        let mut max_stack = 1; // default
        let mut max_locals = 1; // default

        let mut code = Vec::new();

        let mut exception_table_length = 0;
        let mut exception_table = vec![];
        let mut code_attributes_count = 0;
        let mut code_attributes = vec![];

        self.curr_code_offset = 0;
        for instr in &method_def.instructions {
            match instr {
                PhoronInstruction::PhoronDirective(ref dir) => match dir {
                    PhoronDirective::LimitStack(stack) => {
                        max_stack = *stack;
                    }

                    PhoronDirective::LimitLocals(locals) => {
                        max_locals = *locals;
                    }

                    // this is a top-level attribute inside Methhodnfo, ot the same level as the
                    // Code attribute. Just like the Code attribute, there may be at most only one
                    // entry in the method attributes.
                    PhoronDirective::Throws { ref class_name } => {
                        // TODO - create test file for this attribute
                        //Exceptions {
                        //    attribute_name_index: u16,
                        //    attribute_length: u32,
                        //    number_of_exceptions: u16,
                        //    exception_index_table: Vec<u16>,
                        //},

                        //#[derive(Default, Debug)]
                        //pub struct LineNumber {
                        //    pub start_pc: u16,
                        //    pub line_number: u16,
                        //}

                        method_info.attributes_count += 1;
                        // TODO
                    }

                    // this goes in the `code_attributes` field of the `Code` attribute of the method
                    PhoronDirective::LineNumber(ref linum) => {
                        //LineNumberTable {
                        //    attribute_name_index: u16,
                        //    attribute_length: u32,
                        //    line_number_table_length: u16,
                        //    line_number_table: Vec<LineNumber>,
                        //},

                        // even though we can have multiple LineNumberTable attributes, we restrict
                        // ourselves to one LineNumberTable attribute per method, adding the line
                        // numbers for that method into the same entry, as in the case of the `.var` directive
                        let line_num_table_index = if code_attributes.is_empty() {
                            code_attributes_count += 1;
                            code_attributes_length += 8;

                            let attribute_name_index = *cp.get_name(PHORON_LINE_NUMBER_TABLE).ok_or(CodegenError::AttributeError {
                                attr: "Code",
                                details: "missing attribute name index for line number table in Code attribute",
                            })?;

                            let mut attribute_length = 2; // excluding the initial 5 bytes, as per the spec
                            let line_number_table_length = 0;
                            let line_number_table = Vec::new();

                            code_attributes.push(AttributeInfo::LineNumberTable {
                                attribute_name_index,
                                attribute_length,
                                line_number_table_length,
                                line_number_table,
                            });
                            0
                        } else {
                            let mut lntindex = 0;
                            for (index, attr) in code_attributes.iter().enumerate() {
                                if let AttributeInfo::LineNumberTable { .. } = attr {
                                    lntindex = index;
                                    break;
                                }
                            }
                            lntindex
                        };

                        let start_pc = self.curr_code_offset as u16;

                        if let AttributeInfo::LineNumberTable {
                            ref mut attribute_length,
                            ref mut line_number_table_length,
                            ref mut line_number_table,
                            ..
                        } = code_attributes[line_num_table_index]
                        {
                            *line_number_table_length += 1;

                            line_number_table.push(LineNumber {
                                start_pc,
                                line_number: *linum,
                            });

                            *attribute_length += 4;
                            code_attributes_length += 4;
                        }
                    }

                    // this goes in the `code_attributes` field of the `Code` attribute of the
                    // method
                    PhoronDirective::Var {
                        ref varnum,
                        ref name,
                        ref field_descriptor,
                        ref from_label,
                        ref to_label,
                    } => {
                        // there should be only one (at most) LocalVariableTable attribute in the
                        // code_attributes vector (per method). If empty, create a new entry, and fill local
                        // vars in it as and when encountered.
                        // If not empty, find the index of the vector which contains the
                        // LocalVariableTable, and enter the local vars there.
                        let local_var_table_index = if code_attributes.is_empty() {
                            code_attributes_count += 1;
                            code_attributes_length += 8;

                            let attribute_name_index = *cp.get_name(PHORON_LOCAL_VARIABLE_TABLE).ok_or(CodegenError::AttributeError {
                                attr: "Code",
                                details: "misisng attribute name index for local variable table in Code attribute"

                            })?;

                            let mut attribute_length = 2;
                            let local_variable_table_length = 0;
                            let local_variable_table = Vec::new();

                            code_attributes.push(AttributeInfo::LocalVariableTable {
                                attribute_name_index,
                                attribute_length,
                                local_variable_table_length,
                                local_variable_table,
                            });
                            0
                        } else {
                            let mut lvtindex = 0;
                            for (index, attr) in code_attributes.iter().enumerate() {
                                if let AttributeInfo::LocalVariableTable { .. } = attr {
                                    lvtindex = index;
                                    break;
                                }
                            }
                            lvtindex
                        };

                        let start_pc = *self.label_mapping.get(from_label).ok_or(
                            CodegenError::AttributeError {
                                attr: "Code",
                                details: "missing start_pc for local var for code attribute",
                            },
                        )?;

                        if let AttributeInfo::LocalVariableTable {
                            ref mut attribute_length,
                            ref mut local_variable_table_length,
                            ref mut local_variable_table,
                            ..
                        } = code_attributes[local_var_table_index]
                        {
                            *local_variable_table_length += 1;

                            let start_pc = *self.label_mapping.get(from_label).ok_or(CodegenError::AttributeError {
                                attr: "Code",
                                details: "missing start pc for local var for local variable table for Code attribute",
                            })? as u16;

                            let end_pc = *self.label_mapping.get(to_label).ok_or(CodegenError::AttributeError {
                                attr: "Code",
                                details: "missing length for local var for local vaiable table for Code attribute",
                            })? as u16;

                            let length = end_pc - start_pc;

                            let name_index = *cp.get_name(&name).ok_or(CodegenError::AttributeError {
                                attr: "Code",
                                details: "missing name index for local var for local variable table for Code attribute",
                            })?;

                            let descriptor_index = *cp.get_name(&field_descriptor.to_string()).ok_or(CodegenError::AttributeError {
                                attr: "Code",
                                details: "missing descriptor index for local var for local variable table for Code attribute",
                            })?;

                            let index = if local_variable_table.is_empty() {
                                0
                            } else {
                                local_variable_table.len() as u16
                            };

                            local_variable_table.push(LocalVariable {
                                start_pc,
                                length,
                                name_index,
                                descriptor_index,
                                index,
                            });

                            *attribute_length += 10; // 10 bytes per LocalVariable struct
                            code_attributes_length += 10;
                        }
                    }

                    // this goes in the exception_table field of the Code attribute
                    PhoronDirective::Catch {
                        ref class_name,
                        ref from_label,
                        ref to_label,
                        ref handler_label,
                    } => {
                        exception_table_length += 1;

                        let mut exc_handler = ExceptionHandler::default();
                        exc_handler.start_pc = *self.label_mapping.get(from_label).ok_or(
                            CodegenError::AttributeError {
                                attr: "Code",
                                details: "missing start_pc for exception handler",
                            },
                        )? as u16;

                        exc_handler.end_pc = *self.label_mapping.get(to_label).ok_or(
                            CodegenError::AttributeError {
                                attr: "Code",
                                details: "missing end_pc for exception handler",
                            },
                        )? as u16;

                        exc_handler.handler_pc = *self.label_mapping.get(handler_label).ok_or(
                            CodegenError::AttributeError {
                                attr: "Code",
                                details: "missing handler_pc for exception handler",
                            },
                        )? as u16;

                        exc_handler.catch_type =
                            *cp.get_class(class_name)
                                .ok_or(CodegenError::AttributeError {
                                    attr: "Code",
                                    details: "missing catch_type in exception handler",
                                })?
                                - 1; // fixme: see why this doesn't work evern though the idx in the cp is correct

                        exception_table.push(exc_handler);

                        // update attribute length for `Code` attribute
                        code_attributes_length += 4 * std::mem::size_of::<u16>() as u32;
                    }
                },

                PhoronInstruction::PhoronLabel(ref label) => {}

                PhoronInstruction::JvmInstruction(ref jvm_instr) => {
                    let opcodes = self.visit_jvm_instruction(jvm_instr, cp)?;

                    if let CodegenResultType::ByteVec(instr_opcodes) = opcodes {
                        let opcode_len = instr_opcodes.len() as i16;
                        code.extend_from_slice(&instr_opcodes);
                        self.curr_code_offset += opcode_len;
                    } else {
                        return Err(CodegenError::Unknown);
                    }
                }
            }
        }

        let code_length = code.len() as u32;
        code_attributes_length += code_length; // need to add sizes of all attributes

        method_info.attributes.push(AttributeInfo::Code {
            attribute_name_index,
            attribute_length: code_attributes_length,
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
            Aaload => CodegenResultType::ByteVec(vec![0x32]),
            Aastore => CodegenResultType::ByteVec(vec![0x53]),
            Aconstnull => CodegenResultType::ByteVec(vec![0x01]),

            // check: wide and varnum size implications
            Aload { ref varnum } => {
                let mut opcodes = vec![0x19];
                opcodes.extend_from_slice(&varnum.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Aload0 => CodegenResultType::ByteVec(vec![0x2a]),
            Aload1 => CodegenResultType::ByteVec(vec![0x2b]),
            Aload2 => CodegenResultType::ByteVec(vec![0x2c]),
            Aload3 => CodegenResultType::ByteVec(vec![0x2d]),

            Anewarray { ref component_type } => {
                let mut opcodes = vec![0xbd];

                let component_name = component_type.to_string();
                opcodes.extend_from_slice(&match component_type {
                    PhoronFieldDescriptor::BaseType(..) => unreachable!(),

                    PhoronFieldDescriptor::ObjectType { ref class_name } => {
                        let class_ref = *cp
                            .get_class(&component_name[1..component_name.len() - 1])
                            .ok_or(CodegenError::OpcodeError {
                                opcode: "anewarray",
                                details: "missing class reference",
                            })?;
                        class_ref.to_be_bytes()
                    }

                    PhoronFieldDescriptor::ArrayType { .. } => {
                        let array_class_ref =
                            *cp.get_class(&component_name)
                                .ok_or(CodegenError::OpcodeError {
                                    opcode: "anewarray",
                                    details: "missing array class reference",
                                })?;
                        array_class_ref.to_be_bytes()
                    }
                });

                CodegenResultType::ByteVec(opcodes)
            }

            Areturn => CodegenResultType::ByteVec(vec![0xb0]),
            Arraylength => CodegenResultType::ByteVec(vec![0xbe]),

            Astore { ref varnum } => {
                let mut opcodes = vec![0x3a];
                opcodes.extend_from_slice(&varnum.to_be_bytes());
                CodegenResultType::ByteVec(opcodes)
            }

            Astore0 => CodegenResultType::ByteVec(vec![0x4b]),
            Astore1 => CodegenResultType::ByteVec(vec![0x4c]),
            Astore2 => CodegenResultType::ByteVec(vec![0x4d]),
            Astore3 => CodegenResultType::ByteVec(vec![0x4e]),
            Athrow => CodegenResultType::ByteVec(vec![0xbf]),
            Baload => CodegenResultType::ByteVec(vec![0x33]),
            Bastore => CodegenResultType::ByteVec(vec![0x54]),

            Bipush(sb) => {
                let mut opcodes = vec![0x10];
                opcodes.extend_from_slice(&sb.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Caload => CodegenResultType::ByteVec(vec![0x34]),
            Castore => CodegenResultType::ByteVec(vec![0x55]),

            Checkcast { ref cast_type } => {
                let mut opcodes = vec![0xc0];

                let cast_name = cast_type.to_string();
                opcodes.extend_from_slice(&match cast_type {
                    PhoronFieldDescriptor::BaseType(..) => unreachable!(),

                    PhoronFieldDescriptor::ObjectType { ref class_name } => {
                        let class_ref = *cp.get_class(&cast_name[1..cast_name.len() - 1]).ok_or(
                            CodegenError::OpcodeError {
                                opcode: "checkcast",
                                details: "missing class reference",
                            },
                        )?;
                        class_ref.to_be_bytes()
                    }

                    PhoronFieldDescriptor::ArrayType { .. } => {
                        let array_class_ref =
                            *cp.get_class(&cast_name).ok_or(CodegenError::OpcodeError {
                                opcode: "checkcast",
                                details: "missing array class reference",
                            })?;
                        array_class_ref.to_be_bytes()
                    }
                });

                CodegenResultType::ByteVec(opcodes)
            }

            D2f => CodegenResultType::ByteVec(vec![0x90]),
            D2i => CodegenResultType::ByteVec(vec![0x8e]),
            D2l => CodegenResultType::ByteVec(vec![0x8f]),
            Dadd => CodegenResultType::ByteVec(vec![0x63]),
            Daload => CodegenResultType::ByteVec(vec![0x31]),
            Dastore => CodegenResultType::ByteVec(vec![0x52]),
            Dcmpg => CodegenResultType::ByteVec(vec![0x98]),
            Dcmpl => CodegenResultType::ByteVec(vec![0x97]),
            Dconst0 => CodegenResultType::ByteVec(vec![0x0e]),
            Dconst1 => CodegenResultType::ByteVec(vec![0x0f]),
            Ddiv => CodegenResultType::ByteVec(vec![0x6f]),

            Dload { ref varnum } => {
                let mut opcodes = vec![0x18];
                opcodes.extend_from_slice(&varnum.to_be_bytes());
                CodegenResultType::ByteVec(opcodes)
            }

            Dload0 => CodegenResultType::ByteVec(vec![0x26]),
            Dload1 => CodegenResultType::ByteVec(vec![0x27]),
            Dload2 => CodegenResultType::ByteVec(vec![0x28]),
            Dload3 => CodegenResultType::ByteVec(vec![0x29]),

            Dmul => CodegenResultType::ByteVec(vec![0x6b]),
            Dneg => CodegenResultType::ByteVec(vec![0x77]),
            Drem => CodegenResultType::ByteVec(vec![0x73]),
            Dreturn => CodegenResultType::ByteVec(vec![0xaf]),

            Dstore { ref varnum } => {
                let mut opcodes = vec![0x39];
                opcodes.extend_from_slice(&varnum.to_be_bytes());
                CodegenResultType::ByteVec(opcodes)
            }

            Dstore0 => CodegenResultType::ByteVec(vec![0x47]),
            Dstore1 => CodegenResultType::ByteVec(vec![0x48]),
            Dstore2 => CodegenResultType::ByteVec(vec![0x49]),
            Dstore3 => CodegenResultType::ByteVec(vec![0x4a]),
            Dsub => CodegenResultType::ByteVec(vec![0x67]),
            Dup => CodegenResultType::ByteVec(vec![0x59]),
            Dupx1 => CodegenResultType::ByteVec(vec![0x5a]),
            Dupx2 => CodegenResultType::ByteVec(vec![0x5b]),
            Dup2 => CodegenResultType::ByteVec(vec![0x5c]),
            Dup2x1 => CodegenResultType::ByteVec(vec![0x5d]),
            Dup2x2 => CodegenResultType::ByteVec(vec![0x5e]),
            F2d => CodegenResultType::ByteVec(vec![0x8d]),
            F2i => CodegenResultType::ByteVec(vec![0x8b]),
            F2l => CodegenResultType::ByteVec(vec![0x8c]),
            Fadd => CodegenResultType::ByteVec(vec![0x62]),
            Faload => CodegenResultType::ByteVec(vec![0x30]),
            Fastore => CodegenResultType::ByteVec(vec![0x51]),
            Fcmpg => CodegenResultType::ByteVec(vec![0x96]),
            Fcmpl => CodegenResultType::ByteVec(vec![0x95]),
            Fconst0 => CodegenResultType::ByteVec(vec![0x0b]),
            Fconst1 => CodegenResultType::ByteVec(vec![0x0c]),
            Fconst2 => CodegenResultType::ByteVec(vec![0x0d]),
            Fdiv => CodegenResultType::ByteVec(vec![0x6e]),

            Fload { ref varnum } => {
                let mut opcodes = vec![0x17];
                opcodes.extend_from_slice(&varnum.to_be_bytes());
                CodegenResultType::ByteVec(opcodes)
            }

            Fload0 => CodegenResultType::ByteVec(vec![0x22]),
            Fload1 => CodegenResultType::ByteVec(vec![0x23]),
            Fload2 => CodegenResultType::ByteVec(vec![0x24]),
            Fload3 => CodegenResultType::ByteVec(vec![0x25]),
            Fmul => CodegenResultType::ByteVec(vec![0x6a]),
            Fneg => CodegenResultType::ByteVec(vec![0x76]),
            Frem => CodegenResultType::ByteVec(vec![0x72]),
            Freturn => CodegenResultType::ByteVec(vec![0xae]),

            Fstore { ref varnum } => {
                let mut opcodes = vec![0x38];
                opcodes.extend_from_slice(&varnum.to_be_bytes());
                CodegenResultType::ByteVec(opcodes)
            }

            Fstore0 => CodegenResultType::ByteVec(vec![0x43]),
            Fstore1 => CodegenResultType::ByteVec(vec![0x44]),
            Fstore2 => CodegenResultType::ByteVec(vec![0x45]),
            Fstore3 => CodegenResultType::ByteVec(vec![0x46]),
            Fsub => CodegenResultType::ByteVec(vec![0x66]),

            Getstatic {
                ref class_name,
                ref field_name,
                ref field_descriptor,
            } => {
                let mut opcodes = vec![0xb2];

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
                let mut opcodes = vec![0xb4];
                let fieldref = *cp
                    .get_fieldref(class_name, field_name, &field_descriptor.to_string())
                    .ok_or(CodegenError::OpcodeError {
                        opcode: "getfield",
                        details: "missing field reference",
                    })?;
                opcodes.extend_from_slice(&fieldref.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Goto { ref label } => {
                let mut opcodes = vec![0xa7];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "goto",
                            details: "invalid label",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Gotow { ref label } => {
                let mut opcodes = vec![0xc8];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "goto_w",
                            details: "invalid label",
                        })?;

                let offset = (label_offset - self.curr_code_offset) as i32;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            I2b => CodegenResultType::ByteVec(vec![0x91]),
            I2c => CodegenResultType::ByteVec(vec![0x92]),
            I2d => CodegenResultType::ByteVec(vec![0x87]),
            I2f => CodegenResultType::ByteVec(vec![0x86]),
            I2l => CodegenResultType::ByteVec(vec![0x85]),
            I2s => CodegenResultType::ByteVec(vec![0x93]),
            Iadd => CodegenResultType::ByteVec(vec![0x60]),
            Iaload => CodegenResultType::ByteVec(vec![0x2e]),
            Iand => CodegenResultType::ByteVec(vec![0x7e]),
            Iastore => CodegenResultType::ByteVec(vec![0x4f]),
            Iconstm1 => CodegenResultType::ByteVec(vec![0x02]),
            Iconst0 => CodegenResultType::ByteVec(vec![0x03]),
            Iconst1 => CodegenResultType::ByteVec(vec![0x04]),
            Iconst2 => CodegenResultType::ByteVec(vec![0x05]),
            Iconst3 => CodegenResultType::ByteVec(vec![0x06]),
            Iconst4 => CodegenResultType::ByteVec(vec![0x07]),
            Iconst5 => CodegenResultType::ByteVec(vec![0x08]),
            Idiv => CodegenResultType::ByteVec(vec![0x6c]),

            Ifacmpeq { ref label } => {
                todo!()
            }
            Ifacmpne { ref label } => {
                todo!()
            }

            Ificmpeq { ref label } => {
                let mut opcodes = vec![0x9f];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "if_icmpeq",
                            details: "invalid label ",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ificmpne { ref label } => {
                let mut opcodes = vec![0xa0];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "if_icmpne",
                            details: "invalid label ",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ificmplt { ref label } => {
                let mut opcodes = vec![0xa1];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "if_icmplt",
                            details: "invalid label ",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ificmpge { ref label } => {
                let mut opcodes = vec![0xa2];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "if_icmpge",
                            details: "invalid label ",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ificmpgt { ref label } => {
                let mut opcodes = vec![0xa3];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "if_icmpgt",
                            details: "invalid label",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ificmple { ref label } => {
                let mut opcodes = vec![0xa4];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "if_icmple",
                            details: "invalid label ",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ifeq { ref label } => {
                let mut opcodes = vec![0x99];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "ifeq",
                            details: "invalid label",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ifne { ref label } => {
                let mut opcodes = vec![0x9a];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "ifne",
                            details: "invalid label",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Iflt { ref label } => {
                let mut opcodes = vec![0x9b];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "iflt",
                            details: "invalid label",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ifge { ref label } => {
                let mut opcodes = vec![0x9c];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "ifge",
                            details: "invalid label",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ifgt { ref label } => {
                let mut opcodes = vec![0x9d];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "ifgt",
                            details: "invalid label",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ifle { ref label } => {
                let mut opcodes = vec![0x9e];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "ifle",
                            details: "invalid label",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ifnull { ref label } => {
                let mut opcodes = vec![0xc6];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "ifnull",
                            details: "invalid label",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Ifnonnull { ref label } => {
                let mut opcodes = vec![0xc7];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "ifnull",
                            details: "invalid label",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            // check: wide
            Iinc {
                ref varnum,
                ref delta,
            } => {
                let mut opcodes = vec![0x84];

                let ub_varnum = *varnum as u8;
                let sb_delta = *delta as i8;

                opcodes.extend_from_slice(&ub_varnum.to_be_bytes());
                opcodes.extend_from_slice(&sb_delta.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Iload { ref varnum } => {
                let mut opcodes = vec![0x15];
                opcodes.extend_from_slice(&varnum.to_be_bytes());
                CodegenResultType::ByteVec(opcodes)
            }

            Iload0 => CodegenResultType::ByteVec(vec![0x1a]),
            Iload1 => CodegenResultType::ByteVec(vec![0x1b]),
            Iload2 => CodegenResultType::ByteVec(vec![0x1c]),
            Iload3 => CodegenResultType::ByteVec(vec![0x1d]),
            Imul => CodegenResultType::ByteVec(vec![0x68]),
            Ineg => CodegenResultType::ByteVec(vec![0x74]),

            Instanceof { ref check_type } => {
                let mut opcodes = vec![0xc1];

                let check_name = check_type.to_string();
                opcodes.extend_from_slice(&match check_type {
                    PhoronFieldDescriptor::BaseType(..) => unreachable!(),

                    PhoronFieldDescriptor::ObjectType { ref class_name } => {
                        let class_ref = *cp.get_class(&check_name[1..check_name.len() - 1]).ok_or(
                            CodegenError::OpcodeError {
                                opcode: "instanecof",
                                details: "missing class reference",
                            },
                        )?;
                        class_ref.to_be_bytes()
                    }

                    PhoronFieldDescriptor::ArrayType { .. } => {
                        let array_class_ref =
                            *cp.get_class(&check_name).ok_or(CodegenError::OpcodeError {
                                opcode: "instanceof",
                                details: "missing array class reference",
                            })?;
                        array_class_ref.to_be_bytes()
                    }
                });

                CodegenResultType::ByteVec(opcodes)
            }

            // check
            Invokeinterface {
                ref interface_name,
                ref method_name,
                ref method_descriptor,
                ref ub,
            } => {
                let mut opcodes = vec![0xb9];

                let methodref_index = *cp
                    .get_methodref(interface_name, method_name, &method_descriptor.to_string())
                    .ok_or(CodegenError::OpcodeError {
                        opcode: "invokeinterface",
                        details: "missing method reference",
                    })?;

                opcodes.extend_from_slice(&methodref_index.to_be_bytes());
                opcodes.extend_from_slice(&ub.to_be_bytes());
                opcodes.push(0); // as per the spec

                CodegenResultType::ByteVec(opcodes)
            }

            Invokespecial {
                ref class_name,
                ref method_name,
                ref method_descriptor,
            } => {
                let mut opcodes = vec![0xb7];

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
                let mut opcodes = vec![0xb8];

                let methodref_index = *cp
                    .get_methodref(class_name, method_name, &method_descriptor.to_string())
                    .ok_or(CodegenError::OpcodeError {
                        opcode: "invokestatic",
                        details: "missing method reference",
                    })?;
                opcodes.extend_from_slice(&methodref_index.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
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

            Ior => CodegenResultType::ByteVec(vec![0x80]),
            Irem => CodegenResultType::ByteVec(vec![0x70]),
            Ireturn => CodegenResultType::ByteVec(vec![0xac]),
            Ishl => CodegenResultType::ByteVec(vec![0x78]),
            Ishr => CodegenResultType::ByteVec(vec![0x7a]),

            Istore { ref varnum } => {
                let mut opcodes = vec![0x36];
                opcodes.extend_from_slice(&varnum.to_be_bytes());
                CodegenResultType::ByteVec(opcodes)
            }
            Istore0 => CodegenResultType::ByteVec(vec![0x3b]),
            Istore1 => CodegenResultType::ByteVec(vec![0x3c]),
            Istore2 => CodegenResultType::ByteVec(vec![0x3d]),
            Istore3 => CodegenResultType::ByteVec(vec![0x3e]),
            Isub => CodegenResultType::ByteVec(vec![0x64]),
            Iushr => CodegenResultType::ByteVec(vec![0x7c]),
            Ixor => CodegenResultType::ByteVec(vec![0x82]),

            Jsr { ref label } => {
                let mut opcodes = vec![0xa8];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "jsr",
                            details: "missing label",
                        })?;

                let offset = label_offset - self.curr_code_offset;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Jsrw { ref label } => {
                let mut opcodes = vec![0xc9];

                let label_offset =
                    self.label_mapping
                        .get(label)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "jsr_w",
                            details: "missing label",
                        })?;

                let offset = (label_offset - self.curr_code_offset) as i32;
                opcodes.extend_from_slice(&offset.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            L2d => CodegenResultType::ByteVec(vec![0x8a]),
            L2f => CodegenResultType::ByteVec(vec![0x89]),
            L2i => CodegenResultType::ByteVec(vec![0x88]),
            Ladd => CodegenResultType::ByteVec(vec![0x61]),
            Laload => CodegenResultType::ByteVec(vec![0x2f]),
            Land => CodegenResultType::ByteVec(vec![0x7f]),
            Lastore => CodegenResultType::ByteVec(vec![0x50]),
            Lcmp => CodegenResultType::ByteVec(vec![0x94]),
            Lconst0 => CodegenResultType::ByteVec(vec![0x09]),
            Lconst1 => CodegenResultType::ByteVec(vec![0x0a]),

            Ldc(ref ldc_val) => {
                let mut opcodes = vec![0x12];

                match ldc_val {
                    LdcValue::QuotedString(ref string) => {
                        let string_index =
                            *cp.get_string(string).ok_or(CodegenError::OpcodeError {
                                opcode: "ldc",
                                details: "missing quoted string",
                            })?;

                        opcodes.extend_from_slice(&string_index.to_be_bytes()[1..])
                    }

                    LdcValue::Integer(int) => {
                        let int_index = *cp.get_integer(*int).ok_or(CodegenError::OpcodeError {
                            opcode: "ldc",
                            details: "missing integer",
                        })?;
                        opcodes.push(int_index.to_be_bytes()[1]);
                    }

                    LdcValue::Float(float) => {
                        let float_index =
                            *cp.get_float(*float).ok_or(CodegenError::OpcodeError {
                                opcode: "lcd",
                                details: "missing float",
                            })?;
                        opcodes.push(float_index.to_be_bytes()[1]);
                    }
                }

                CodegenResultType::ByteVec(opcodes)
            }

            Ldcw(ref ldcw_val) => {
                let mut opcodes = vec![0x13];

                match ldcw_val {
                    LdcwValue::QuotedString(ref string) => {
                        let string_index =
                            *cp.get_string(string).ok_or(CodegenError::OpcodeError {
                                opcode: "ldcw",
                                details: "missing quoted string",
                            })?;

                        opcodes.extend_from_slice(&string_index.to_be_bytes())
                    }

                    LdcwValue::Integer(int) => {
                        let int_index = *cp.get_integer(*int).ok_or(CodegenError::OpcodeError {
                            opcode: "ldcw",
                            details: "missing integer",
                        })?;
                        opcodes.extend_from_slice(&int_index.to_be_bytes());
                    }

                    LdcwValue::Float(float) => {
                        let float_index =
                            *cp.get_float(*float).ok_or(CodegenError::OpcodeError {
                                opcode: "ldcw",
                                details: "missing float",
                            })?;
                        opcodes.extend_from_slice(&float_index.to_be_bytes());
                    }
                }

                CodegenResultType::ByteVec(opcodes)
            }

            Ldc2w(ref ldc2w_val) => {
                let mut opcodes = vec![0x14];

                match ldc2w_val {
                    Ldc2wValue::Long(long) => {
                        let long_index = *cp.get_long(*long).ok_or(CodegenError::OpcodeError {
                            opcode: "ldc2w",
                            details: "missing long",
                        })?;
                        opcodes.extend_from_slice(&long_index.to_be_bytes());
                    }

                    Ldc2wValue::Double(double) => {
                        let double_index =
                            *cp.get_double(*double).ok_or(CodegenError::OpcodeError {
                                opcode: "ldc2w",
                                details: "missing double",
                            })?;
                        opcodes.extend_from_slice(&double_index.to_be_bytes());
                    }
                }

                CodegenResultType::ByteVec(opcodes)
            }

            Ldiv => CodegenResultType::ByteVec(vec![0x6d]),

            Lload { ref varnum } => {
                let mut opcodes = vec![0x16];
                opcodes.extend_from_slice(&varnum.to_be_bytes());
                CodegenResultType::ByteVec(opcodes)
            }

            Lload0 => CodegenResultType::ByteVec(vec![0x1e]),
            Lload1 => CodegenResultType::ByteVec(vec![0x1f]),
            Lload2 => CodegenResultType::ByteVec(vec![0x20]),
            Lload3 => CodegenResultType::ByteVec(vec![0x21]),
            Lmul => CodegenResultType::ByteVec(vec![0x69]),
            Lneg => CodegenResultType::ByteVec(vec![0x75]),

            Lookupswitch {
                ref switches,
                ref default,
            } => {
                let mut opcodes = vec![0xab];

                // default offset padding with zerees
                let opcode_len =
                    1i16 + switches.len() as i16 * 2i16 * std::mem::size_of::<u32>() as i16;
                let default_offset_padding = (self.curr_code_offset + opcode_len) % 4;

                for _ in 0..default_offset_padding {
                    opcodes.push(0);
                }

                // default case
                let default_label_offset =
                    self.label_mapping
                        .get(default)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "lookupswitch",
                            details: "missing default label",
                        })?;

                let default_label_offset = (default_label_offset - self.curr_code_offset) as i32;
                opcodes.extend_from_slice(&default_label_offset.to_be_bytes());

                // number of match pairs
                let n = switches.len() as i32;
                opcodes.extend_from_slice(&n.to_be_bytes());

                // each switch pair, in sorted order of keys
                for swpair in switches {
                    opcodes.extend_from_slice(&swpair.key.to_be_bytes());
                    let label_offset =
                        self.label_mapping
                            .get(&swpair.label)
                            .ok_or(CodegenError::OpcodeError {
                                opcode: "lookupswitch",
                                details: "missing label for switch pair",
                            })?;

                    let offset = (label_offset - self.curr_code_offset) as i32;
                    opcodes.extend_from_slice(&offset.to_be_bytes());
                }

                CodegenResultType::ByteVec(opcodes)
            }

            Lor => CodegenResultType::ByteVec(vec![0x81]),
            Lrem => CodegenResultType::ByteVec(vec![0x71]),
            Lreturn => CodegenResultType::ByteVec(vec![0xad]),
            Lshl => CodegenResultType::ByteVec(vec![0x79]),
            Lshr => CodegenResultType::ByteVec(vec![0x7b]),

            Lstore { ref varnum } => {
                let mut opcodes = vec![0x37];
                opcodes.extend_from_slice(&varnum.to_be_bytes());
                CodegenResultType::ByteVec(opcodes)
            }
            Lstore0 => CodegenResultType::ByteVec(vec![0x3f]),
            Lstore1 => CodegenResultType::ByteVec(vec![0x40]),
            Lstore2 => CodegenResultType::ByteVec(vec![0x41]),
            Lstore3 => CodegenResultType::ByteVec(vec![0x42]),
            Lsub => CodegenResultType::ByteVec(vec![0x65]),
            Lushr => CodegenResultType::ByteVec(vec![0x7d]),
            Lxor => CodegenResultType::ByteVec(vec![0x83]),
            Monitorenter => CodegenResultType::ByteVec(vec![0xc2]),
            Monitorexit => CodegenResultType::ByteVec(vec![0xc3]),

            Multianewarray {
                ref component_type,
                ref dimensions,
            } => {
                let mut opcodes = vec![0xc5];
                let class_ref = *cp.get_class(&component_type.to_string()).ok_or(
                    CodegenError::OpcodeError {
                        opcode: "multianewarray",
                        details: "missing class reference",
                    },
                )?;
                opcodes.extend_from_slice(&class_ref.to_be_bytes());
                opcodes.push(*dimensions);

                CodegenResultType::ByteVec(opcodes)
            }

            Newarray { ref component_type } => {
                let mut opcodes = vec![0xbc];

                opcodes.push(match component_type {
                    PhoronBaseType::Byte => 8,
                    PhoronBaseType::Character => 5,
                    PhoronBaseType::Double => 7,
                    PhoronBaseType::Float => 6,
                    PhoronBaseType::Integer => 10,
                    PhoronBaseType::Long => 11,
                    PhoronBaseType::Short => 9,
                    PhoronBaseType::Boolean => 4,
                });

                CodegenResultType::ByteVec(opcodes)
            }

            New { ref class_name } => {
                let mut opcodes = vec![0xbb];

                let class_ref =
                    *cp.get_class(&class_name.to_string())
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "new",
                            details: "missing class reference",
                        })?;
                opcodes.extend_from_slice(&class_ref.to_be_bytes());

                CodegenResultType::ByteVec(opcodes)
            }

            Nop => CodegenResultType::ByteVec(vec![0x00]),
            Pop => CodegenResultType::ByteVec(vec![0x57]),
            Pop2 => CodegenResultType::ByteVec(vec![0x58]),

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

            Ret { ref varnum } => {
                let mut opcodes = vec![0xa9];
                opcodes.extend_from_slice(&varnum.to_be_bytes());
                CodegenResultType::ByteVec(opcodes)
            }

            Saload => CodegenResultType::ByteVec(vec![0x35]),
            Sastore => CodegenResultType::ByteVec(vec![0x56]),

            Sipush(ref ss) => {
                let mut opcodes = vec![0x11];
                opcodes.extend_from_slice(&ss.to_be_bytes());
                CodegenResultType::ByteVec(opcodes)
            }

            Swap => CodegenResultType::ByteVec(vec![0x5f]),

            Tableswitch {
                ref low,
                ref high,
                ref switches,
                ref default,
            } => {
                let mut opcodes = vec![0xaa];

                // default offset padding with zerees
                let opcode_len = 1i16
                    + 2i16 * std::mem::size_of::<i32>() as i16
                    + (high - low + 1) as i16 * std::mem::size_of::<i32>() as i16;

                let default_offset_padding = (self.curr_code_offset + opcode_len) % 4;

                for _ in 0..default_offset_padding {
                    opcodes.push(0);
                }

                // default case
                let default_label_offset =
                    self.label_mapping
                        .get(default)
                        .ok_or(CodegenError::OpcodeError {
                            opcode: "tableswitch",
                            details: "missing default label",
                        })?;

                let default_label_offset =
                    (4 + default_label_offset - self.curr_code_offset) as i32;
                opcodes.extend_from_slice(&default_label_offset.to_be_bytes());

                // low and high
                opcodes.extend_from_slice(&low.to_be_bytes());
                opcodes.extend_from_slice(&high.to_be_bytes());

                // switch offsets
                for swoffset in switches {
                    let label_offset =
                        self.label_mapping
                            .get(swoffset)
                            .ok_or(CodegenError::OpcodeError {
                                opcode: "tableswitch",
                                details: "missing label for switch offset",
                            })?;

                    let offset = (4 + label_offset - self.curr_code_offset) as i32;
                    opcodes.extend_from_slice(&offset.to_be_bytes());
                }

                CodegenResultType::ByteVec(opcodes)
            }

            Wide(ref wide_instr) => {
                let mut opcodes = vec![0xc4];

                match wide_instr {
                    WideInstruction::Iload { ref varnum } => {
                        opcodes.push(0x15);

                        // fixme: how to map this to the local var array as in the case of label
                        // offsets? How is the local variable number mapped - at the Jasmin level
                        // and at the JVM level?
                        opcodes.extend_from_slice(&varnum.to_be_bytes());
                    }

                    WideInstruction::Fload { ref varnum } => {
                        opcodes.push(0x17);
                        opcodes.extend_from_slice(&varnum.to_be_bytes());
                    }

                    WideInstruction::Aload { ref varnum } => {
                        opcodes.push(0x19);
                        opcodes.extend_from_slice(&varnum.to_be_bytes());
                    }

                    WideInstruction::Lload { ref varnum } => {
                        opcodes.push(0x16);
                        opcodes.extend_from_slice(&varnum.to_be_bytes());
                    }

                    WideInstruction::Dload { ref varnum } => {
                        opcodes.push(0x18);
                        opcodes.extend_from_slice(&varnum.to_be_bytes());
                    }

                    WideInstruction::Istore { ref varnum } => {
                        opcodes.push(0x36);
                        opcodes.extend_from_slice(&varnum.to_be_bytes());
                    }

                    WideInstruction::Fstore { ref varnum } => {
                        opcodes.push(0x38);
                        opcodes.extend_from_slice(&varnum.to_be_bytes());
                    }

                    WideInstruction::Astore { ref varnum } => {
                        opcodes.push(0x3a);
                        opcodes.extend_from_slice(&varnum.to_be_bytes());
                    }

                    WideInstruction::Lstore { ref varnum } => {
                        opcodes.push(0x37);
                        opcodes.extend_from_slice(&varnum.to_be_bytes());
                    }

                    WideInstruction::Dstore { ref varnum } => {
                        opcodes.push(0x39);
                        opcodes.extend_from_slice(&varnum.to_be_bytes());
                    }

                    WideInstruction::Ret { ref varnum } => {
                        opcodes.push(0xa9);
                        opcodes.extend_from_slice(&varnum.to_be_bytes());
                    }

                    WideInstruction::IInc {
                        ref varnum,
                        ref delta,
                    } => {
                        opcodes.push(0x84);

                        opcodes.extend_from_slice(&varnum.to_be_bytes());
                        opcodes.extend_from_slice(&delta.to_be_bytes());
                    }
                }

                CodegenResultType::ByteVec(opcodes)
            }
        })
    }
}
