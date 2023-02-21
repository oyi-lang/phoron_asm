#[derive(Debug, PartialEq)]
pub struct PhoronProgram {
    pub header: PhoronHeader,
    pub body: PhoronBody,
}

// header

#[derive(Debug, PartialEq)]
pub struct PhoronSourceFileDef {
    pub source_file: String,
}

// classes and interfaces

#[derive(Debug, PartialEq)]
pub enum PhoronClassOrInterfaceAccessFlag {
    AccPublic,
    AccFinal,
    AccSuper,
    AccInterface,
    AccAbstract,
    AccSynthetic,
    AccAnnotation,
    AccEnum,
    AccModule,
}

#[derive(PartialEq, Debug)]
pub struct PhoronClassDef {
    pub name: String,
    pub access_flags: Vec<PhoronClassOrInterfaceAccessFlag>,
}

#[derive(PartialEq, Debug)]
pub struct PhoronInterfaceDef {
    pub name: String,
    pub access_flags: Vec<PhoronClassOrInterfaceAccessFlag>,
}

#[derive(PartialEq, Debug)]
pub enum PhoronClassOrInterface {
    Class(PhoronClassDef),
    Interface(PhoronInterfaceDef),
}

#[derive(PartialEq, Debug)]
pub struct PhoronSuperDef {
    pub super_class_name: String,
}

#[derive(PartialEq, Debug)]
pub struct PhoronHeader {
    pub sourcefile_def: Option<PhoronSourceFileDef>,
    pub class_or_interface_def: PhoronClassOrInterface,
    pub super_def: PhoronSuperDef,
}

// Descriptors

#[derive(PartialEq, Debug)]
pub enum PhoronFieldDescriptor {
    BaseType(PhoronBaseType),
    ObjectType {
        class_name: String,
    },
    ArrayType {
        component_type: Box<PhoronFieldDescriptor>,
    },
}

#[derive(PartialEq, Debug)]
pub enum PhoronBaseType {
    Byte,
    Character,
    Double,
    Float,
    Integer,
    Long,
    Short,
    Boolean,
}

#[derive(PartialEq, Debug)]
pub struct PhoronMethodDescriptor {
    pub param_descriptor: Option<PhoronFieldDescriptor>,
    pub return_descriptor: PhoronReturnDescriptor,
}

#[derive(PartialEq, Debug)]
pub enum PhoronReturnDescriptor {
    FieldDescriptor(PhoronFieldDescriptor),
    VoidDescriptor,
}

// body

// Fields

#[derive(PartialEq, Debug)]
pub enum PhoronFieldAccessFlag {
    AccPublic,
    AccPrivate,
    AccProtected,
    AccStatic,
    AccFinal,
    AccVolatile,
    AccTransient,
    AccSynthetic,
    AccEnum,
}

#[derive(PartialEq, Debug)]
pub enum PhoronFieldInitValue {
    Double(f64),
    Integer(i64),
    QuotedString(String),
}

#[derive(PartialEq, Debug)]
pub struct PhoronFieldDef {
    pub name: String,
    pub access_flags: Vec<PhoronFieldAccessFlag>,
    pub descriptor: PhoronFieldDescriptor,
    pub init_val: Option<PhoronFieldInitValue>,
}

// methods

#[derive(PartialEq, Debug)]
pub enum PhoronMethodAccessFlag {
    AccPublic,
    AccPrivate,
    AccProtected,
    AccStatic,
    AccFinal,
    AccSynchronized,
    AccBridge,
    AccVarargs,
    AccNative,
    AccAbstract,
    AccStrict,
    AccSynthetic,
}

#[derive(PartialEq, Debug)]
pub enum PhoronDirective {
    LimitStack(u16),
    LimitLocals(u16),
    Throws {
        class_name: String,
    },
    LineNumber(u16),
    Var {
        name: String,
        field_descriptor: String,
        from_label: String,
        to_label: String,
    },
    Catch {
        class_name: String,
        from_label: String,
        to_label: String,
        handler_label: String,
    },
}

#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    Boolean,
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
}

#[derive(Debug, PartialEq)]
pub enum LdcValue {
    Double(f64),
    Integer(i64),
    QuotedString(String),
}

