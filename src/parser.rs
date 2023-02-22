use crate::{
    ast::*,
    lexer::{Lexer, LexerError, Token},
};

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum DirectiveError {
    ThrowsMissingClassName,

    VarMissingVarnum,
    VarMissingIsKeyword,
    VarMissingName,
    VarMissingFieldDescriptor,
    VarMissingFromKeyword,
    VarMissingFromLabel,
    VarMissingToKeyword,
    VarMissingToLabel,

    InvalidDirective(String),
    LocalsDirectiveMissingCount,
    MissingStackOrLocal,
    StackDirectiveMissingCount,
}

impl fmt::Display for DirectiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                DirectiveError::ThrowsMissingClassName => ".throws - missing class name".into(),
                DirectiveError::VarMissingVarnum => ".var - missing var num".into(),
                DirectiveError::VarMissingIsKeyword => ".var - missing is keyword".into(),
                DirectiveError::VarMissingName => ".var - missing name".into(),
                DirectiveError::VarMissingFieldDescriptor =>
                    ".var - missing field descriptor".into(),
                DirectiveError::VarMissingFromKeyword => ".var - missing from keyword".into(),
                DirectiveError::VarMissingFromLabel => ".var - missing from label".into(),
                DirectiveError::VarMissingToKeyword => ".var - missing to keyword".into(),
                DirectiveError::VarMissingToLabel => ".var - missing to label".into(),

                DirectiveError::StackDirectiveMissingCount =>
                    ".limit stack : missing count value for .limit stack directive".into(),
                DirectiveError::InvalidDirective(ref tok) => format!("invalid directive {tok}"),
                DirectiveError::LocalsDirectiveMissingCount =>
                    "missing count value for .limit locals directive".into(),
                DirectiveError::MissingStackOrLocal =>
                    "directive limit : missing `stack` or `locals`".into(),
            }
        )
    }
}

#[derive(Debug)]
pub enum JvmInstructionError {
    LstoreMissingOrInvalidVarnum,
    LloadMissingOrInvalidVarnum,
    InstanceofMissingOrInvalidCheckType,
    IfnonnullMissingOrInvalidLabel,
    IfnullMissingOrInvalidLabel,
    IfleMissingOrInvalidLabel,
    IfltMissingOrInvalidLabel,
    IfgeMissingOrInvalidLabel,
    IfgtMissingOrInvalidLabel,
    IfcmpneMissingOrInvalidLabel,
    IfeqMissingOrInvalidLabel,
    IficmpgtMissingLabel,
    IficmpltMissingLabel,
    IficmpleMissingLabel,
    IloadMissingVarnum,
    IfneMissingLabel,
    IficmpeqMissingOrInvalidLabel,
    IficmpgeMissingOrInvalidLabel,
    IfacmpneMissingOrInvalidLabel,
    IfacmpeqMissingOrInvalidLabel,
    GetfieldMissingOrInvalidClassName,
    GetfieldMissingOrInvalidFieldName,
    GetfieldMissingOrInvalidFieldDescriptor,
    FstoreMissingOrInvalidVarnum,
    FloadMissingOrInvalidVarnum,
    DstoreMissingOrInvalidVarnum,
    DloadMissingOrInvalidVarnum,
    AnewarrayMissingOrInvalidComponentType,
    AloadMissingVarnum,
    AstoreMissingVarnum,
    IstoreMissingVarnum,

    TableswitchMissingLow,
    TableswitchMissingHigh,
    TableswitchMissingDefault,

    LookupswitchInvalidDefault,
    LookupswitchMissingDefault,
    LookupswitchInvalidSwitchEntry,
    LookupswitchMissingLabelforSwitchEntry,

    CheckcastInvalidOrMissingType,
    RetMissingVarnum,
    JsrMissingLabel,
    JsrwMissingLabel,
    GotoMissingLabel,
    GotowMissingLabel,
    SipushMissingConstant,
    AnewarrayInvalidTypeDescriptor(String),
    BipushMissingByte,
    GetfieldInvalid,
    GetfieldInvalidOrMissingFieldDescriptor,
    GetfieldMissingFieldName,
    GetstaticMissingOrInvalidClassName,
    GetstaticInvalidOrMissingFieldDescriptor,
    GetstaticMissingOrInvalidFieldName,
    IincMissingOrInvalidVarnum,
    IincMissingOrInvalidDelta,
    InvokespecialInvalid,
    InvokespecialInvalidOrMissingMethodDescriptor,
    InvokespecialMissingMethodName,
    InvokestaticInvalid,
    InvokestaticInvalidOrMissingMethodDescriptor,
    InvokestaticMissingMethodName,
    InvokevirtualInvalid,
    InvokevirtualInvalidOrMissingMethodDescriptor,
    InvokevirtualMissingMethodName,
    LdcIncorrectValue,
    LdcwIncorrectValue,
    Ldc2wIncorrectValue,
    MultianewarrayMissingDimensions,
    NewMissingClassName,
    NewarrayInvalidType,
}

impl fmt::Display for JvmInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                JvmInstructionError::LstoreMissingOrInvalidVarnum =>
                    "lstore : missing or invalid var num".into(),
                JvmInstructionError::LloadMissingOrInvalidVarnum =>
                    "lload : missing or invalid var num".into(),
                JvmInstructionError::InstanceofMissingOrInvalidCheckType =>
                    "instanceof : missing or invalid check type".into(),
                JvmInstructionError::IfeqMissingOrInvalidLabel =>
                    "ifeq : missing or invalid label".into(),
                JvmInstructionError::IfcmpneMissingOrInvalidLabel =>
                    "if_cmpne : missing or invalid label".into(),
                JvmInstructionError::IfnonnullMissingOrInvalidLabel =>
                    "if_nonnull : missing or invalid label".into(),
                JvmInstructionError::IfnullMissingOrInvalidLabel =>
                    "if_null : missing or invalid label".into(),
                JvmInstructionError::IfleMissingOrInvalidLabel =>
                    "ifle : missing or invalid label".into(),
                JvmInstructionError::IfltMissingOrInvalidLabel =>
                    "iflt : missing or invalid label".into(),
                JvmInstructionError::IfgeMissingOrInvalidLabel =>
                    "ifge : missing or invalid label".into(),
                JvmInstructionError::IfgtMissingOrInvalidLabel =>
                    "ifgt : missing or invalid label".into(),
                JvmInstructionError::IficmpeqMissingOrInvalidLabel =>
                    "if_icmpeq : missing or invalid label".into(),
                JvmInstructionError::IficmpgeMissingOrInvalidLabel =>
                    "if_icmpge : missing or invalid label".into(),
                JvmInstructionError::IfacmpneMissingOrInvalidLabel =>
                    "if_acmpne : missing or invalid label".into(),
                JvmInstructionError::IfacmpeqMissingOrInvalidLabel =>
                    "if_acmpeq : missing or invalid label".into(),
                JvmInstructionError::GetfieldMissingOrInvalidClassName =>
                    "getfield : missing or invalid class name".into(),
                JvmInstructionError::GetfieldMissingOrInvalidFieldName =>
                    "getfield : missing or invalid field name".into(),
                JvmInstructionError::GetfieldMissingOrInvalidFieldDescriptor =>
                    "getfield : missing or invalid field descriptor".into(),
                JvmInstructionError::FstoreMissingOrInvalidVarnum =>
                    "fstore : missing or invalid var num".into(),
                JvmInstructionError::FloadMissingOrInvalidVarnum =>
                    "fload : missing or invalid var num".into(),
                JvmInstructionError::DstoreMissingOrInvalidVarnum =>
                    "dstore : missing or invalod var num".into(),
                JvmInstructionError::DloadMissingOrInvalidVarnum =>
                    "dload : missing or invalid var num".into(),
                JvmInstructionError::AnewarrayMissingOrInvalidComponentType =>
                    "anewarray : missing or invalid component type".into(),
                JvmInstructionError::AloadMissingVarnum => "aload - missing varn num".into(),
                JvmInstructionError::TableswitchMissingLow => "tableswitch : missing low".into(),
                JvmInstructionError::TableswitchMissingHigh => "tableswitch : missing high".into(),
                JvmInstructionError::TableswitchMissingDefault =>
                    "tableswitch : missing default".into(),
                JvmInstructionError::LookupswitchInvalidDefault =>
                    "lookupswitch : invalid default".into(),
                JvmInstructionError::LookupswitchInvalidSwitchEntry =>
                    "lookupswitch : invalid entry".into(),
                JvmInstructionError::LookupswitchMissingDefault =>
                    "lookupswitch : missing default".into(),
                JvmInstructionError::LookupswitchMissingLabelforSwitchEntry =>
                    "lookupswitch : missing label for switch entry".into(),
                JvmInstructionError::CheckcastInvalidOrMissingType =>
                    "checkcast :  invalid or missing type".into(),
                JvmInstructionError::RetMissingVarnum => "ret : missing var num".into(),
                JvmInstructionError::JsrMissingLabel => "jsr : missing label".into(),
                JvmInstructionError::JsrwMissingLabel => "jsrw : missing label".into(),
                JvmInstructionError::GotoMissingLabel => "goto : missing label".into(),
                JvmInstructionError::GotowMissingLabel => "gotow : missing label".into(),
                JvmInstructionError::SipushMissingConstant =>
                    "sipush : missing constant value".into(),
                JvmInstructionError::IficmpgtMissingLabel => "if_icmpgt : missing label".into(),
                JvmInstructionError::IficmpltMissingLabel => "if_icmplt : missing label".into(),
                JvmInstructionError::IficmpleMissingLabel => "if_icmple : missing label".into(),
                JvmInstructionError::IloadMissingVarnum => "iload : missing var number".into(),
                JvmInstructionError::IfneMissingLabel => "ifne : missing label".into(),
                JvmInstructionError::IincMissingOrInvalidVarnum =>
                    "iinc : missing or invalid var num".into(),
                JvmInstructionError::IincMissingOrInvalidDelta =>
                    "iinc : missing or invalid delta".into(),
                JvmInstructionError::AnewarrayInvalidTypeDescriptor(ref err_type) =>
                    format!("anewarray : incorrect type - {err_type}"),
                JvmInstructionError::InvokespecialInvalid =>
                    "invokespecial: badly-formed or invalid".into(),
                JvmInstructionError::InvokespecialMissingMethodName =>
                    "invokespecial : missing method name".into(),
                JvmInstructionError::InvokespecialInvalidOrMissingMethodDescriptor =>
                    "invokespecial : missing or invalid method descriptor".into(),
                JvmInstructionError::InvokevirtualInvalid =>
                    "invokevirtual: badly-formed or invalid".into(),
                JvmInstructionError::InvokevirtualMissingMethodName =>
                    "invokevirtual : missing method name".into(),
                JvmInstructionError::InvokevirtualInvalidOrMissingMethodDescriptor =>
                    "invokevirtual : missing or invalid method descriptor".into(),
                JvmInstructionError::InvokestaticInvalid =>
                    "invokestatic: badly-formed or invalid".into(),
                JvmInstructionError::InvokestaticMissingMethodName =>
                    "invokestatic : missing method name".into(),
                JvmInstructionError::InvokestaticInvalidOrMissingMethodDescriptor =>
                    "invokestatic : missing or invalid method descriptor".into(),
                JvmInstructionError::BipushMissingByte => "Bipush : missing byte".into(),
                JvmInstructionError::GetfieldInvalid => "getfield : badly-formed or invalid".into(),
                JvmInstructionError::GetfieldInvalidOrMissingFieldDescriptor =>
                    "getfield : missing or invalid field descriptor".into(),
                JvmInstructionError::GetfieldMissingFieldName =>
                    "getfield : missing field name".into(),
                JvmInstructionError::GetstaticMissingOrInvalidClassName =>
                    "getstatic : missing or invalid class name".into(),
                JvmInstructionError::GetstaticInvalidOrMissingFieldDescriptor =>
                    "getstatic : missing or invalid field descriptor".into(),
                JvmInstructionError::GetstaticMissingOrInvalidFieldName =>
                    "getstatic : missing or invalid field name".into(),
                JvmInstructionError::LdcIncorrectValue => "ldc : incorrect value".into(),
                JvmInstructionError::LdcwIncorrectValue => "ldcw : incorrect value".into(),
                JvmInstructionError::Ldc2wIncorrectValue => "ldc2_w : incorrect value".into(),
                JvmInstructionError::MultianewarrayMissingDimensions =>
                    "multianewarray : missing dimensions".into(),
                JvmInstructionError::NewMissingClassName => "newa : missing class name".into(),
                JvmInstructionError::NewarrayInvalidType => "newarray : invalid type".into(),

                JvmInstructionError::IstoreMissingVarnum => "istore : missing var num".into(),
                JvmInstructionError::AstoreMissingVarnum => "astore : missing var num".into(),
            }
        )
    }
}

