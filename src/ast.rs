/// Trait to visit the nodes of the AST.
///
/// `Input` is the expected input to the visitor methods.
/// `Result` is the result type of the operations themselves.
pub trait PhoronAstVisitor<'a> {
    type Input;
    type Result;

    fn visit_program(&mut self, program: &PhoronProgram, input: Self::Input) -> Self::Result;
    fn visit_header(&mut self, header: &PhoronHeader, input: Self::Input) -> Self::Result;

    fn visit_sourcefile_def(
        &mut self,
        sourcefile_def: &PhoronSourceFileDef,
        input: Self::Input,
    ) -> Self::Result;

    fn visit_class_def(&mut self, class_def: &PhoronClassDef, input: Self::Input) -> Self::Result;
    fn visit_interface_def(
        &mut self,
        class_def: &PhoronInterfaceDef,
        input: Self::Input,
    ) -> Self::Result;

    fn visit_super_def(&mut self, super_def: &PhoronSuperDef, input: Self::Input) -> Self::Result;

    fn visit_body(&mut self, body: &PhoronBody, input: Self::Input) -> Self::Result;
    fn visit_field_def(&mut self, field_def: &PhoronFieldDef, input: Self::Input) -> Self::Result;

    fn visit_method_def(
        &mut self,
        method_def: &PhoronMethodDef,
        input: Self::Input,
    ) -> Self::Result;

    fn visit_directive(&mut self, directive: &PhoronDirective, input: Self::Input) -> Self::Result;
    fn visit_jvm_instruction(&mut self, instr: &JvmInstruction, input: Self::Input)
        -> Self::Result;
}

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

use std::fmt;

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

impl fmt::Display for PhoronFieldDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PhoronFieldDescriptor::*;

        write!(
            f,
            "{}",
            match *self {
                BaseType(ref base_type) => base_type.to_string(),
                ObjectType { ref class_name } => format!("L{};", class_name),
                ArrayType { ref component_type } => format!("[{}", component_type.to_string()),
            }
        )
    }
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

impl fmt::Display for PhoronBaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PhoronBaseType::*;

        write!(
            f,
            "{}",
            match *self {
                Byte => "B",
                Character => "C",
                Double => "D",
                Float => "F",
                Integer => "I",
                Long => "J",
                Short => "S",
                Boolean => "Z",
            }
        )
    }
}

#[derive(PartialEq, Debug)]
pub struct PhoronMethodDescriptor {
    pub param_descriptor: Vec<PhoronFieldDescriptor>,
    pub return_descriptor: PhoronReturnDescriptor,
}

impl fmt::Display for PhoronMethodDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut param_descriptor = String::new();
        param_descriptor.push('(');

        self.param_descriptor
            .iter()
            .for_each(|field_desc| param_descriptor.push_str(&field_desc.to_string()));
        param_descriptor.push(')');

        write!(
            f,
            "{}",
            format!("{}{}", param_descriptor, self.return_descriptor.to_string())
        )
    }
}

#[derive(PartialEq, Debug)]
pub enum PhoronReturnDescriptor {
    FieldDescriptor(PhoronFieldDescriptor),
    VoidDescriptor,
}

impl fmt::Display for PhoronReturnDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PhoronReturnDescriptor::*;

        write!(
            f,
            "{}",
            match *self {
                FieldDescriptor(ref field_desc) => field_desc.to_string(),
                VoidDescriptor => "V".into(),
            }
        )
    }
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
    pub field_descriptor: PhoronFieldDescriptor,
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
        varnum: u16,
        name: String,
        field_descriptor: PhoronFieldDescriptor,
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
pub enum ClassOrArrayTypeDescriptor {
    ClassType {
        class_name: String,
    },

