use crate::{
    ast::*,
    lexer::{Lexer, LexerError, Token},
};

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum DirectiveError {
    LineMissingLineNumber,
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
        use DirectiveError::*;

        write!(
            f,
            "{}",
            match *self {
                LineMissingLineNumber => ".line - missing line number".into(),
                ThrowsMissingClassName => ".throws - missing class name".into(),
                VarMissingVarnum => ".var - missing var num".into(),
                VarMissingIsKeyword => ".var - missing is keyword".into(),
                VarMissingName => ".var - missing name".into(),
                VarMissingFieldDescriptor => ".var - missing field descriptor".into(),
                VarMissingFromKeyword => ".var - missing from keyword".into(),
                VarMissingFromLabel => ".var - missing from label".into(),
                VarMissingToKeyword => ".var - missing to keyword".into(),
                VarMissingToLabel => ".var - missing to label".into(),
                StackDirectiveMissingCount =>
                    ".limit stack : missing count value for .limit stack directive".into(),
                InvalidDirective(ref tok) => format!("invalid directive {tok}"),
                LocalsDirectiveMissingCount =>
                    "missing count value for .limit locals directive".into(),
                MissingStackOrLocal => "directive limit : missing `stack` or `locals`".into(),
            }
        )
    }
}

#[derive(Debug)]
pub enum JvmInstructionError {
    LookupswitchMissingDefault,
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
    PutfieldMissingOrInvalidClassName,
    PutfieldMissingOrInvalidFieldDescriptor,
    PutfieldMissingOrInvalidFieldName,
    GetstaticMissingOrInvalidClassName,
    GetstaticInvalidOrMissingFieldDescriptor,
    GetstaticMissingOrInvalidFieldName,
    PutstaticMissingOrInvalidClassName,
    PutstaticMissingOrInvalidFieldDescriptor,
    PutstaticMissingOrInvalidFieldName,
    IincMissingOrInvalidVarnum,
    IincMissingOrInvalidDelta,
    InvokeinterfaceMissingNumericConstant,
    InvokeinterfaceMissingMethodName,
    InvokeinterfaceInvalidOrMissingMethodDescriptor,
    InvokeinterfaceInvalid,
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
    MultianewarrayMissingOrInvalidComponentType,
    MultianewarrayMissingDimensions,
    NewMissingClassName,
    NewarrayInvalidType,
}