#[derive(Debug)]
pub enum ParserError {
    FailedToParsePrimitive,
    MissingDefaultKeyword,
    DirectiveError(DirectiveError),
    IncorrectToken(String, String),
    EmptyFieldDescriptor,
    IllegalLabelError,
    InvalidDirective(String),
    InvalidFieldDescriptor,
    InvalidFieldInitValue(String),
    InvalidInstruction(String),
    InvalidJvmInstruction(String),
    InvalidMethodDescriptor,
    InvalidToken(String),
    JvmInstructionError(JvmInstructionError),
    LexerError(LexerError),
    MissingSourceFileName,
    MissingClassName,
    MissingLabel,
    MissingEndMethodMarker,
    MissingFieldName,
    MissingInterfaceName,
    MissingMethodName,
    MissingSuperclassName,
    UnknownClassOrInterfaceAccessFlag(String),
    UnknownFieldAccessFlag(String),
    UnknownMethodAccessFlag(String),
}

impl std::error::Error for ParserError {}

use std::fmt;
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ParserError::FailedToParsePrimitive => "failed to parse primitive value".into(),
                ParserError::MissingDefaultKeyword => "missing default keyword".into(),
                ParserError::IncorrectToken(ref expected, ref actual) =>
                    format!("token mismatch while parsing: expected {expected} but found {actual}"),
                ParserError::LexerError(ref err) => err.to_string(),
                ParserError::IllegalLabelError => "malformed label - missing semicolon".into(),
                ParserError::InvalidToken(ref tokstr) =>
                    format!("Invalid token at position: {:#?}", tokstr),
                ParserError::InvalidFieldDescriptor => "invalid field descriptor".into(),
                ParserError::InvalidDirective(ref dir) => format!("invalid directive: {dir}"),
                ParserError::EmptyFieldDescriptor => "empty field descriptor".into(),
                ParserError::InvalidFieldInitValue(ref tokstr) =>
                    format!("invalid field initialisation value: {:#?}", tokstr),
                ParserError::InvalidMethodDescriptor => "invalid method descriptor".into(),
                ParserError::InvalidInstruction(ref instr) =>
                    format!("invalid phpron instuction: {instr}"),
                ParserError::InvalidJvmInstruction(ref instr) =>
                    format!("invalid JVM instruction {instr}"),
                ParserError::MissingSourceFileName => "missing source file name".into(),
                ParserError::MissingClassName => "missing class name".into(),
                ParserError::MissingLabel => "missing label".into(),
                ParserError::MissingInterfaceName => "missing interface name".into(),
                ParserError::MissingSuperclassName => "missing super class name".into(),
                ParserError::MissingFieldName => "field name missing in definition".into(),
                ParserError::MissingMethodName => "method name missing in definition".into(),
                ParserError::MissingEndMethodMarker =>
                    "malformed end method - missing method keyword".into(),
                ParserError::UnknownClassOrInterfaceAccessFlag(ref flag) =>
                    format!("Invalid flag for class/interface: {flag}"),
                ParserError::UnknownFieldAccessFlag(ref flag) =>
                    format!("Invalid flag for field: {flag}"),
                ParserError::UnknownMethodAccessFlag(ref flag) =>
                    format!("Invalid flag for method: {flag}"),
                ParserError::JvmInstructionError(ref jvm_err) => jvm_err.to_string(),
                ParserError::DirectiveError(ref dir_err) => dir_err.to_string(),
            }
        )
    }
}

impl From<LexerError> for ParserError {
    fn from(lex_err: LexerError) -> Self {
        ParserError::LexerError(lex_err)
    }
}