// todo - fill in the exact parameters for each instruction
#[derive(PartialEq, Debug)]
pub enum JvmInstruction {
    Aaload,
    Anewarray,
    Areturn,
    Aastore,
    Aconstnull,
    Aload0,
    Aload1,
    Aload2,
    Aload3,
    Aload,
    Arraylength,
    Astore0,
    Astore1,
    Astore2,
    Astore3,
    Astore,
    Athrow,
    Baload,
    Bastore,
    Bipush(i8),
    Caload,
    Castore,
    Checkcast,
    Dadd,
    Daload,
    Dastore,
    Dcmpg,
    Dcmpl,
    Dconst0,
    Dconst1,
    Ddiv,
    D2f,
    D2i,
    D2l,
    Dload0,
    Dload1,
    Dload2,
    Dload3,
    Dload,
    Dmul,
    Dneg,
    Drem,
    Dreturn,
    Dstore0,
    Dstore1,
    Dstore2,
    Dstore3,
    Dstore,
    Dsub,
    Dup2x1,
    Dup2x2,
    Dup2,
    Dupx1,
    Dupx2,
    Dup,
    F2d,
    F2i,
    F2l,
    Fadd,
    Faload,
    Fastore,
    Fcmpg,
    Fcmpl,
    Fconst0,
    Fconst1,
    Fconst2,
    Fdiv,
    Fload0,
    Fload1,
    Fload2,
    Fload3,
    Fload,
    Fmul,
    Fneg,
    Frem,
    Freturn,
    Fstore0,
    Fstore1,
    Fstore2,
    Fstore3,
    Fstore,
    Fsub,
    Getstatic {
        class_name: String,
        field_name: String,
        descriptor: PhoronFieldDescriptor,
    },
    Getfield {
        class_name: String,
        field_name: String,
        descriptor: PhoronFieldDescriptor,
    },
    Goto,
    Gotow,
    I2b,
    I2c,
    I2d,
    I2f,
    I2l,
    I2s,
    Iadd,
    Iaload,
    Iand,
    Iastore,
    Iconstm1,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,
    Idiv,
    Ifacmpeq,
    Ifacmpne,
    Ificmpeq,
    Ificmpge,
    Ificmpgt,
    Ificmple,
    Ificmplt,
    Ificmpne,
    Ifeq,
    Ifge,
    Ifgt,
    Ifle,
    Iflt,
    Ifne,
    Ifnonnull,
    Ifnull,
    Iinc,
    Iload0,
    Iload1,
    Iload2,
    Iload3,
    Iload,
    Imul,
    Ineg,
    Instanceof,
    Invokeinterface,
    Invokespecial {
        class_name: String,
        method_name: String,
        descriptor: PhoronMethodDescriptor,
    },
    Invokestatic {
        class_name: String,
        method_name: String,
        descriptor: PhoronMethodDescriptor,
    },
    Invokevirtual {
        class_name: String,
        method_name: String,
        descriptor: PhoronMethodDescriptor,
    },
    Ior,
    Irem,
    Ireturn,
    Ishl,
    Ishr,
    Istore0,
    Istore1,
    Istore2,
    Istore3,
    Istore,
    Isub,
    Iushr,
    Ixor,
    Jsrw,
    Jsr,
    L2d,
    L2f,
    L2i,
    Ladd,
    Loaload,
    Land,
    Lastore,
    Lcmp,
    Lconst0,
    Lconst1,
    Ldc2w,
    Ldcw,
    Ldc(LdcValue),
    Ldiv,
    Lload,
    Lload0,
    Lload1,
    Lload2,
    Lload3,
    Lmul,
    Lneg,
    Lookupswitch,
    Lor,
    Lrem,
    Lreturn,
    Lshl,
    Lshr,
    Lstore,
    Lstore0,
    Lstore1,
    Lstore2,
    Lstore3,
    Lsub,
    Lushr,
    Lxor,
    Monitorenter,
    Monitorexit,
    Multianewarray,
    Newarray {
        component_type: PrimitiveType,
    },
    New {
        class_name: String,
    },
    Nop,
    Pop2,
    Pop,
    Putfield,
    Putstatic,
    Return,
    Ret,
    Saload,
    Sastore,
    Sipush,
    Swap,
    Tableswitch,
}

#[derive(PartialEq, Debug)]
pub enum PhoronInstruction {
    PhoronDirective(PhoronDirective),
    JvmInstruction(JvmInstruction),
    PhoronLabel(String),
}

#[derive(PartialEq, Debug)]
pub struct PhoronMethodDef {
    pub name: String,
    pub access_flags: Vec<PhoronMethodAccessFlag>,
    pub descriptor: PhoronMethodDescriptor,
    pub instructions: Vec<PhoronInstruction>,
}

#[derive(PartialEq, Debug)]
pub struct PhoronBody {
    pub field_defs: Vec<PhoronFieldDef>,
    pub method_defs: Vec<PhoronMethodDef>,
}