impl fmt::Display for JvmInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use JvmInstructionError::*;

        write!(
            f,
            "{}",
            match *self {
                JvmInstructionError::PutfieldMissingOrInvalidClassName =>
                    "putfield : invalid or missing class name".into(),
                JvmInstructionError::PutfieldMissingOrInvalidFieldDescriptor =>
                    "putfield : missing or invalid field descriptor".into(),
                JvmInstructionError::PutfieldMissingOrInvalidFieldName =>
                    "putfield :missing or invalid field name".into(),
                JvmInstructionError::PutstaticMissingOrInvalidClassName =>
                    "putstatic : missing or invalid class name".into(),
                JvmInstructionError::PutstaticMissingOrInvalidFieldDescriptor =>
                    "putstatic : missing or invalid field descriptor".into(),
                JvmInstructionError::PutstaticMissingOrInvalidFieldName =>
                    "putstatic : missing or invalid field name".into(),

                MultianewarrayMissingOrInvalidComponentType =>
                    "multianewarray - missing or invalid component type".into(),
                InvokeinterfaceMissingMethodName => "invokeinterface : missing method name".into(),
                InvokeinterfaceInvalidOrMissingMethodDescriptor =>
                    "invokeinterface : missing or invalid method descriptor".into(),
                InvokeinterfaceInvalid =>
                    "invokeinterface : missing or invalid interface name".into(),
                InvokeinterfaceMissingNumericConstant =>
                    "invokeinterface: missing numeric constant".into(),
                LstoreMissingOrInvalidVarnum => "lstore : missing or invalid var num".into(),
                LloadMissingOrInvalidVarnum => "lload : missing or invalid var num".into(),
                InstanceofMissingOrInvalidCheckType =>
                    "instanceof : missing or invalid check type".into(),
                IfeqMissingOrInvalidLabel => "ifeq : missing or invalid label".into(),
                IfcmpneMissingOrInvalidLabel => "if_cmpne : missing or invalid label".into(),
                IfnonnullMissingOrInvalidLabel => "if_nonnull : missing or invalid label".into(),
                IfnullMissingOrInvalidLabel => "if_null : missing or invalid label".into(),
                IfleMissingOrInvalidLabel => "ifle : missing or invalid label".into(),
                IfltMissingOrInvalidLabel => "iflt : missing or invalid label".into(),
                IfgeMissingOrInvalidLabel => "ifge : missing or invalid label".into(),
                IfgtMissingOrInvalidLabel => "ifgt : missing or invalid label".into(),
                IficmpeqMissingOrInvalidLabel => "if_icmpeq : missing or invalid label".into(),
                IficmpgeMissingOrInvalidLabel => "if_icmpge : missing or invalid label".into(),
                IfacmpneMissingOrInvalidLabel => "if_acmpne : missing or invalid label".into(),
                IfacmpeqMissingOrInvalidLabel => "if_acmpeq : missing or invalid label".into(),
                GetfieldMissingOrInvalidClassName =>
                    "getfield : missing or invalid class name".into(),
                GetfieldMissingOrInvalidFieldName =>
                    "getfield : missing or invalid field name".into(),
                GetfieldMissingOrInvalidFieldDescriptor =>
                    "getfield : missing or invalid field descriptor".into(),
                FstoreMissingOrInvalidVarnum => "fstore : missing or invalid var num".into(),
                FloadMissingOrInvalidVarnum => "fload : missing or invalid var num".into(),
                DstoreMissingOrInvalidVarnum => "dstore : missing or invalod var num".into(),
                DloadMissingOrInvalidVarnum => "dload : missing or invalid var num".into(),
                AnewarrayMissingOrInvalidComponentType =>
                    "anewarray : missing or invalid component type".into(),
                AloadMissingVarnum => "aload - missing varn num".into(),
                TableswitchMissingLow => "tableswitch : missing low".into(),
                TableswitchMissingHigh => "tableswitch : missing high".into(),
                TableswitchMissingDefault => "tableswitch : missing default".into(),
                LookupswitchMissingDefault => "lookupswitch : invalid default".into(),
                LookupswitchInvalidSwitchEntry => "lookupswitch : invalid entry".into(),
                LookupswitchMissingLabelforSwitchEntry =>
                    "lookupswitch : missing label for switch entry".into(),
                CheckcastInvalidOrMissingType => "checkcast :  invalid or missing type".into(),
                RetMissingVarnum => "ret : missing var num".into(),
                JsrMissingLabel => "jsr : missing label".into(),
                JsrwMissingLabel => "jsrw : missing label".into(),
                GotoMissingLabel => "goto : missing label".into(),
                GotowMissingLabel => "gotow : missing label".into(),
                SipushMissingConstant => "sipush : missing constant value".into(),
                IficmpgtMissingLabel => "if_icmpgt : missing label".into(),
                IficmpltMissingLabel => "if_icmplt : missing label".into(),
                IficmpleMissingLabel => "if_icmple : missing label".into(),
                IloadMissingVarnum => "iload : missing var number".into(),
                IfneMissingLabel => "ifne : missing label".into(),
                IincMissingOrInvalidVarnum => "iinc : missing or invalid var num".into(),
                IincMissingOrInvalidDelta => "iinc : missing or invalid delta".into(),
                AnewarrayInvalidTypeDescriptor(ref err_type) =>
                    format!("anewarray : incorrect type - {err_type}"),
                InvokespecialInvalid => "invokespecial: badly-formed or invalid".into(),
                InvokespecialMissingMethodName => "invokespecial : missing method name".into(),
                InvokespecialInvalidOrMissingMethodDescriptor =>
                    "invokespecial : missing or invalid method descriptor".into(),
                InvokevirtualInvalid => "invokevirtual: badly-formed or invalid".into(),
                InvokevirtualMissingMethodName => "invokevirtual : missing method name".into(),
                InvokevirtualInvalidOrMissingMethodDescriptor =>
                    "invokevirtual : missing or invalid method descriptor".into(),
                InvokestaticInvalid => "invokestatic: badly-formed or invalid".into(),
                InvokestaticMissingMethodName => "invokestatic : missing method name".into(),
                InvokestaticInvalidOrMissingMethodDescriptor =>
                    "invokestatic : missing or invalid method descriptor".into(),
                BipushMissingByte => "Bipush : missing byte".into(),
                GetfieldInvalid => "getfield : badly-formed or invalid".into(),
                GetfieldInvalidOrMissingFieldDescriptor =>
                    "getfield : missing or invalid field descriptor".into(),
                GetfieldMissingFieldName => "getfield : missing field name".into(),
                GetstaticMissingOrInvalidClassName =>
                    "getstatic : missing or invalid class name".into(),
                GetstaticInvalidOrMissingFieldDescriptor =>
                    "getstatic : missing or invalid field descriptor".into(),
                GetstaticMissingOrInvalidFieldName =>
                    "getstatic : missing or invalid field name".into(),
                LdcIncorrectValue => "ldc : incorrect value".into(),
                LdcwIncorrectValue => "ldcw : incorrect value".into(),
                Ldc2wIncorrectValue => "ldc2_w : incorrect value".into(),
                MultianewarrayMissingDimensions => "multianewarray : missing dimensions".into(),
                NewMissingClassName => "newa : missing class name".into(),
                NewarrayInvalidType => "newarray : invalid type".into(),
                IstoreMissingVarnum => "istore : missing var num".into(),
                AstoreMissingVarnum => "astore : missing var num".into(),
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
        use ParserError::{DirectiveError, JvmInstructionError, *};

        write!(
            f,
            "{}",
            match *self {
                FailedToParsePrimitive => "failed to parse primitive value".into(),
                MissingDefaultKeyword => "missing default keyword".into(),
                IncorrectToken(ref expected, ref actual) =>
                    format!("token mismatch while parsing: expected {expected} but found {actual}"),
                LexerError(ref err) => err.to_string(),
                IllegalLabelError => "malformed label - missing semicolon".into(),
                InvalidToken(ref tokstr) => format!("Invalid token at position: {:#?}", tokstr),
                InvalidFieldDescriptor => "invalid field descriptor".into(),
                InvalidDirective(ref dir) => format!("invalid directive: {dir}"),
                EmptyFieldDescriptor => "empty field descriptor".into(),
                InvalidFieldInitValue(ref tokstr) =>
                    format!("invalid field initialisation value: {:#?}", tokstr),
                InvalidMethodDescriptor => "invalid method descriptor".into(),
                InvalidInstruction(ref instr) => format!("invalid phpron instuction: {instr}"),
                InvalidJvmInstruction(ref instr) => format!("invalid JVM instruction {instr}"),
                MissingSourceFileName => "missing source file name".into(),
                MissingClassName => "missing class name".into(),
                MissingLabel => "missing label".into(),
                MissingInterfaceName => "missing interface name".into(),
                MissingSuperclassName => "missing super class name".into(),
                MissingFieldName => "field name missing in definition".into(),
                MissingMethodName => "method name missing in definition".into(),
                MissingEndMethodMarker => "malformed end method - missing method keyword".into(),
                UnknownClassOrInterfaceAccessFlag(ref flag) =>
                    format!("Invalid flag for class/interface: {flag}"),
                UnknownFieldAccessFlag(ref flag) => format!("Invalid flag for field: {flag}"),
                UnknownMethodAccessFlag(ref flag) => format!("Invalid flag for method: {flag}"),
                JvmInstructionError(ref jvm_err) => jvm_err.to_string(),
                DirectiveError(ref dir_err) => dir_err.to_string(),
            }
        )
    }
}