/// The Phoron parser
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_tok: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer,
            curr_tok: Token::TEof,
        }
    }

    fn advance(&mut self) -> ParserResult<()> {
        self.curr_tok = self.lexer.lex()?;
        Ok(())
    }

    fn advance_if(&mut self, expected_token: &Token) -> ParserResult<()> {
        if self.see() != expected_token {
            Err(ParserError::IncorrectToken(
                format!("{:#?}", expected_token),
                format!("{:#?}", self.see()),
            ))
        } else {
            self.advance()?;
            Ok(())
        }
    }

    fn see(&self) -> &Token {
        &self.curr_tok
    }

    fn is_class_or_interface_access_flag(&self, tok: &Token) -> bool {
        match tok {
            Token::TPublic
            | Token::TFinal
            | Token::TSuper
            | Token::TInterface
            | Token::TAbstract
            | Token::TSynthetic
            | Token::TAnnotation
            | Token::TEnum
            | Token::TModule => true,
            _ => false,
        }
    }

    fn is_field_access_flag(&self, tok: &Token) -> bool {
        match tok {
            Token::TPublic
            | Token::TPrivate
            | Token::TProtected
            | Token::TStatic
            | Token::TFinal
            | Token::TVolatile
            | Token::TTransient
            | Token::TSynthetic
            | Token::TEnum => true,
            _ => false,
        }
    }

    fn is_method_access_flag(&self, tok: &Token) -> bool {
        match tok {
            Token::TPublic
            | Token::TPrivate
            | Token::TProtected
            | Token::TStatic
            | Token::TFinal
            | Token::TSynthetic
            | Token::TSynchronized
            | Token::TBridge
            | Token::TVarargs
            | Token::TNative
            | Token::TAbstract
            | Token::TStrict => true,
            _ => false,
        }
    }

    fn get_class_or_interface_access_flag(
        &self,
        tok: &Token,
    ) -> ParserResult<PhoronClassOrInterfaceAccessFlag> {
        Ok(match tok {
            Token::TPublic => PhoronClassOrInterfaceAccessFlag::AccPublic,
            Token::TFinal => PhoronClassOrInterfaceAccessFlag::AccFinal,
            Token::TSuper => PhoronClassOrInterfaceAccessFlag::AccSuper,
            Token::TInterface => PhoronClassOrInterfaceAccessFlag::AccInterface,
            Token::TAbstract => PhoronClassOrInterfaceAccessFlag::AccAbstract,
            Token::TSynthetic => PhoronClassOrInterfaceAccessFlag::AccSynthetic,
            Token::TAnnotation => PhoronClassOrInterfaceAccessFlag::AccAnnotation,
            Token::TEnum => PhoronClassOrInterfaceAccessFlag::AccEnum,
            Token::TModule => PhoronClassOrInterfaceAccessFlag::AccModule,
            _ => {
                return Err(ParserError::UnknownClassOrInterfaceAccessFlag(format!(
                    "{:#?}",
                    tok
                )))
            }
        })
    }

    fn get_field_access_flags(&self, tok: &Token) -> ParserResult<PhoronFieldAccessFlag> {
        Ok(match tok {
            Token::TPublic => PhoronFieldAccessFlag::AccPublic,
            Token::TPrivate => PhoronFieldAccessFlag::AccPrivate,
            Token::TProtected => PhoronFieldAccessFlag::AccProtected,
            Token::TStatic => PhoronFieldAccessFlag::AccStatic,
            Token::TFinal => PhoronFieldAccessFlag::AccFinal,
            Token::TVolatile => PhoronFieldAccessFlag::AccVolatile,
            Token::TTransient => PhoronFieldAccessFlag::AccTransient,
            Token::TSynthetic => PhoronFieldAccessFlag::AccSynthetic,
            Token::TEnum => PhoronFieldAccessFlag::AccEnum,
            _ => return Err(ParserError::UnknownFieldAccessFlag(format!("{:#?}", tok))),
        })
    }

    fn get_method_acess_flags(&self, tok: &Token) -> ParserResult<PhoronMethodAccessFlag> {
        Ok(match tok {
            Token::TPublic => PhoronMethodAccessFlag::AccPublic,
            Token::TPrivate => PhoronMethodAccessFlag::AccPrivate,
            Token::TProtected => PhoronMethodAccessFlag::AccProtected,
            Token::TStatic => PhoronMethodAccessFlag::AccStatic,
            Token::TSynchronized => PhoronMethodAccessFlag::AccSynchronized,
            Token::TFinal => PhoronMethodAccessFlag::AccFinal,
            Token::TBridge => PhoronMethodAccessFlag::AccBridge,
            Token::TVarargs => PhoronMethodAccessFlag::AccVarargs,
            Token::TNative => PhoronMethodAccessFlag::AccNative,
            Token::TAbstract => PhoronMethodAccessFlag::AccAbstract,
            Token::TStrict => PhoronMethodAccessFlag::AccStrict,
            Token::TSynthetic => PhoronMethodAccessFlag::AccSynthetic,
            _ => return Err(ParserError::UnknownMethodAccessFlag(format!("{:#?}", tok))),
        })
    }

    /// ClassDef <- CLASS_keyword AccessFlag* ClassName newline
    fn parse_class_def(&mut self) -> ParserResult<PhoronClassDef> {
        self.advance()?;

        let mut access_flags = Vec::new();

        Ok(match self.see() {
            Token::TPublic
            | Token::TFinal
            | Token::TSuper
            | Token::TInterface
            | Token::TAbstract
            | Token::TSynthetic
            | Token::TAnnotation
            | Token::TEnum
            | Token::TModule => {
                while self.is_class_or_interface_access_flag(self.see()) {
                    access_flags.push(self.get_class_or_interface_access_flag(&self.see())?);
                    self.advance()?;
                }

                if let Token::TIdent(ident) = self.see() {
                    let name = ident.to_string();
                    self.advance()?;

                    PhoronClassDef { name, access_flags }
                } else {
                    return Err(ParserError::MissingClassName);
                }
            }

            Token::TIdent(ident) => {
                let name = ident.to_string();
                PhoronClassDef { name, access_flags }
            }

            tok => return Err(ParserError::InvalidToken(format!("{tok:#?}"))),
        })
    }

    /// InterfaceDef <- INTERFACE_keyword AccessFlag* ClassName newline
    fn parse_interface_def(&mut self) -> ParserResult<PhoronInterfaceDef> {
        self.advance()?;

        let mut access_flags = Vec::new();

        Ok(match self.see() {
            Token::TPublic
            | Token::TFinal
            | Token::TSuper
            | Token::TInterface
            | Token::TAbstract
            | Token::TSynthetic
            | Token::TAnnotation
            | Token::TEnum
            | Token::TModule => {
                while self.is_class_or_interface_access_flag(self.see()) {
                    access_flags.push(self.get_class_or_interface_access_flag(&self.see())?);
                    self.advance()?;
                }

                if let Token::TIdent(ident) = self.see() {
                    let name = ident.to_string();
                    self.advance()?;

                    PhoronInterfaceDef { name, access_flags }
                } else {
                    return Err(ParserError::MissingInterfaceName);
                }
            }

            Token::TIdent(ident) => {
                let name = ident.to_string();
                PhoronInterfaceDef { name, access_flags }
            }

            tok => return Err(ParserError::InvalidToken(format!("{tok:#?}"))),
        })
    }

    /// SuperDef <- SUPER_keyword ClassName newline
    fn parse_super_def(&mut self) -> ParserResult<PhoronSuperDef> {
        self.advance()?;

        if let Token::TIdent(ident) = self.see() {
            let super_class_name = ident.to_string();
            self.advance()?;

            Ok(PhoronSuperDef { super_class_name })
        } else {
            Err(ParserError::MissingSuperclassName)
        }
    }

    /// FieldIniValue <- Double / Integer / QuotedString
    fn parse_field_init_value(&mut self) -> ParserResult<Option<PhoronFieldInitValue>> {
        if let Token::TAssign = self.see() {
            self.advance()?;

            Ok(if let Token::TInt(int) = self.see() {
                let ival = *int;
                self.advance()?;
                Some(PhoronFieldInitValue::Integer(ival))
            } else if let Token::TFloat(float) = self.see() {
                let fval = *float;
                self.advance()?;
                Some(PhoronFieldInitValue::Double(fval))
            } else if let Token::TString(s) = self.see() {
                let sval = s.to_owned();
                self.advance()?;
                Some(PhoronFieldInitValue::QuotedString(sval))
            } else {
                return Err(ParserError::InvalidFieldInitValue(format!(
                    "{:#?}",
                    self.see()
                )));
            })
        } else {
            Ok(None)
        }
    }

    /// AnewarrayTypeDescriptor <- ClassType / ArrayType
    /// ClassType <- ClassName
    /// ArrayType <- '[' AnewarrayTypeDescriptor ';'
    fn parse_class_or_array_type(&mut self) -> ParserResult<ClassOrArrayTypeDescriptor> {
        Ok(match self.see() {
            Token::TLeftSquareBracket => {
                self.advance()?;
                let component_type = self.parse_class_or_array_type()?;

                ClassOrArrayTypeDescriptor::ArrayType {
                    component_type: Box::new(component_type),
                }
            }
            Token::TIdent(class_name_str) => {
                let class_name = class_name_str.to_string();
                self.advance()?;

                ClassOrArrayTypeDescriptor::ClassType { class_name }
            }

            _ => {
                return Err(ParserError::JvmInstructionError(
                    JvmInstructionError::AnewarrayInvalidTypeDescriptor(format!(
                        "{:#?}",
                        self.see()
                    )),
                ))
            }
        })
    }

    /// FieldDescriptor <- FieldType
    /// FieldType <- BaseType / ObjectType / ArrayType
    /// BaseType <- 'B' / 'C' / 'D' / 'F' / 'I' / 'J' / 'S' / 'Z'
    /// ObjectType <- 'L' ClassName ';'
    /// ArrayType <- '[' ComponentType
    /// ComponentType <- FieldType
    fn parse_field_descriptor(&mut self) -> ParserResult<PhoronFieldDescriptor> {
        let ident_tok = self.see();

        if let Token::TIdent(ident) = ident_tok {
            Ok(match &ident[0..1] {
                "B" => {
                    self.advance()?;
                    PhoronFieldDescriptor::BaseType(PhoronBaseType::Byte)
                }
                "C" => {
                    self.advance()?;
                    PhoronFieldDescriptor::BaseType(PhoronBaseType::Character)
                }
                "D" => {
                    self.advance()?;
                    PhoronFieldDescriptor::BaseType(PhoronBaseType::Double)
                }
                "F" => {
                    self.advance()?;
                    PhoronFieldDescriptor::BaseType(PhoronBaseType::Float)
                }
                "I" => {
                    self.advance()?;
                    PhoronFieldDescriptor::BaseType(PhoronBaseType::Integer)
                }
                "J" => {
                    self.advance()?;
                    PhoronFieldDescriptor::BaseType(PhoronBaseType::Long)
                }
                "S" => {
                    self.advance()?;
                    PhoronFieldDescriptor::BaseType(PhoronBaseType::Short)
                }
                "Z" => {
                    self.advance()?;
                    PhoronFieldDescriptor::BaseType(PhoronBaseType::Boolean)
                }
                "L" => {
                    let class_name = ident[1..ident.len() - 1].to_string();

                    if ident.chars().last().unwrap() == ';' {
                        self.advance()?;
                        PhoronFieldDescriptor::ObjectType { class_name }
                    } else {
                        return Err(ParserError::InvalidFieldDescriptor);
                    }
                }
                _ => return Err(ParserError::InvalidFieldDescriptor),
            })
        } else if let Token::TLeftSquareBracket = self.see() {
            self.advance()?;
            let component_type = self.parse_field_descriptor()?;
            Ok(PhoronFieldDescriptor::ArrayType {
                component_type: Box::new(component_type),
            })
        } else if let Token::TRightParen = self.see() {
            Err(ParserError::EmptyFieldDescriptor)
        } else {
            Err(ParserError::InvalidFieldDescriptor)
        }
    }

    /// FieldDef <- line_comment* FIELD_keyword FieldAccessFlag* FieldName FieldDescriptor (EQ_symbol FieldIniValue)? newline
    fn parse_field_def(&mut self) -> ParserResult<PhoronFieldDef> {
        self.advance()?;

        let mut access_flags = Vec::new();
        while self.is_field_access_flag(self.see()) {
            access_flags.push(self.get_field_access_flags(&self.see())?);
            self.advance()?;
        }

        if let Token::TIdent(ident) = self.see() {
            let name = ident.to_string();
            self.advance()?;

            let field_descriptor = self.parse_field_descriptor()?;
            let init_val = self.parse_field_init_value()?;

            Ok(PhoronFieldDef {
                name,
                access_flags,
                field_descriptor,
                init_val,
            })
        } else {
            Err(ParserError::MissingFieldName)
        }
    }

    fn parse_field_defs(&mut self) -> ParserResult<Vec<PhoronFieldDef>> {
        let mut field_defs = Vec::new();
        while let Token::TField = self.see() {
            field_defs.push(self.parse_field_def()?);
        }

        Ok(field_defs)
    }

    fn parse_class_name(&mut self) -> ParserResult<String> {
        if let Token::TIdent(classname) = self.see() {
            let classname = classname.to_owned();
            self.advance()?;

            Ok(classname)
        } else {
            Err(ParserError::MissingClassName)
        }
    }

    fn parse_label(&mut self) -> ParserResult<String> {
        if let Token::TIdent(label) = self.see() {
            let label = label.to_owned();
            self.advance()?;

            Ok(label)
        } else {
            Err(ParserError::MissingLabel)
        }
    }

    /// Directive <- (LIMIT_keyword (StackDirective / LocalDirective) / ThrowsDirective / LineNumberDirective / VarDirective / CatchDirective) newline
    /// StackDirective <-  STACK_keyword Integer
    /// LocalDirective <- LOCAL_keyword Integer
    /// ThrowsDirective <- THROWS_keyword ClassName
    /// LineNumberDirective <- LINE_keyword Integer
    /// VarDirective <- VAR_keyword Integer IS_keyword VarName FieldDescriptor FROM_keyword Label TO_keyword Label
    /// CatchDirective <- CATCH_keyword ClassName FROM_keyword Label TO_keyword Label USING_keyword Label
    fn parse_directive(&mut self) -> ParserResult<PhoronDirective> {
        Ok(match self.see() {
            Token::TLimit => {
                self.advance()?;

                match self.see() {
                    Token::TStack => {
                        self.advance()?;

                        if let Token::TInt(n) = self.see() {
                            let max_stack = *n as u16;
                            self.advance()?;
                            PhoronDirective::LimitStack(max_stack)
                        } else {
                            return Err(ParserError::DirectiveError(
                                DirectiveError::StackDirectiveMissingCount,
                            ));
                        }
                    }

                    Token::TLocals => {
                        self.advance()?;

                        if let Token::TInt(n) = self.see() {
                            let max_locals = *n as u16;
                            self.advance()?;
                            PhoronDirective::LimitLocals(max_locals)
                        } else {
                            return Err(ParserError::DirectiveError(
                                DirectiveError::LocalsDirectiveMissingCount,
                            ));
                        }
                    }
                    _ => {
                        return Err(ParserError::DirectiveError(
                            DirectiveError::MissingStackOrLocal,
                        ))
                    }
                }
            }

            Token::TThrows => {
                self.advance()?;

                let class_name = self.parse_class_name().map_err(|_| {
                    ParserError::DirectiveError(DirectiveError::ThrowsMissingClassName)
                })?;

                PhoronDirective::Throws { class_name }
            }
            Token::TLine => {
                todo!()
            }

            Token::TVar => {
                self.advance()?;

                let varnum = self
                    .parse_u16()
                    .map_err(|_| ParserError::DirectiveError(DirectiveError::VarMissingVarnum))?;

                self.advance_if(&Token::TIs).map_err(|_| {
                    ParserError::DirectiveError(DirectiveError::VarMissingIsKeyword)
                })?;

                let name = self
                    .parse_label()
                    .map_err(|_| ParserError::DirectiveError(DirectiveError::VarMissingName))?;

                let field_descriptor = self.parse_field_descriptor().map_err(|_| {
                    ParserError::DirectiveError(DirectiveError::VarMissingFieldDescriptor)
                })?;

                self.advance_if(&Token::TFrom).map_err(|_| {
                    ParserError::DirectiveError(DirectiveError::VarMissingFromKeyword)
                })?;

                let from_label = self.parse_label().map_err(|_| {
                    ParserError::DirectiveError(DirectiveError::VarMissingFromLabel)
                })?;

                self.advance_if(&Token::TTo).map_err(|_| {
                    ParserError::DirectiveError(DirectiveError::VarMissingToKeyword)
                })?;

                let to_label = self
                    .parse_label()
                    .map_err(|_| ParserError::DirectiveError(DirectiveError::VarMissingToLabel))?;

                PhoronDirective::Var {
                    varnum,
                    name,
                    field_descriptor,
                    from_label,
                    to_label,
                }
            }

            Token::TCatch => {
                self.advance()?;

                let class_name = self.parse_class_name()?;
                self.advance_if(&Token::TFrom)?;
                let from_label = self.parse_label()?;
                self.advance_if(&Token::TTo)?;
                let to_label = self.parse_label()?;
                self.advance_if(&Token::TUsing)?;
                let handler_label = self.parse_label()?;

                PhoronDirective::Catch {
                    class_name,
                    from_label,
                    to_label,
                    handler_label,
                }
            }
            _ => {
                return Err(ParserError::DirectiveError(
                    DirectiveError::InvalidDirective(format!("{:#?}", self.see())),
                ))
            }
        })
    }

    fn parse_i8(&mut self) -> ParserResult<i8> {
        if let Token::TInt(n) = self.see() {
            let n = *n as i8;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::FailedToParsePrimitive)
        }
    }

    fn parse_i16(&mut self) -> ParserResult<i16> {
        if let Token::TInt(n) = self.see() {
            let n = *n as i16;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::FailedToParsePrimitive)
        }
    }

    fn parse_u16(&mut self) -> ParserResult<u16> {
        if let Token::TInt(n) = self.see() {
            let n = *n as u16;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::FailedToParsePrimitive)
        }
    }

    fn parse_i32(&mut self) -> ParserResult<i32> {
        if let Token::TInt(n) = self.see() {
            let n = *n as i32;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::FailedToParsePrimitive)
        }
    }

    fn parse_table_switches(&mut self) -> ParserResult<Vec<String>> {
        let mut switches = Vec::new();

        while let Token::TIdent(label) = self.see() {
            let label = label.to_string();
            self.advance()?;

            switches.push(label);
        }

        Ok(switches)
    }

    fn parse_lookup_switches(&mut self) -> ParserResult<Vec<LookupSwitchPair>> {
        let mut switches = Vec::new();

        while let Token::TInt(key) = self.see() {
            let key = *key as i32;
            self.advance()?;

            if let Token::TColon = self.see() {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::LookupswitchMissingLabelforSwitchEntry,
                    )
                })?;
                switches.push(LookupSwitchPair { key, label })
            } else {
                return Err(ParserError::JvmInstructionError(
                    JvmInstructionError::LookupswitchInvalidSwitchEntry,
                ));
            }
        }

        Ok(switches)
    }

    fn parse_default_switch_pair(&mut self) -> ParserResult<String> {
        if let Token::TDefault = self.see() {
            self.advance()?;

            if let Token::TColon = self.see() {
                self.advance()?;

                let label = self.parse_label()?;
                Ok(label)
            } else {
                Err(ParserError::JvmInstructionError(
                    JvmInstructionError::LookupswitchInvalidDefault,
                ))
            }
        } else {
            Err(ParserError::MissingDefaultKeyword)
        }
    }

    fn parse_jvm_instruction(&mut self) -> ParserResult<JvmInstruction> {
        Ok(match self.see() {
            // aaload
            Token::TAaload => {
                self.advance()?;
                JvmInstruction::Aaload
            }

            // aastore
            Token::TAastore => {
                self.advance()?;
                JvmInstruction::Aastore
            }

            // aconst_null
            Token::TAconstnull => {
                self.advance()?;
                JvmInstruction::Aconstnull
            }

            // aload <varnum>
            Token::TAload => {
                self.advance()?;
                let varnum = self.parse_u16().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::AloadMissingVarnum)
                })?;
                JvmInstruction::Aload { varnum }
            }

            // aload_0
            Token::TAload0 => {
                self.advance()?;
                JvmInstruction::Aload0
            }

            // aload_1
            Token::TAload1 => {
                self.advance()?;
                JvmInstruction::Aload1
            }

            // aload_2
            Token::TAload2 => {
                self.advance()?;
                JvmInstruction::Aload2
            }

            // aload_3
            Token::TAload3 => {
                self.advance()?;
                JvmInstruction::Aload3
            }

            // anewarray <type>
            Token::TAnewarray => {
                self.advance()?;

                let component_type = self.parse_class_or_array_type().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::AnewarrayMissingOrInvalidComponentType,
                    )
                })?;
                JvmInstruction::Anewarray { component_type }
            }

            // areturn
            Token::TAreturn => {
                self.advance()?;
                JvmInstruction::Areturn
            }

            // arraylength
            Token::TArraylength => {
                self.advance()?;
                JvmInstruction::Arraylength
            }

            // astore <varnum>
            Token::TAstore => {
                self.advance()?;

                let varnum = self.parse_u16().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::AstoreMissingVarnum)
                })?;
                JvmInstruction::Astore { varnum }
            }

            // astore_0
            Token::TAstore0 => {
                self.advance()?;
                JvmInstruction::Astore0
            }

            // astore_1
            Token::TAstore1 => {
                self.advance()?;
                JvmInstruction::Astore1
            }

            // astore_2
            Token::TAstore2 => {
                self.advance()?;
                JvmInstruction::Astore2
            }

            // astore_3
            Token::TAstore3 => {
                self.advance()?;
                JvmInstruction::Astore3
            }

            // athrow
            Token::TAthrow => {
                self.advance()?;
                JvmInstruction::Athrow
            }

            // baload
            Token::TBaload => {
                self.advance()?;
                JvmInstruction::Baload
            }

            // bastore
            Token::TBastore => {
                self.advance()?;
                JvmInstruction::Bastore
            }

            // bipush i8
            Token::TBipush => {
                self.advance()?;
                let ub = self.parse_i8().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::BipushMissingByte)
                })?;

                JvmInstruction::Bipush(ub)
            }

            // caload
            Token::TCaload => {
                self.advance()?;
                JvmInstruction::Caload
            }

            // castore
            Token::TCastore => {
                self.advance()?;
                JvmInstruction::Castore
            }

            // checkcast <type>
            Token::TCheckcast => {
                self.advance()?;
                let cast_type = self.parse_class_or_array_type().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::CheckcastInvalidOrMissingType,
                    )
                })?;
                JvmInstruction::Checkcast { cast_type }
            }

            // d2f
            Token::TD2f => {
                self.advance()?;
                JvmInstruction::D2f
            }

            // d2i
            Token::TD2i => {
                self.advance()?;
                JvmInstruction::D2i
            }

            // d2l
            Token::TD2l => {
                self.advance()?;
                JvmInstruction::D2l
            }

            // dadd
            Token::TDadd => {
                self.advance()?;
                JvmInstruction::Dadd
            }

            // daload
            Token::TDaload => {
                self.advance()?;
                JvmInstruction::Daload
            }

            // dastore
            Token::TDastore => {
                self.advance()?;
                JvmInstruction::Dastore
            }

            // dcmpg
            Token::TDcmpg => {
                self.advance()?;
                JvmInstruction::Dcmpg
            }

            // dcmpl
            Token::TDcmpl => {
                self.advance()?;
                JvmInstruction::Dcmpl
            }

            // dconst_0
            Token::TDconst0 => {
                self.advance()?;
                JvmInstruction::Dconst0
            }

            // dconst_1
            Token::TDconst1 => {
                self.advance()?;
                JvmInstruction::Dconst1
            }

            // ddiv
            Token::TDdiv => {
                self.advance()?;
                JvmInstruction::Ddiv
            }

            // dload <arnum>
            Token::TDload => {
                self.advance()?;

                let varnum = self.parse_u16().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::DloadMissingOrInvalidVarnum,
                    )
                })?;

                JvmInstruction::Dload { varnum }
            }

            // dload_0
            Token::TDload0 => {
                self.advance()?;
                JvmInstruction::Dload0
            }

            // dload_1
            Token::TDload1 => {
                self.advance()?;
                JvmInstruction::Dload1
            }

            // dload_2
            Token::TDload2 => {
                self.advance()?;
                JvmInstruction::Dload2
            }

            // dload_3
            Token::TDload3 => {
                self.advance()?;
                JvmInstruction::Dload3
            }

            // dmul
            Token::TDmul => {
                self.advance()?;
                JvmInstruction::Dmul
            }

            // dneg
            Token::TDneg => {
                self.advance()?;
                JvmInstruction::Dneg
            }

            // drem
            Token::TDrem => {
                self.advance()?;
                JvmInstruction::Drem
            }

            // dreturn
            Token::TDreturn => {
                self.advance()?;
                JvmInstruction::Dreturn
            }

            // dstore< <varnum>
            Token::TDstore => {
                self.advance()?;

                let varnum = self.parse_u16().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::DstoreMissingOrInvalidVarnum,
                    )
                })?;

                JvmInstruction::Dstore { varnum }
            }

            // dstore_0
            Token::TDstore0 => {
                self.advance()?;
                JvmInstruction::Dstore0
            }

            // dstore_1
            Token::TDstore1 => {
                self.advance()?;
                JvmInstruction::Dstore1
            }

            // dstore_2
            Token::TDstore2 => {
                self.advance()?;
                JvmInstruction::Dstore2
            }

            // dstore_3
            Token::TDstore3 => {
                self.advance()?;
                JvmInstruction::Dstore3
            }

            // dsub
            Token::TDsub => {
                self.advance()?;
                JvmInstruction::Dsub
            }

            // dup
            Token::TDup => {
                self.advance()?;
                JvmInstruction::Dup
            }

            // dup2
            Token::TDup2 => {
                self.advance()?;
                JvmInstruction::Dup2
            }

            // dup2_x1
            Token::TDup2x1 => {
                self.advance()?;
                JvmInstruction::Dup2x1
            }

            // dup2_x2
            Token::TDup2x2 => {
                self.advance()?;
                JvmInstruction::Dup2x2
            }

            // dup_x1
            Token::TDupx1 => {
                self.advance()?;
                JvmInstruction::Dupx1
            }

            // dup_x2
            Token::TDupx2 => {
                self.advance()?;
                JvmInstruction::Dupx2
            }

            // f2d
            Token::TF2d => {
                self.advance()?;
                JvmInstruction::F2d
            }

            // f2i
            Token::TF2i => {
                self.advance()?;
                JvmInstruction::F2i
            }

            // f2l
            Token::TF2l => {
                self.advance()?;
                JvmInstruction::F2l
            }

            // fadd
            Token::TFadd => {
                self.advance()?;
                JvmInstruction::Fadd
            }

            // faload
            Token::TFaload => {
                self.advance()?;
                JvmInstruction::Faload
            }

            // fastore
            Token::TFastore => {
                self.advance()?;
                JvmInstruction::Fastore
            }

            // fcmpg
            Token::TFcmpg => {
                self.advance()?;
                JvmInstruction::Fcmpg
            }

            // fcmpl
            Token::TFcmpl => {
                self.advance()?;
                JvmInstruction::Fcmpl
            }

            // fconst_0
            Token::TFconst0 => {
                self.advance()?;
                JvmInstruction::Fconst0
            }

            // fconst_1
            Token::TFconst1 => {
                self.advance()?;
                JvmInstruction::Fconst1
            }

            // fconst_2
            Token::TFconst2 => {
                self.advance()?;
                JvmInstruction::Fconst2
            }

            // fdiv
            Token::TFdiv => {
                self.advance()?;
                JvmInstruction::Fdiv
            }

            // fload <varnum>
            Token::TFload => {
                self.advance()?;

                let varnum = self.parse_u16().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::FloadMissingOrInvalidVarnum,
                    )
                })?;
                JvmInstruction::Fload { varnum }
            }

            // fload_0
            Token::TFload0 => {
                self.advance()?;
                JvmInstruction::Fload0
            }

            // fload_1
            Token::TFload1 => {
                self.advance()?;
                JvmInstruction::Fload1
            }

            // fload_2
            Token::TFload2 => {
                self.advance()?;
                JvmInstruction::Fload2
            }

            // fload_3
            Token::TFload3 => {
                self.advance()?;
                JvmInstruction::Fload3
            }

            // fmul
            Token::TFmul => {
                self.advance()?;
                JvmInstruction::Fmul
            }

            // fneg
            Token::TFneg => {
                self.advance()?;
                JvmInstruction::Fneg
            }

            // frem
            Token::TFrem => {
                self.advance()?;
                JvmInstruction::Frem
            }

            // freturn
            Token::TFreturn => {
                self.advance()?;
                JvmInstruction::Freturn
            }

            // fstore <varnum>
            Token::TFstore => {
                self.advance()?;

                let varnum = self.parse_u16().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::FstoreMissingOrInvalidVarnum,
                    )
                })?;
                JvmInstruction::Fstore { varnum }
            }

            // fstore_0
            Token::TFstore0 => {
                self.advance()?;
                JvmInstruction::Fstore0
            }

            // fstore_1
            Token::TFstore1 => {
                self.advance()?;
                JvmInstruction::Fstore1
            }

            // fstore_2
            Token::TFstore2 => {
                self.advance()?;
                JvmInstruction::Fstore2
            }

            // fstore_3
            Token::TFstore3 => {
                self.advance()?;
                JvmInstruction::Fstore3
            }

            // fsub
            Token::TFsub => {
                self.advance()?;
                JvmInstruction::Fsub
            }

            // getfield <field-spec> <descriptor>
            Token::TGetfield => {
                self.advance()?;

                if let Token::TIdent(gf_str) = self.see() {
                    if let Some(pos) = gf_str.rfind('/') {
                        let class_name = gf_str[..pos].to_owned();
                        let field_name = gf_str[pos + 1..].to_owned();
                        self.advance()?;

                        let field_descriptor = self.parse_field_descriptor().map_err(|_| {
                            ParserError::JvmInstructionError(
                                JvmInstructionError::GetfieldInvalidOrMissingFieldDescriptor,
                            )
                        })?;

                        JvmInstruction::Getfield {
                            class_name,
                            field_name,
                            field_descriptor,
                        }
                    } else {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::GetfieldMissingOrInvalidFieldName,
                        ));
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::GetfieldMissingOrInvalidClassName,
                    ));
                }
            }

            // getstatic <field-spec> <descriptor>
            Token::TGetstatic => {
                self.advance()?;

                if let Token::TIdent(gs_str) = self.see() {
                    if let Some(pos) = gs_str.rfind('/') {
                        let class_name = gs_str[..pos].to_owned();
                        let field_name = gs_str[pos + 1..].to_owned();
                        self.advance()?;

                        let field_descriptor = self.parse_field_descriptor().map_err(|_| {
                            ParserError::JvmInstructionError(
                                JvmInstructionError::GetstaticInvalidOrMissingFieldDescriptor,
                            )
                        })?;

                        JvmInstruction::Getstatic {
                            class_name,
                            field_name,
                            field_descriptor,
                        }
                    } else {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::GetstaticMissingOrInvalidFieldName,
                        ));
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::GetstaticMissingOrInvalidClassName,
                    ));
                }
            }

            // goto <label>
            Token::TGoto => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::GotoMissingLabel)
                })?;
                JvmInstruction::Goto { label }
            }

            // goto_w <label>
            Token::TGotow => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::GotowMissingLabel)
                })?;
                JvmInstruction::Gotow { label }
            }

            // i2b
            Token::TI2b => {
                self.advance()?;
                JvmInstruction::I2b
            }

            // i2c
            Token::TI2c => {
                self.advance()?;
                JvmInstruction::I2c
            }

            // i2d
            Token::TI2d => {
                self.advance()?;
                JvmInstruction::I2c
            }

            // i2f
            Token::TI2f => {
                self.advance()?;
                JvmInstruction::I2f
            }

            // i2l
            Token::TI2l => {
                self.advance()?;
                JvmInstruction::I2l
            }

            // i2s
            Token::TI2s => {
                self.advance()?;
                JvmInstruction::I2s
            }

            // iadd
            Token::TIadd => {
                self.advance()?;
                JvmInstruction::Iadd
            }

            // iaload
            Token::TIaload => {
                self.advance()?;
                JvmInstruction::Iaload
            }

            // iand
            Token::TIand => {
                self.advance()?;
                JvmInstruction::Iand
            }

            // iastore
            Token::TIastore => {
                self.advance()?;
                JvmInstruction::Iastore
            }

            // iconst_m1
            Token::TIconstm1 => {
                self.advance()?;
                JvmInstruction::Iconstm1
            }

            // iconst_0
            Token::TIconst0 => {
                self.advance()?;
                JvmInstruction::Iconst0
            }

            // iconst_1
            Token::TIconst1 => {
                self.advance()?;
                JvmInstruction::Iconst1
            }

            // iconst_2
            Token::TIconst2 => {
                self.advance()?;
                JvmInstruction::Iconst2
            }

            // iconst-3
            Token::TIconst3 => {
                self.advance()?;
                JvmInstruction::Iconst3
            }

            // iconst_4
            Token::TIconst4 => {
                self.advance()?;
                JvmInstruction::Iconst4
            }

            // iconst_5
            Token::TIconst5 => {
                self.advance()?;
                JvmInstruction::Iconst5
            }

            // idiv
            Token::TIdiv => {
                self.advance()?;
                JvmInstruction::Idiv
            }

            // if_acmpeq <label>
            Token::TIfacmpeq => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::IfacmpeqMissingOrInvalidLabel,
                    )
                })?;

                JvmInstruction::Ifacmpeq { label }
            }

            // if_acmpne <label>
            Token::TIfacmpne => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::IfacmpneMissingOrInvalidLabel,
                    )
                })?;

                JvmInstruction::Ifacmpne { label }
            }

            // if_icmpeq <label>
            Token::TIficmpeq => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::IficmpeqMissingOrInvalidLabel,
                    )
                })?;

                JvmInstruction::Ificmpeq { label }
            }

            // if_icmpge <label>
            Token::TIficmpge => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::IficmpgeMissingOrInvalidLabel,
                    )
                })?;

                JvmInstruction::Ificmpge { label }
            }

            // if_icmpgt <label>
            Token::TIficmpgt => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::IficmpgtMissingLabel)
                })?;
                JvmInstruction::Ificmpgt { label }
            }

            // if_icmple <label>
            Token::TIficmple => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::IficmpleMissingLabel)
                })?;
                JvmInstruction::Ificmple { label }
            }

            // if_icmplt <label>
            Token::TIficmplt => {
                self.advance()?;
                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::IficmpltMissingLabel)
                })?;
                JvmInstruction::Ificmplt { label }
            }

            // ifne <label>
            Token::TIfne => {
                self.advance()?;

                if let Token::TIdent(label) = self.see() {
                    let label = label.to_string();
                    self.advance()?;

                    JvmInstruction::Ifne { label }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::IfneMissingLabel,
                    ));
                }
            }

            // if_icmpne <label>
            Token::TIficmpne => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::IfcmpneMissingOrInvalidLabel,
                    )
                })?;

                JvmInstruction::Ificmpne { label }
            }

            // ifeq <label>
            Token::TIfeq => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::IfeqMissingOrInvalidLabel)
                })?;

                JvmInstruction::Ifeq { label }
            }

            // ifge <label>
            Token::TIfge => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::IfgeMissingOrInvalidLabel)
                })?;

                JvmInstruction::Ifge { label }
            }

            // ifgt <label>
            Token::TIfgt => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::IfgtMissingOrInvalidLabel)
                })?;

                JvmInstruction::Ifgt { label }
            }

            // ifle <label>
            Token::TIfle => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::IfleMissingOrInvalidLabel)
                })?;

                JvmInstruction::Ifle { label }
            }

            // iflt <label>
            Token::TIflt => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::IfltMissingOrInvalidLabel)
                })?;

                JvmInstruction::Iflt { label }
            }

            // ifnonnull <label>
            Token::TIfnonnull => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::IfnonnullMissingOrInvalidLabel,
                    )
                })?;

                JvmInstruction::Ifnonnull { label }
            }

            // ifnull <label>
            Token::TIfnull => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::IfnullMissingOrInvalidLabel,
                    )
                })?;

                JvmInstruction::Ifnull { label }
            }

            // iinc <varnum> <n>
            Token::TIinc => {
                self.advance()?;

                let varnum = self.parse_u16().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::IincMissingOrInvalidVarnum,
                    )
                })?;

                let delta = self.parse_i16().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::IincMissingOrInvalidDelta)
                })?;

                JvmInstruction::Iinc { varnum, delta }
            }

            // iload <varnum>
            Token::TIload => {
                self.advance()?;

                let varnum = self.parse_u16().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::IloadMissingVarnum)
                })?;

                JvmInstruction::Iload { varnum }
            }

            // iload_0
            Token::TIload0 => {
                self.advance()?;
                JvmInstruction::Iload0
            }

            // iload_1
            Token::TIload1 => {
                self.advance()?;
                JvmInstruction::Iload1
            }

            // iload_2
            Token::TIload2 => {
                self.advance()?;
                JvmInstruction::Iload2
            }

            // iload_3
            Token::TIload3 => {
                self.advance()?;
                JvmInstruction::Iload3
            }

            // imul
            Token::TImul => {
                self.advance()?;
                JvmInstruction::Imul
            }

            // ineg
            Token::TIneg => {
                self.advance()?;
                JvmInstruction::Ineg
            }

            // instanceof
            Token::TInstanceof => {
                self.advance()?;

                let check_type = self.parse_class_or_array_type().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::InstanceofMissingOrInvalidCheckType,
                    )
                })?;
                JvmInstruction::Instanceof { check_type }
            }

            // TODO
            Token::TInvokeinterface => {
                todo!()
            }

            // TODO
            // invokespecial <method-spec>
            Token::TInvokespecial => {
                self.advance()?;

                if let Token::TIdent(is_str) = self.see() {
                    if let Some(pos) = is_str.rfind('/') {
                        let class_name = is_str[..pos].to_owned();
                        let method_name = is_str[pos + 1..].to_owned();
                        self.advance()?;

                        if let Ok(method_descriptor) = self.parse_method_descriptor() {
                            JvmInstruction::Invokespecial {
                                class_name,
                                method_name,
                                method_descriptor,
                            }
                        } else {
                            return Err(ParserError::JvmInstructionError(
                                JvmInstructionError::InvokespecialInvalidOrMissingMethodDescriptor,
                            ));
                        }
                    } else {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::InvokespecialMissingMethodName,
                        ));
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::InvokespecialInvalid,
                    ));
                }
            }

            // TODO
            // invokestatic <method-spec>
            Token::TInvokestatic => {
                self.advance()?;

                if let Token::TIdent(is_str) = self.see() {
                    if let Some(pos) = is_str.rfind('/') {
                        let class_name = is_str[..pos].to_owned();
                        let method_name = is_str[pos + 1..].to_owned();
                        self.advance()?;

                        if let Ok(method_descriptor) = self.parse_method_descriptor() {
                            JvmInstruction::Invokestatic {
                                class_name,
                                method_name,
                                method_descriptor,
                            }
                        } else {
                            return Err(ParserError::JvmInstructionError(
                                JvmInstructionError::InvokestaticInvalidOrMissingMethodDescriptor,
                            ));
                        }
                    } else {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::InvokestaticMissingMethodName,
                        ));
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::InvokestaticInvalid,
                    ));
                }
            }

            // TODO
            // invokevirtual <method-spec>
            Token::TInvokevirtual => {
                self.advance()?;

                if let Token::TIdent(is_str) = self.see() {
                    if let Some(pos) = is_str.rfind('/') {
                        let class_name = is_str[..pos].to_owned();
                        let method_name = is_str[pos + 1..].to_owned();
                        self.advance()?;

                        if let Ok(method_descriptor) = self.parse_method_descriptor() {
                            JvmInstruction::Invokevirtual {
                                class_name,
                                method_name,
                                method_descriptor,
                            }
                        } else {
                            return Err(ParserError::JvmInstructionError(
                                JvmInstructionError::InvokevirtualInvalidOrMissingMethodDescriptor,
                            ));
                        }
                    } else {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::InvokevirtualMissingMethodName,
                        ));
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::InvokevirtualInvalid,
                    ));
                }
            }

            // ior
            Token::TIor => {
                self.advance()?;
                JvmInstruction::Ior
            }

            // irem
            Token::TIrem => {
                self.advance()?;
                JvmInstruction::Irem
            }

            // ireturn
            Token::TIreturn => {
                self.advance()?;
                JvmInstruction::Ireturn
            }

            // ishl
            Token::TIshl => {
                self.advance()?;
                JvmInstruction::Ishl
            }

            Token::TIshr => {
                todo!()
            }

            // istore <varnum>
            Token::TIstore => {
                self.advance()?;

                let varnum = self.parse_u16().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::IstoreMissingVarnum)
                })?;

                JvmInstruction::Istore { varnum }
            }

            // istore_0
            Token::TIstore0 => {
                self.advance()?;
                JvmInstruction::Istore0
            }

            // istore_1
            Token::TIstore1 => {
                self.advance()?;
                JvmInstruction::Istore1
            }

            // istore_2
            Token::TIstore2 => {
                self.advance()?;
                JvmInstruction::Istore2
            }

            // istore_3
            Token::TIstore3 => {
                self.advance()?;
                JvmInstruction::Istore3
            }

            // isub
            Token::TIsub => {
                self.advance()?;
                JvmInstruction::Isub
            }

            // iushr
            Token::TIushr => {
                self.advance()?;
                JvmInstruction::Iushr
            }

            // ixor
            Token::TIxor => {
                self.advance()?;
                JvmInstruction::Ixor
            }

            // jsr <label>
            Token::TJsr => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::JsrMissingLabel)
                })?;
                JvmInstruction::Jsr { label }
            }

            // jsr_w <label>
            Token::TJsrw => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::JsrwMissingLabel)
                })?;
                JvmInstruction::Jsrw { label }
            }

            // l2d
            Token::TL2d => {
                self.advance()?;
                JvmInstruction::L2d
            }

            // l2f
            Token::TL2f => {
                self.advance()?;
                JvmInstruction::L2f
            }

            // l2i
            Token::TL2i => {
                self.advance()?;
                JvmInstruction::L2i
            }

            // ladd
            Token::TLadd => {
                self.advance()?;
                JvmInstruction::Ladd
            }

            // laload
            Token::TLaload => {
                self.advance()?;
                JvmInstruction::Laload
            }

            // land
            Token::TLand => {
                self.advance()?;
                JvmInstruction::Land
            }

            // lastore
            Token::TLastore => {
                self.advance()?;
                JvmInstruction::Lastore
            }

            // lcmp
            Token::TLcmp => {
                self.advance()?;
                JvmInstruction::Lcmp
            }

            // locaonst_0
            Token::TLconst0 => {
                self.advance()?;
                JvmInstruction::Lconst0
            }

            // lconst_1
            Token::TLconst1 => {
                self.advance()?;
                JvmInstruction::Lconst1
            }

            // ldc <double / integer / quoted string>
            Token::TLdc => {
                self.advance()?;

                match self.see() {
                    Token::TInt(n) => {
                        let ival = *n as i64;
                        self.advance()?;
                        JvmInstruction::Ldc(LdcValue::Integer(ival))
                    }

                    Token::TFloat(f) => {
                        let dval = *f as f64;
                        self.advance()?;
                        JvmInstruction::Ldc(LdcValue::Double(dval))
                    }

                    Token::TString(s) => {
                        let sval = s.to_owned();
                        self.advance()?;
                        JvmInstruction::Ldc(LdcValue::QuotedString(sval))
                    }

                    _ => {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::LdcIncorrectValue,
                        ))
                    }
                }
            }

            // ldc2_w <Long / Double>
            Token::TLdc2w => {
                self.advance()?;

                match self.see() {
                    Token::TInt(n) => {
                        let ival = *n as i64;
                        self.advance()?;
                        JvmInstruction::Ldc2w(Ldc2wValue::Long(ival))
                    }

                    Token::TFloat(f) => {
                        let dval = *f as f64;
                        self.advance()?;
                        JvmInstruction::Ldc2w(Ldc2wValue::Double(dval))
                    }

                    _ => {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::Ldc2wIncorrectValue,
                        ))
                    }
                }
            }

            // ldcw <double / integer / quoted string>
            Token::TLdcw => {
                self.advance()?;

                match self.see() {
                    Token::TInt(n) => {
                        let ival = *n as i64;
                        self.advance()?;
                        JvmInstruction::Ldcw(LdcValue::Integer(ival))
                    }

                    Token::TFloat(f) => {
                        let dval = *f as f64;
                        self.advance()?;
                        JvmInstruction::Ldcw(LdcValue::Double(dval))
                    }

                    Token::TString(s) => {
                        let sval = s.to_owned();
                        self.advance()?;
                        JvmInstruction::Ldcw(LdcValue::QuotedString(sval))
                    }

                    _ => {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::LdcwIncorrectValue,
                        ))
                    }
                }
            }

            // ldiv
            Token::TLdiv => {
                self.advance()?;
                JvmInstruction::Ldiv
            }

            // lload <varnum>
            Token::TLload => {
                self.advance()?;

                let varnum = self.parse_u16().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::LloadMissingOrInvalidVarnum,
                    )
                })?;

                JvmInstruction::Lload { varnum }
            }

            // lload_0
            Token::TLload0 => {
                self.advance()?;
                JvmInstruction::Lload0
            }

            // lload_1
            Token::TLload1 => {
                self.advance()?;
                JvmInstruction::Lload1
            }

            // lload_2
            Token::TLload2 => {
                self.advance()?;
                JvmInstruction::Lload2
            }

            //lload_3
            Token::TLload3 => {
                self.advance()?;
                JvmInstruction::Lload3
            }

            // lmul
            Token::TLmul => {
                self.advance()?;
                JvmInstruction::Lmul
            }

            // lneg
            Token::TLneg => {
                self.advance()?;
                JvmInstruction::Lneg
            }

            // TODO
            // lookupswitch          <-  'lookupswitch'   LookupSwitchPair*  DefaultSwitchPair
            // LookupSwitchPair      <-  Integer          COLON_symbol       Label
            // DefaultSwitchPair     <-  DEFAULT_keyword  COLON_symbol       Label
            Token::TLookupswitch => {
                self.advance()?;

                let switches = self.parse_lookup_switches()?;
                let default = self.parse_default_switch_pair().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::LookupswitchMissingDefault,
                    )
                })?;

                JvmInstruction::Lookupswitch { switches, default }
            }

            // lor
            Token::TLor => {
                self.advance()?;
                JvmInstruction::Lor
            }

            // lrem
            Token::TLrem => {
                self.advance()?;
                JvmInstruction::Lrem
            }

            // lreturn
            Token::TLreturn => {
                self.advance()?;
                JvmInstruction::Lreturn
            }

            // lshl
            Token::TLshl => {
                self.advance()?;
                JvmInstruction::Lshl
            }

            // lshr
            Token::TLshr => {
                self.advance()?;
                JvmInstruction::Lshr
            }

            // lstore <varnum>
            Token::TLstore => {
                self.advance()?;

                let varnum = self.parse_u16().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::LstoreMissingOrInvalidVarnum,
                    )
                })?;

                JvmInstruction::Lstore { varnum }
            }

            // lstore_0
            Token::TLstore0 => {
                self.advance()?;
                JvmInstruction::Lstore0
            }

            // lstore_1
            Token::TLstore1 => {
                self.advance()?;
                JvmInstruction::Lstore1
            }

            // lstore_2
            Token::TLstore2 => {
                self.advance()?;
                JvmInstruction::Lstore2
            }

            // lstore_3
            Token::TLstore3 => {
                self.advance()?;
                JvmInstruction::Lstore3
            }

            // lsub
            Token::TLsub => {
                self.advance()?;
                JvmInstruction::Lsub
            }

            // lushr
            Token::TLushr => {
                self.advance()?;
                JvmInstruction::Lushr
            }

            // lxor
            Token::TLxor => {
                self.advance()?;
                JvmInstruction::Lxor
            }

            // monitorenter
            Token::TMonitorenter => {
                self.advance()?;
                JvmInstruction::Monitorenter
            }

            // monitorexit
            Token::TMonitorexit => {
                self.advance()?;
                JvmInstruction::Monitorexit
            }

            // TODO
            Token::TMultianewarray => {
                self.advance()?;

                let component_type = self.parse_field_descriptor()?;
                if let Token::TInt(n) = self.see() {
                    let dimensions = *n as u8;
                    self.advance()?;
                    JvmInstruction::Multianewarray {
                        component_type,
                        dimensions,
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::MultianewarrayMissingDimensions,
                    ));
                }
            }

            // TODO
            // new <class>
            Token::TNew => {
                self.advance()?;

                if let Token::TIdent(class_name) = self.see() {
                    let class_name = class_name.to_string();
                    self.advance()?;
                    JvmInstruction::New { class_name }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::NewMissingClassName,
                    ));
                }
            }

            // TODO
            // newarray <primitive_type>
            Token::TNewarray => {
                self.advance()?;

                if let Token::TIdent(prim_type) = self.see() {
                    match prim_type.as_str() {
                        "char" => {
                            self.advance()?;
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Character,
                            }
                        }

                        "float" => {
                            self.advance()?;
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Float,
                            }
                        }

                        "double" => {
                            self.advance()?;
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Double,
                            }
                        }

                        "byte" => {
                            self.advance()?;
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Byte,
                            }
                        }

                        "short" => {
                            self.advance()?;
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Short,
                            }
                        }

                        "int" => {
                            self.advance()?;
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Integer,
                            }
                        }

                        "long" => {
                            self.advance()?;
                            JvmInstruction::Newarray {
                                component_type: PhoronBaseType::Long,
                            }
                        }

                        _ => {
                            return Err(ParserError::JvmInstructionError(
                                JvmInstructionError::NewarrayInvalidType,
                            ))
                        }
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::NewarrayInvalidType,
                    ));
                }
            }

            // nop
            Token::TNop => {
                self.advance()?;
                JvmInstruction::Nop
            }

            // pop
            Token::TPop => {
                self.advance()?;
                JvmInstruction::Pop
            }

            // pop2
            Token::TPop2 => {
                self.advance()?;
                JvmInstruction::Pop2
            }

            // TODO
            Token::TPutfield => {
                todo!()
            }

            // TODO
            Token::TPutstatic => {
                todo!()
            }

            // ret <varnum>
            Token::TRet => {
                self.advance()?;

                if let Token::TInt(varnum) = self.see() {
                    let varnum = *varnum as u16;
                    self.advance()?;

                    JvmInstruction::Ret { varnum }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::RetMissingVarnum,
                    ));
                }
            }

            // return
            Token::TReturn => {
                self.advance()?;
                JvmInstruction::Return
            }

            // saload
            Token::TSaload => {
                self.advance()?;
                JvmInstruction::Saload
            }

            // sastore
            Token::TSastore => {
                self.advance()?;
                JvmInstruction::Sastore
            }

            // sipush i16
            Token::TSipush => {
                self.advance()?;

                if let Token::TInt(s) = self.see() {
                    let s = *s as i16;
                    self.advance()?;

                    JvmInstruction::Sipush(s)
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::SipushMissingConstant,
                    ));
                }
            }

            // swap
            Token::TSwap => {
                self.advance()?;
                JvmInstruction::Swap
            }

            // tableswitch    <-  'tableswitch'   Low   High  TableSwitchSingleton*  DefaultSwitchPair
            // TableSwitchSingleton  <-  Label
            Token::TTableswitch => {
                self.advance()?;

                let low = self.parse_i32().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::TableswitchMissingLow)
                })?;

                let high = self.parse_i32().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::TableswitchMissingHigh)
                })?;

                let switches = self.parse_table_switches()?;
                let default = self.parse_default_switch_pair().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::TableswitchMissingDefault)
                })?;

                JvmInstruction::Tableswitch {
                    low,
                    high,
                    switches,
                    default,
                }
            }

            // wide
            Token::Twide => {
                self.advance()?;
                JvmInstruction::Wide
            }

            _ => {
                return Err(ParserError::InvalidJvmInstruction(format!(
                    "{:#?}",
                    self.see()
                )))
            }
        })
    }

    /// Instruction <- line_comment* (Directive / JvmInstruction / Label) line_comment?  newline
    fn parse_instruction(&mut self) -> ParserResult<PhoronInstruction> {
        Ok(match self.see() {
            Token::TThrows | Token::TCatch | Token::TLimit | Token::TVar | Token::TLine => {
                PhoronInstruction::PhoronDirective(self.parse_directive()?)
            }

            Token::TAaload
            | Token::TAastore
            | Token::TAconstnull
            | Token::TAload
            | Token::TAload0
            | Token::TAload1
            | Token::TAload2
            | Token::TAload3
            | Token::TAnewarray
            | Token::TAreturn
            | Token::TArraylength
            | Token::TAssign
            | Token::TAstore
            | Token::TAstore0
            | Token::TAstore1
            | Token::TAstore2
            | Token::TAstore3
            | Token::TAthrow
            | Token::TBaload
            | Token::TBastore
            | Token::TBipush
            | Token::TBridge
            | Token::TCaload
            | Token::TCastore
            | Token::TCheckcast
            | Token::TD2f
            | Token::TD2i
            | Token::TD2l
            | Token::TDadd
            | Token::TDaload
            | Token::TDastore
            | Token::TDcmpg
            | Token::TDcmpl
            | Token::TDconst0
            | Token::TDconst1
            | Token::TDdiv
            | Token::TDefault
            | Token::TDload
            | Token::TDload0
            | Token::TDload1
            | Token::TDload2
            | Token::TDload3
            | Token::TDmul
            | Token::TDneg
            | Token::TDot
            | Token::TDrem
            | Token::TDreturn
            | Token::TDstore
            | Token::TDstore0
            | Token::TDstore1
            | Token::TDstore2
            | Token::TDstore3
            | Token::TDsub
            | Token::TDup
            | Token::TDup2
            | Token::TDup2x1
            | Token::TDup2x2
            | Token::TDupx1
            | Token::TDupx2
            | Token::TEnum
            | Token::TF2d
            | Token::TF2i
            | Token::TF2l
            | Token::TFadd
            | Token::TFaload
            | Token::TFastore
            | Token::TFcmpg
            | Token::TFcmpl
            | Token::TFconst0
            | Token::TFconst1
            | Token::TFconst2
            | Token::TFdiv
            | Token::TField
            | Token::TFinal
            | Token::TFload
            | Token::TFload0
            | Token::TFload1
            | Token::TFload2
            | Token::TFload3
            | Token::TFmul
            | Token::TFneg
            | Token::TFrem
            | Token::TFreturn
            | Token::TFrom
            | Token::TFstore
            | Token::TFstore0
            | Token::TFstore1
            | Token::TFstore2
            | Token::TFstore3
            | Token::TFsub
            | Token::TGetfield
            | Token::TGetstatic
            | Token::TGoto
            | Token::TGotow
            | Token::TI2b
            | Token::TI2c
            | Token::TI2d
            | Token::TI2f
            | Token::TI2l
            | Token::TI2s
            | Token::TIadd
            | Token::TIaload
            | Token::TIand
            | Token::TIastore
            | Token::TIconst0
            | Token::TIconst1
            | Token::TIconst2
            | Token::TIconst3
            | Token::TIconst4
            | Token::TIconst5
            | Token::TIconstm1
            | Token::TIdiv
            | Token::TIfacmpeq
            | Token::TIfacmpne
            | Token::TIfeq
            | Token::TIfge
            | Token::TIfgt
            | Token::TIficmpeq
            | Token::TIficmpge
            | Token::TIficmpgt
            | Token::TIficmple
            | Token::TIficmplt
            | Token::TIficmpne
            | Token::TIfle
            | Token::TIflt
            | Token::TIfne
            | Token::TIfnonnull
            | Token::TIfnull
            | Token::TIinc
            | Token::TIload
            | Token::TIload0
            | Token::TIload1
            | Token::TIload2
            | Token::TIload3
            | Token::TImul
            | Token::TIneg
            | Token::TInstanceof
            | Token::TInvokeinterface
            | Token::TInvokespecial
            | Token::TInvokestatic
            | Token::TInvokevirtual
            | Token::TIor
            | Token::TIrem
            | Token::TIreturn
            | Token::TIshl
            | Token::TIshr
            | Token::TIstore
            | Token::TIstore0
            | Token::TIstore1
            | Token::TIstore2
            | Token::TIstore3
            | Token::TIsub
            | Token::TIushr
            | Token::TIxor
            | Token::TJsr
            | Token::TJsrw
            | Token::TL2d
            | Token::TL2f
            | Token::TL2i
            | Token::TLadd
            | Token::TLand
            | Token::TLastore
            | Token::TLcmp
            | Token::TLconst0
            | Token::TLconst1
            | Token::TLdc
            | Token::TLdc2w
            | Token::TLdcw
            | Token::TLdiv
            | Token::TLload
            | Token::TLload0
            | Token::TLload1
            | Token::TLload2
            | Token::TLload3
            | Token::TLmul
            | Token::TLneg
            | Token::TLoaload
            | Token::TLookupswitch
            | Token::TLor
            | Token::TLrem
            | Token::TLreturn
            | Token::TLshl
            | Token::TLshr
            | Token::TLstore
            | Token::TLstore0
            | Token::TLstore1
            | Token::TLstore2
            | Token::TLstore3
            | Token::TLsub
            | Token::TLushr
            | Token::TLxor
            | Token::TMonitorenter
            | Token::TMonitorexit
            | Token::TMultianewarray
            | Token::TNew
            | Token::TNewarray
            | Token::TNop
            | Token::TPop
            | Token::TPop2
            | Token::TPutfield
            | Token::TPutstatic
            | Token::TRet
            | Token::TReturn
            | Token::TSaload
            | Token::TSastore
            | Token::TSipush
            | Token::TSuper
            | Token::TSwap
            | Token::TTableswitch => {
                PhoronInstruction::JvmInstruction(self.parse_jvm_instruction()?)
            }

            Token::TIdent(label_str) => {
                let label = label_str.to_string();
                self.advance()?;

                if let Token::TColon = self.see() {
                    self.advance()?;
                    PhoronInstruction::PhoronLabel(label)
                } else {
                    return Err(ParserError::IllegalLabelError);
                }
            }
            _ => {
                return Err(ParserError::InvalidInstruction(format!(
                    "{:#?}",
                    self.see()
                )))
            }
        })
    }

    fn parse_instructions(&mut self) -> ParserResult<Vec<PhoronInstruction>> {
        let mut instructions = Vec::new();

        while self.see() != &Token::TEof {
            if let Token::TEnd = self.see() {
                self.advance()?;
                if let Token::TEndMethod = self.see() {
                    self.advance()?;
                    break;
                } else {
                    return Err(ParserError::MissingEndMethodMarker);
                }
            } else {
                instructions.push(self.parse_instruction()?);
            }
        }

        Ok(instructions)
    }

    /// MethodDescriptor <- LPAREN_symbol ParameterDescriptor* RPAREN_symbol ReturnDescriptor
    /// ParameterDescriptor <- FieldType
    /// ReturnDescriptor <- FieldType / VoidType
    /// VoidType <- 'V'
    fn parse_method_descriptor(&mut self) -> ParserResult<PhoronMethodDescriptor> {
        if let Token::TLeftParen = self.see() {
            self.advance()?;
            let param_descriptor = match self.parse_field_descriptor() {
                Err(err) => match err {
                    ParserError::EmptyFieldDescriptor => None,
                    _ => return Err(err),
                },
                Ok(desc) => Some(desc),
            };

            if let Token::TRightParen = self.see() {
                self.advance()?;

                let return_descriptor = if let Token::TIdent(maybe_v) = self.see() {
                    if maybe_v == "V" {
                        self.advance()?;
                        PhoronReturnDescriptor::VoidDescriptor
                    } else {
                        PhoronReturnDescriptor::FieldDescriptor(self.parse_field_descriptor()?)
                    }
                } else if let Token::TLeftSquareBracket = self.see() {
                    PhoronReturnDescriptor::FieldDescriptor(self.parse_field_descriptor()?)
                } else {
                    return Err(ParserError::InvalidMethodDescriptor);
                };

                Ok(PhoronMethodDescriptor {
                    param_descriptor,
                    return_descriptor,
                })
            } else {
                Err(ParserError::InvalidMethodDescriptor)
            }
        } else {
            Err(ParserError::InvalidMethodDescriptor)
        }
    }

    /// MethodDef <- line_comment*
    ///    METHOD_keyword  MethodAccessFlag* MethodName MethodDescriptor newline
    ///      Instruction*
    ///    END_Keyword METHOD_END_keyword newline
    fn parse_method_def(&mut self) -> ParserResult<PhoronMethodDef> {
        self.advance()?;

        let mut access_flags = Vec::new();
        while self.is_method_access_flag(self.see()) {
            access_flags.push(self.get_method_acess_flags(&self.see())?);
            self.advance()?;
        }

        if let Token::TIdent(name_str) = self.see() {
            let name = name_str.to_string();
            self.advance()?;

            let method_descriptor = self.parse_method_descriptor()?;
            let instructions = self.parse_instructions()?;

            Ok(PhoronMethodDef {
                name,
                access_flags,
                method_descriptor,
                instructions,
            })
        } else {
            Err(ParserError::MissingMethodName)
        }
    }

    fn parse_method_defs(&mut self) -> ParserResult<Vec<PhoronMethodDef>> {
        let mut method_defs = Vec::new();

        while let Token::TMethod = self.see() {
            method_defs.push(self.parse_method_def()?);
        }

        Ok(method_defs)
    }

    /// Body <- FieldDef* MethodDef*
    fn parse_body(&mut self) -> ParserResult<PhoronBody> {
        let field_defs = self.parse_field_defs()?;
        let method_defs = self.parse_method_defs()?;

        Ok(PhoronBody {
            field_defs,
            method_defs,
        })
    }

    /// SourceFileDef <- SOURCE_keyword FileName newline
    fn parse_sourcefile_def(&mut self) -> ParserResult<PhoronSourceFileDef> {
        if let Token::TIdent(source_file_str) = self.see() {
            let source_file = source_file_str.to_string();
            self.advance()?;

            Ok(PhoronSourceFileDef { source_file })
        } else {
            Err(ParserError::MissingSourceFileName)
        }
    }

    /// Header <- SourceFileDef? (ClassDef / InterfaceDef) SuperDef
    fn parse_header(&mut self) -> ParserResult<PhoronHeader> {
        self.advance()?;

        Ok(match self.see() {
            Token::TSource => {
                self.advance()?;
                let sourcefile_def = self.parse_sourcefile_def()?;

                let class_or_interface_def = match self.see() {
                    Token::TClass => PhoronClassOrInterface::Class(self.parse_class_def()?),
                    Token::TInterface => {
                        PhoronClassOrInterface::Interface(self.parse_interface_def()?)
                    }
                    err_tok => return Err(ParserError::InvalidToken(format!("{:#?}", err_tok))),
                };

                let super_def = self.parse_super_def()?;

                PhoronHeader {
                    sourcefile_def: Some(sourcefile_def),
                    class_or_interface_def,
                    super_def,
                }
            }

            Token::TClass => {
                let class_or_interface_def = PhoronClassOrInterface::Class(self.parse_class_def()?);
                let super_def = self.parse_super_def()?;

                PhoronHeader {
                    sourcefile_def: None,
                    class_or_interface_def,
                    super_def,
                }
            }

            Token::TInterface => {
                let class_or_interface_def =
                    PhoronClassOrInterface::Interface(self.parse_interface_def()?);
                let super_def = self.parse_super_def()?;

                PhoronHeader {
                    sourcefile_def: None,
                    class_or_interface_def,
                    super_def,
                }
            }

            tok => return Err(ParserError::InvalidToken(format!("{:#?}", tok))),
        })
    }

    /// PhoronProgram <- line_comment* Header Body eof
    pub fn parse(&mut self) -> ParserResult<PhoronProgram> {
        let header = self.parse_header()?;
        let body = self.parse_body()?;

        Ok(PhoronProgram { header, body })
    }
}