    ArrayType {
        component_type: Box<ClassOrArrayTypeDescriptor>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Ldc2wValue {
    Long(i64),
    Double(f64),
}

#[derive(Debug, PartialEq)]
pub enum LdcValue {
    Double(f64),
    Integer(i64),
    QuotedString(String),
}

#[derive(Debug, PartialEq)]
pub struct LookupSwitchPair {
    pub key: i32,
    pub label: String,
}

#[derive(PartialEq, Debug)]
pub enum JvmInstruction {
    Aaload,
    Anewarray {
        component_type: ClassOrArrayTypeDescriptor,
    },
    Areturn,
    Aastore,
    Aconstnull,
    Aload0,
    Aload1,
    Aload2,
    Aload3,
    Aload {
        varnum: u16,
    },
    Arraylength,
    Astore0,
    Astore1,
    Astore2,
    Astore3,
    Astore {
        varnum: u16,
    },
    Athrow,
    Baload,
    Bastore,
    Bipush(i8),
    Caload,
    Castore,
    Checkcast {
        cast_type: ClassOrArrayTypeDescriptor,
    },
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
    Dload {
        varnum: u16,
    },
    Dmul,
    Dneg,
    Drem,
    Dreturn,
    Dstore0,
    Dstore1,
    Dstore2,
    Dstore3,
    Dstore {
        varnum: u16,
    },
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
    Fload {
        varnum: u16,
    },
    Fmul,
    Fneg,
    Frem,
    Freturn,
    Fstore0,
    Fstore1,
    Fstore2,
    Fstore3,
    Fstore {
        varnum: u16,
    },
    Fsub,
    Getstatic {
        class_name: String,
        field_name: String,
        field_descriptor: PhoronFieldDescriptor,
    },
    Getfield {
        class_name: String,
        field_name: String,
        field_descriptor: PhoronFieldDescriptor,
    },
    Goto {
        label: String,
    },
    Gotow {
        label: String,
    },
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
    Ifacmpeq {
        label: String,
    },
    Ifacmpne {
        label: String,
    },
    Ificmpeq {
        label: String,
    },
    Ificmpge {
        label: String,
    },
    Ificmpgt {
        label: String,
    },
    Ificmple {
        label: String,
    },
    Ificmplt {
        label: String,
    },
    Ificmpne {
        label: String,
    },
    Ifeq {
        label: String,
    },
    Ifge {
        label: String,
    },
    Ifgt {
        label: String,
    },
    Ifle {
        label: String,
    },
    Iflt {
        label: String,
    },
    Ifne {
        label: String,
    },
    Ifnonnull {
        label: String,
    },
    Ifnull {
        label: String,
    },
    Iinc {
        varnum: u16,
        delta: i16,
    },
    Iload0,
    Iload1,
    Iload2,
    Iload3,
    Iload {
        varnum: u16,
    },
    Imul,
    Ineg,
    Instanceof {
        check_type: ClassOrArrayTypeDescriptor,
    },
    Invokeinterface {
        interface_name: String,
        method_name: String,
        method_descriptor: PhoronMethodDescriptor,
        ub: u8,
    },
    Invokespecial {
        class_name: String,
        method_name: String,
        method_descriptor: PhoronMethodDescriptor,
    },
    Invokestatic {
        class_name: String,
        method_name: String,
        method_descriptor: PhoronMethodDescriptor,
    },
    Invokevirtual {
        class_name: String,
        method_name: String,
        method_descriptor: PhoronMethodDescriptor,
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
    Istore {
        varnum: u16,
    },
    Isub,
    Iushr,
    Ixor,
    Jsrw {
        label: String,
    },
    Jsr {
        label: String,
    },
    L2d,
    L2f,
    L2i,
    Ladd,
    Laload,
    Land,
    Lastore,
    Lcmp,
    Lconst0,
    Lconst1,
    Ldc2w(Ldc2wValue),
    Ldcw(LdcValue),
    Ldc(LdcValue),
    Ldiv,
    Lload {
        varnum: u16,
    },
    Lload0,
    Lload1,
    Lload2,
    Lload3,
    Lmul,
    Lneg,
    Lookupswitch {
        switches: Vec<LookupSwitchPair>,
        default: String,
    },
    Lor,
    Lrem,
    Lreturn,
    Lshl,
    Lshr,
    Lstore {
        varnum: u16,
    },
    Lstore0,
    Lstore1,
    Lstore2,
    Lstore3,
    Lsub,
    Lushr,
    Lxor,
    Monitorenter,
    Monitorexit,
    Multianewarray {
        component_type: PhoronFieldDescriptor,
        dimensions: u8,
    },
    Newarray {
        component_type: PhoronBaseType,
    },
    New {
        class_name: String,
    },
    Nop,
    Pop2,
    Pop,
    Putfield {
        class_name: String,
        field_name: String,
        field_descriptor: PhoronFieldDescriptor,
    },
    Putstatic {
        class_name: String,
        field_name: String,
        field_descriptor: PhoronFieldDescriptor,
    },
    Return,
    Ret {
        varnum: u16,
    },
    Saload,
    Sastore,
    Sipush(i16),
    Swap,
    Tableswitch {
        low: i32,
        high: i32,
        switches: Vec<String>,
        default: String,
    },
    Wide,
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
    pub method_descriptor: PhoronMethodDescriptor,
    pub instructions: Vec<PhoronInstruction>,
}

#[derive(PartialEq, Debug)]
pub struct PhoronBody {
    pub field_defs: Vec<PhoronFieldDef>,
    pub method_defs: Vec<PhoronMethodDef>,
}

#[cfg(test)]
mod tests {
    use super::{PhoronBaseType::*, PhoronFieldDescriptor::*, PhoronReturnDescriptor::*, *};

    #[test]
    fn test_base_type_to_string() {
        assert_eq!(&Byte.to_string(), "B");
        assert_eq!(&Character.to_string(), "C");
        assert_eq!(&Double.to_string(), "D");
        assert_eq!(&Float.to_string(), "F");
        assert_eq!(&Integer.to_string(), "I");
        assert_eq!(&Long.to_string(), "J");
        assert_eq!(&Short.to_string(), "S");
        assert_eq!(&Boolean.to_string(), "Z");
    }

    #[test]
    fn test_field_descriptor() {
        let field_desc = PhoronFieldDescriptor::ArrayType {
            component_type: Box::new(PhoronFieldDescriptor::ArrayType {
                component_type: Box::new(PhoronFieldDescriptor::ArrayType {
                    component_type: Box::new(BaseType(Double)),
                }),
            }),
        };
        assert_eq!("[[[D", &field_desc.to_string());
    }

    #[test]
    fn test_method_decriptor() {
        let method_desc = PhoronMethodDescriptor {
            param_descriptor: vec![],
            return_descriptor: VoidDescriptor,
        };
        assert_eq!("()V", &method_desc.to_string());

        let method_desc = PhoronMethodDescriptor {
            param_descriptor: vec![
                BaseType(Integer),
                BaseType(Double),
                ObjectType {
                    class_name: "java/lang/Thread".to_string(),
                },
            ],
            return_descriptor: FieldDescriptor(ObjectType {
                class_name: "java/lang/Object".to_string(),
            }),
        };
        assert_eq!(
            "(IDLjava/lang/Thread;)Ljava/lang/Object;",
            &method_desc.to_string()
        );
    }
}