impl From<LexerError> for ParserError {
    fn from(lex_err: LexerError) -> Self {
        ParserError::LexerError(lex_err)
    }
}

use Token::*;

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
            TPublic | TFinal | TSuper | TInterface | TAbstract | TSynthetic | TAnnotation
            | TEnum | TModule => true,
            _ => false,
        }
    }

    fn is_field_access_flag(&self, tok: &Token) -> bool {
        match tok {
            TPublic | TPrivate | TProtected | TStatic | TFinal | TVolatile | TTransient
            | TSynthetic | TEnum => true,
            _ => false,
        }
    }

    fn is_method_access_flag(&self, tok: &Token) -> bool {
        match tok {
            TPublic | TPrivate | TProtected | TStatic | TFinal | TSynthetic | TSynchronized
            | TBridge | TVarargs | TNative | TAbstract | TStrict => true,
            _ => false,
        }
    }

    fn get_class_or_interface_access_flag(
        &self,
        tok: &Token,
    ) -> ParserResult<PhoronClassOrInterfaceAccessFlag> {
        Ok(match tok {
            TPublic => PhoronClassOrInterfaceAccessFlag::AccPublic,
            TFinal => PhoronClassOrInterfaceAccessFlag::AccFinal,
            TSuper => PhoronClassOrInterfaceAccessFlag::AccSuper,
            TInterface => PhoronClassOrInterfaceAccessFlag::AccInterface,
            TAbstract => PhoronClassOrInterfaceAccessFlag::AccAbstract,
            TSynthetic => PhoronClassOrInterfaceAccessFlag::AccSynthetic,
            TAnnotation => PhoronClassOrInterfaceAccessFlag::AccAnnotation,
            TEnum => PhoronClassOrInterfaceAccessFlag::AccEnum,
            TModule => PhoronClassOrInterfaceAccessFlag::AccModule,
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
            TPublic => PhoronFieldAccessFlag::AccPublic,
            TPrivate => PhoronFieldAccessFlag::AccPrivate,
            TProtected => PhoronFieldAccessFlag::AccProtected,
            TStatic => PhoronFieldAccessFlag::AccStatic,
            TFinal => PhoronFieldAccessFlag::AccFinal,
            TVolatile => PhoronFieldAccessFlag::AccVolatile,
            TTransient => PhoronFieldAccessFlag::AccTransient,
            TSynthetic => PhoronFieldAccessFlag::AccSynthetic,
            TEnum => PhoronFieldAccessFlag::AccEnum,
            _ => return Err(ParserError::UnknownFieldAccessFlag(format!("{:#?}", tok))),
        })
    }

    fn get_method_acess_flags(&self, tok: &Token) -> ParserResult<PhoronMethodAccessFlag> {
        Ok(match tok {
            TPublic => PhoronMethodAccessFlag::AccPublic,
            TPrivate => PhoronMethodAccessFlag::AccPrivate,
            TProtected => PhoronMethodAccessFlag::AccProtected,
            TStatic => PhoronMethodAccessFlag::AccStatic,
            TSynchronized => PhoronMethodAccessFlag::AccSynchronized,
            TFinal => PhoronMethodAccessFlag::AccFinal,
            TBridge => PhoronMethodAccessFlag::AccBridge,
            TVarargs => PhoronMethodAccessFlag::AccVarargs,
            TNative => PhoronMethodAccessFlag::AccNative,
            TAbstract => PhoronMethodAccessFlag::AccAbstract,
            TStrict => PhoronMethodAccessFlag::AccStrict,
            TSynthetic => PhoronMethodAccessFlag::AccSynthetic,
            _ => return Err(ParserError::UnknownMethodAccessFlag(format!("{:#?}", tok))),
        })
    }

    /// ClassDef <- CLASS_keyword AccessFlag* ClassName newline
    fn parse_class_def(&mut self) -> ParserResult<PhoronClassDef> {
        self.advance()?;

        let mut access_flags = Vec::new();
        Ok(match self.see() {
            TPublic | TFinal | TSuper | TInterface | TAbstract | TSynthetic | TAnnotation
            | TEnum | TModule => {
                while self.is_class_or_interface_access_flag(self.see()) {
                    access_flags.push(self.get_class_or_interface_access_flag(&self.see())?);
                    self.advance()?;
                }

                if let Token::TIdent(name) = self.see() {
                    let name = name.to_string();
                    self.advance()?;

                    PhoronClassDef { name, access_flags }
                } else {
                    return Err(ParserError::MissingClassName);
                }
            }

            Token::TIdent(name) => {
                let name = name.to_string();
                self.advance()?;

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
            TPublic | TFinal | TSuper | TInterface | TAbstract | TSynthetic | TAnnotation
            | TEnum | TModule => {
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
                self.advance()?;

                let line_number = self.parse_us().map_err(|_| {
                    ParserError::DirectiveError(DirectiveError::LineMissingLineNumber)
                })?;

                PhoronDirective::LineNumber(line_number)
            }

            Token::TVar => {
                self.advance()?;

                let varnum = self
                    .parse_us()
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

    fn parse_ub(&mut self) -> ParserResult<u8> {
        if let Token::TInt(n) = self.see() {
            let n = *n as u8;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::FailedToParsePrimitive)
        }
    }

    fn parse_sb(&mut self) -> ParserResult<i8> {
        if let Token::TInt(n) = self.see() {
            let n = *n as i8;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::FailedToParsePrimitive)
        }
    }

    fn parse_ss(&mut self) -> ParserResult<i16> {
        if let Token::TInt(n) = self.see() {
            let n = *n as i16;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::FailedToParsePrimitive)
        }
    }

    fn parse_us(&mut self) -> ParserResult<u16> {
        if let Token::TInt(n) = self.see() {
            let n = *n as u16;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::FailedToParsePrimitive)
        }
    }

    fn parse_si(&mut self) -> ParserResult<i32> {
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
                    JvmInstructionError::LookupswitchMissingDefault,
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

                let varnum = self.parse_us().map_err(|_| {
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

                let varnum = self.parse_us().map_err(|_| {
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

                let sb = self.parse_sb().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::BipushMissingByte)
                })?;

                JvmInstruction::Bipush(sb)
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

                let varnum = self.parse_us().map_err(|_| {
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

                let varnum = self.parse_us().map_err(|_| {
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

                let varnum = self.parse_us().map_err(|_| {
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

                let varnum = self.parse_us().map_err(|_| {
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

                let varnum = self.parse_us().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::IincMissingOrInvalidVarnum,
                    )
                })?;

                let delta = self.parse_ss().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::IincMissingOrInvalidDelta)
                })?;

                JvmInstruction::Iinc { varnum, delta }
            }

            // iload <varnum>
            Token::TIload => {
                self.advance()?;

                let varnum = self.parse_us().map_err(|_| {
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

            // invokeinterface <method-spec> <n>
            Token::TInvokeinterface => {
                self.advance()?;

                if let Token::TIdent(is_str) = self.see() {
                    if let Some(pos) = is_str.rfind('/') {
                        let interface_name = is_str[..pos].to_owned();
                        let method_name = is_str[pos + 1..].to_owned();

                        self.advance()?;

                        let ub = self.parse_ub().map_err(|_| {
                            ParserError::JvmInstructionError(
                                JvmInstructionError::InvokeinterfaceMissingNumericConstant,
                            )
                        })?;

                        if let Ok(method_descriptor) = self.parse_method_descriptor() {
                            JvmInstruction::Invokeinterface {
                                interface_name,
                                method_name,
                                method_descriptor,
                                ub,
                            }
                        } else {
                            return Err(ParserError::JvmInstructionError(
                                JvmInstructionError::InvokeinterfaceInvalidOrMissingMethodDescriptor,
                            ));
                        }
                    } else {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::InvokeinterfaceMissingMethodName,
                        ));
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::InvokeinterfaceInvalid,
                    ));
                }
            }

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

                let varnum = self.parse_us().map_err(|_| {
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

                let varnum = self.parse_us().map_err(|_| {
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

            // lookupswitch          <-  'lookupswitch'   LookupSwitchPair*  DefaultSwitchPair
            // LookupSwitchPair      <-  Integer          COLON_symbol       Label
            // DefaultSwitchPair     <-  DEFAULT_keyword  COLON_symbol       Label
            Token::TLookupswitch => {
                self.advance()?;

                let switches = self.parse_lookup_switches()?;
                let default = self.parse_default_switch_pair()?;

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

                let varnum = self.parse_us().map_err(|_| {
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

            Token::TMultianewarray => {
                self.advance()?;

                let component_type = self.parse_field_descriptor().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::MultianewarrayMissingOrInvalidComponentType,
                    )
                })?;

                let dimensions = self.parse_ub().map_err(|_| {
                    ParserError::JvmInstructionError(
                        JvmInstructionError::MultianewarrayMissingDimensions,
                    )
                })?;

                JvmInstruction::Multianewarray {
                    component_type,
                    dimensions,
                }
            }

            // new <class>
            Token::TNew => {
                self.advance()?;

                let class_name = self.parse_class_name().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::NewMissingClassName)
                })?;

                JvmInstruction::New { class_name }
            }

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

            // putfield <field-sepc> <descriptor>
            Token::TPutfield => {
                self.advance()?;

                if let Token::TIdent(gf_str) = self.see() {
                    if let Some(pos) = gf_str.rfind('/') {
                        let class_name = gf_str[..pos].to_owned();
                        let field_name = gf_str[pos + 1..].to_owned();

                        self.advance()?;

                        let field_descriptor = self.parse_field_descriptor().map_err(|_| {
                            ParserError::JvmInstructionError(
                                JvmInstructionError::PutfieldMissingOrInvalidFieldDescriptor,
                            )
                        })?;

                        JvmInstruction::Putfield {
                            class_name,
                            field_name,
                            field_descriptor,
                        }
                    } else {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::PutfieldMissingOrInvalidFieldName,
                        ));
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::PutfieldMissingOrInvalidClassName,
                    ));
                }
            }

            // putstatic <field-spec> <descriptor>
            Token::TPutstatic => {
                self.advance()?;

                if let Token::TIdent(gf_str) = self.see() {
                    if let Some(pos) = gf_str.rfind('/') {
                        let class_name = gf_str[..pos].to_owned();
                        let field_name = gf_str[pos + 1..].to_owned();

                        self.advance()?;

                        let field_descriptor = self.parse_field_descriptor().map_err(|_| {
                            ParserError::JvmInstructionError(
                                JvmInstructionError::PutstaticMissingOrInvalidFieldDescriptor,
                            )
                        })?;

                        JvmInstruction::Putstatic {
                            class_name,
                            field_name,
                            field_descriptor,
                        }
                    } else {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::PutstaticMissingOrInvalidFieldName,
                        ));
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::PutstaticMissingOrInvalidClassName,
                    ));
                }
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

                let ss = self.parse_ss().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::SipushMissingConstant)
                })?;

                JvmInstruction::Sipush(ss)
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

                let low = self.parse_si().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::TableswitchMissingLow)
                })?;

                let high = self.parse_si().map_err(|_| {
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
            TThrows | TCatch | TLimit | TVar | TLine => {
                PhoronInstruction::PhoronDirective(self.parse_directive()?)
            }

            TAaload | TAastore | TAconstnull | TAload | TAload0 | TAload1 | TAload2 | TAload3
            | TAnewarray | TAreturn | TArraylength | TAssign | TAstore | TAstore0 | TAstore1
            | TAstore2 | TAstore3 | TAthrow | TBaload | TBastore | TBipush | TBridge | TCaload
            | TCastore | TCheckcast | TD2f | TD2i | TD2l | TDadd | TDaload | TDastore | TDcmpg
            | TDcmpl | TDconst0 | TDconst1 | TDdiv | TDefault | TDload | TDload0 | TDload1
            | TDload2 | TDload3 | TDmul | TDneg | TDot | TDrem | TDreturn | TDstore | TDstore0
            | TDstore1 | TDstore2 | TDstore3 | TDsub | TDup | TDup2 | TDup2x1 | TDup2x2
            | TDupx1 | TDupx2 | TEnum | TF2d | TF2i | TF2l | TFadd | TFaload | TFastore
            | TFcmpg | TFcmpl | TFconst0 | TFconst1 | TFconst2 | TFdiv | TField | TFinal
            | TFload | TFload0 | TFload1 | TFload2 | TFload3 | TFmul | TFneg | TFrem | TFreturn
            | TFrom | TFstore | TFstore0 | TFstore1 | TFstore2 | TFstore3 | TFsub | TGetfield
            | TGetstatic | TGoto | TGotow | TI2b | TI2c | TI2d | TI2f | TI2l | TI2s | TIadd
            | TIaload | TIand | TIastore | TIconst0 | TIconst1 | TIconst2 | TIconst3 | TIconst4
            | TIconst5 | TIconstm1 | TIdiv | TIfacmpeq | TIfacmpne | TIfeq | TIfge | TIfgt
            | TIficmpeq | TIficmpge | TIficmpgt | TIficmple | TIficmplt | TIficmpne | TIfle
            | TIflt | TIfne | TIfnonnull | TIfnull | TIinc | TIload | TIload0 | TIload1
            | TIload2 | TIload3 | TImul | TIneg | TInstanceof | TInvokeinterface
            | TInvokespecial | TInvokestatic | TInvokevirtual | TIor | TIrem | TIreturn | TIshl
            | TIshr | TIstore | TIstore0 | TIstore1 | TIstore2 | TIstore3 | TIsub | TIushr
            | TIxor | TJsr | TJsrw | TL2d | TL2f | TL2i | TLadd | TLand | TLastore | TLcmp
            | TLconst0 | TLconst1 | TLdc | TLdc2w | TLdcw | TLdiv | TLload | TLload0 | TLload1
            | TLload2 | TLload3 | TLmul | TLneg | TLoaload | TLookupswitch | TLor | TLrem
            | TLreturn | TLshl | TLshr | TLstore | TLstore0 | TLstore1 | TLstore2 | TLstore3
            | TLsub | TLushr | TLxor | TMonitorenter | TMonitorexit | TMultianewarray | TNew
            | TNewarray | TNop | TPop | TPop2 | TPutfield | TPutstatic | TRet | TReturn
            | TSaload | TSastore | TSipush | TSuper | TSwap | TTableswitch => {
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
