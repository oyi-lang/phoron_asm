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
    GotoMissingLabel,
    SipushMissingConstant,
    IficmpgtMissingLabel,
    IficmpltMissingLabel,
    IficmpleMissingLabel,
    IloadMissingVarnum,
    IfneMissingLabel,
    AnewarrayInvalidTypeDescriptor(String),
    BipushMissingByte,
    GetfieldInvalid,
    GetfieldInvalidOrMissingFieldDescriptor,
    GetfieldMissingFieldName,
    GetstaticInvalid,
    GetstaticInvalidOrMissingFieldDescriptor,
    GetstaticMissingFieldName,
    IincMissingVarnum,
    IincMissingDelta,
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
                JvmInstructionError::GotoMissingLabel => "goto : missing label".into(),
                JvmInstructionError::SipushMissingConstant =>
                    "sipush : missing constant value".into(),
                JvmInstructionError::IficmpgtMissingLabel => "if_icmpgt : missing label".into(),
                JvmInstructionError::IficmpltMissingLabel => "if_icmplt : missing label".into(),
                JvmInstructionError::IficmpleMissingLabel => "if_icmple : missing label".into(),
                JvmInstructionError::IloadMissingVarnum => "iload : missing var number".into(),
                JvmInstructionError::IfneMissingLabel => "ifne : missing label".into(),
                JvmInstructionError::IincMissingVarnum => "iinc : missing var num".into(),
                JvmInstructionError::IincMissingDelta => "iinc : missing delta".into(),
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
                JvmInstructionError::GetstaticInvalid =>
                    "getstatic : badly-formed or invalid".into(),
                JvmInstructionError::GetstaticInvalidOrMissingFieldDescriptor =>
                    "getstatic : missing or invalid field descriptor".into(),
                JvmInstructionError::GetstaticMissingFieldName =>
                    "getstatic : missing field name".into(),
                JvmInstructionError::LdcIncorrectValue => "ldc : incorrect value".into(),
                JvmInstructionError::LdcwIncorrectValue => "ldcw : incorrect value".into(),
                JvmInstructionError::Ldc2wIncorrectValue => "ldc2_w : incorrect value".into(),
                JvmInstructionError::MultianewarrayMissingDimensions =>
                    "multianewarray : missing dimensions".into(),
                JvmInstructionError::NewMissingClassName => "newa : missing class name".into(),
                JvmInstructionError::NewarrayInvalidType => "newarray : invalid type".into(),
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

            let descriptor = self.parse_field_descriptor()?;
            let init_val = self.parse_field_init_value()?;

            Ok(PhoronFieldDef {
                name,
                access_flags,
                descriptor,
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
                todo!()
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
                let component_type = self.parse_class_or_array_type()?;
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

            Token::TAssign => {
                todo!()
            }

            Token::TAstore => {
                todo!()
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

            Token::TBaload => {
                todo!()
            }

            Token::TBastore => {
                todo!()
            }

            // bipush i8
            Token::TBipush => {
                self.advance()?;

                if let Token::TInt(b) = self.see() {
                    let byte = *b as i8;
                    self.advance()?;
                    JvmInstruction::Bipush(byte)
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::BipushMissingByte,
                    ));
                }
            }

            Token::TBridge => {
                todo!()
            }

            Token::TCaload => {
                todo!()
            }

            Token::TCastore => {
                todo!()
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

            Token::TClass => {
                todo!()
            }

            Token::TColon => {
                todo!()
            }

            Token::TD2f => {
                todo!()
            }

            Token::TD2i => {
                todo!()
            }

            Token::TD2l => {
                todo!()
            }

            Token::TDadd => {
                todo!()
            }

            Token::TDaload => {
                todo!()
            }

            Token::TDastore => {
                todo!()
            }

            Token::TDcmpg => {
                todo!()
            }

            Token::TDcmpl => {
                todo!()
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

            Token::TDdiv => {
                todo!()
            }

            Token::TDefault => {
                todo!()
            }

            Token::TDload => {
                todo!()
            }

            Token::TDload0 => {
                todo!()
            }

            Token::TDload1 => {
                todo!()
            }

            Token::TDload2 => {
                todo!()
            }

            Token::TDload3 => {
                todo!()
            }

            Token::TDmul => {
                todo!()
            }

            Token::TDneg => {
                todo!()
            }

            Token::TDot => {
                todo!()
            }

            Token::TDrem => {
                todo!()
            }

            Token::TDreturn => {
                todo!()
            }

            Token::TDstore => {
                todo!()
            }

            Token::TDstore0 => {
                todo!()
            }

            Token::TDstore1 => {
                todo!()
            }

            Token::TDstore2 => {
                todo!()
            }

            Token::TDstore3 => {
                todo!()
            }

            Token::TDsub => {
                todo!()
            }

            // dup
            Token::TDup => {
                self.advance()?;
                JvmInstruction::Dup
            }

            Token::TDup2 => {
                todo!()
            }

            Token::TDup2x1 => {
                todo!()
            }

            Token::TDup2x2 => {
                todo!()
            }

            Token::TDupx1 => {
                todo!()
            }

            Token::TDupx2 => {
                todo!()
            }

            Token::TEnd => {
                todo!()
            }

            Token::TEndMethod => {
                todo!()
            }

            Token::TEnum => {
                todo!()
            }

            Token::TEof => {
                todo!()
            }

            Token::TF2d => {
                todo!()
            }

            Token::TF2i => {
                todo!()
            }

            Token::TF2l => {
                todo!()
            }

            Token::TFadd => {
                todo!()
            }

            Token::TFaload => {
                todo!()
            }

            Token::TFastore => {
                todo!()
            }

            Token::TFcmpg => {
                todo!()
            }

            Token::TFcmpl => {
                todo!()
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

            Token::TFdiv => {
                todo!()
            }

            Token::TField => {
                todo!()
            }

            Token::TFinal => {
                todo!()
            }

            Token::TFload => {
                todo!()
            }

            Token::TFload0 => {
                todo!()
            }

            Token::TFload1 => {
                todo!()
            }

            Token::TFload2 => {
                todo!()
            }

            Token::TFload3 => {
                todo!()
            }

            Token::TFmul => {
                todo!()
            }

            Token::TFneg => {
                todo!()
            }

            Token::TFrem => {
                todo!()
            }

            Token::TFreturn => {
                todo!()
            }

            Token::TFstore => {
                todo!()
            }

            Token::TFstore0 => {
                todo!()
            }

            Token::TFstore1 => {
                todo!()
            }

            Token::TFstore2 => {
                todo!()
            }

            Token::TFstore3 => {
                todo!()
            }

            Token::TFsub => {
                todo!()
            }

            // getfield <field-spec> <descriptor>
            Token::TGetfield => {
                self.advance()?;

                if let Token::TIdent(gf_str) = self.see() {
                    if let Some(pos) = gf_str.rfind('/') {
                        let class_name = gf_str[..pos].to_owned();
                        let field_name = gf_str[pos + 1..].to_owned();
                        self.advance()?;

                        if let Ok(descriptor) = self.parse_field_descriptor() {
                            JvmInstruction::Getfield {
                                class_name,
                                field_name,
                                descriptor,
                            }
                        } else {
                            return Err(ParserError::JvmInstructionError(
                                JvmInstructionError::GetfieldInvalidOrMissingFieldDescriptor,
                            ));
                        }
                    } else {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::GetfieldMissingFieldName,
                        ));
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::GetfieldInvalid,
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

                        if let Ok(descriptor) = self.parse_field_descriptor() {
                            JvmInstruction::Getstatic {
                                class_name,
                                field_name,
                                descriptor,
                            }
                        } else {
                            return Err(ParserError::JvmInstructionError(
                                JvmInstructionError::GetstaticInvalidOrMissingFieldDescriptor,
                            ));
                        }
                    } else {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::GetstaticMissingFieldName,
                        ));
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::GetstaticInvalid,
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

            Token::TGotow => {
                todo!()
            }

            Token::TI2b => {
                todo!()
            }

            Token::TI2c => {
                todo!()
            }

            Token::TI2d => {
                todo!()
            }

            Token::TI2f => {
                todo!()
            }

            Token::TI2l => {
                todo!()
            }

            Token::TI2s => {
                todo!()
            }

            // iadd
            Token::TIadd => {
                self.advance()?;
                JvmInstruction::Iadd
            }

            Token::TIaload => {
                todo!()
            }

            Token::TIand => {
                todo!()
            }

            Token::TIastore => {
                todo!()
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

            // iconst_m1
            Token::TIconstm1 => {
                self.advance()?;
                JvmInstruction::Iconstm1
            }

            Token::TIdiv => {
                todo!()
            }

            Token::TIfacmpeq => {
                todo!()
            }

            Token::TIfacmpne => {
                todo!()
            }

            Token::TIfeq => {
                todo!()
            }

            Token::TIfge => {
                todo!()
            }

            Token::TIfgt => {
                todo!()
            }

            Token::TIficmpeq => {
                todo!()
            }

            Token::TIficmpge => {
                todo!()
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

            Token::TIficmpne => {
                todo!()
            }

            Token::TIfle => {
                todo!()
            }

            Token::TIflt => {
                todo!()
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

            Token::TIfnonnull => {
                todo!()
            }

            Token::TIfnull => {
                todo!()
            }

            // iinc <varnum> <n>
            Token::TIinc => {
                self.advance()?;

                if let Token::TInt(v) = self.see() {
                    let varnum = *v as u16;
                    self.advance()?;

                    if let Token::TInt(n) = self.see() {
                        let delta = *n as i16;
                        self.advance()?;

                        JvmInstruction::Iinc { varnum, delta }
                    } else {
                        return Err(ParserError::JvmInstructionError(
                            JvmInstructionError::IincMissingDelta,
                        ));
                    }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::IincMissingVarnum,
                    ));
                }
            }

            // iload <varnum>
            Token::TIload => {
                self.advance()?;

                if let Token::TInt(varnum) = self.see() {
                    let varnum = *varnum as u16;
                    self.advance()?;

                    JvmInstruction::Iload { varnum }
                } else {
                    return Err(ParserError::JvmInstructionError(
                        JvmInstructionError::IloadMissingVarnum,
                    ));
                }
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

            Token::TIneg => {
                todo!()
            }

            // instanceof
            Token::TInstanceof => {
                self.advance()?;
                let check_type = self.parse_class_or_array_type()?;
                JvmInstruction::Instanceof { check_type }
            }

            Token::TInterface => {
                todo!()
            }

            Token::TInvokeinterface => {
                todo!()
            }

            // invokespecial <method-spec>
            Token::TInvokespecial => {
                self.advance()?;

                if let Token::TIdent(is_str) = self.see() {
                    if let Some(pos) = is_str.rfind('/') {
                        let class_name = is_str[..pos].to_owned();
                        let method_name = is_str[pos + 1..].to_owned();
                        self.advance()?;

                        if let Ok(descriptor) = self.parse_method_descriptor() {
                            JvmInstruction::Invokespecial {
                                class_name,
                                method_name,
                                descriptor,
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

                        if let Ok(descriptor) = self.parse_method_descriptor() {
                            JvmInstruction::Invokestatic {
                                class_name,
                                method_name,
                                descriptor,
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

                        if let Ok(descriptor) = self.parse_method_descriptor() {
                            JvmInstruction::Invokevirtual {
                                class_name,
                                method_name,
                                descriptor,
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

            Token::TIor => {
                todo!()
            }

            Token::TIrem => {
                todo!()
            }

            // ireturn
            Token::TIreturn => {
                self.advance()?;
                JvmInstruction::Ireturn
            }

            Token::TIshl => {
                todo!()
            }

            Token::TIshr => {
                todo!()
            }

            Token::TIstore => {
                todo!()
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

            Token::TIushr => {
                todo!()
            }

            Token::TIxor => {
                todo!()
            }

            // jsr <label>
            Token::TJsr => {
                self.advance()?;
                let label = self.parse_label().map_err(|_| {
                    ParserError::JvmInstructionError(JvmInstructionError::JsrMissingLabel)
                })?;
                JvmInstruction::Jsr { label }
            }

            Token::TJsrw => {
                todo!()
            }

            Token::TL2d => {
                todo!()
            }

            Token::TL2f => {
                todo!()
            }

            Token::TL2i => {
                todo!()
            }

            Token::TLadd => {
                todo!()
            }

            Token::TLand => {
                todo!()
            }

            Token::TLastore => {
                todo!()
            }

            Token::TLcmp => {
                todo!()
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

            Token::TLdiv => {
                todo!()
            }

            Token::TLeftParen => {
                todo!()
            }

            Token::TLimit => {
                todo!()
            }

            Token::TLine => {
                todo!()
            }

            Token::TLload => {
                todo!()
            }

            Token::TLload0 => {
                todo!()
            }

            Token::TLload1 => {
                todo!()
            }

            Token::TLload2 => {
                todo!()
            }

            Token::TLload3 => {
                todo!()
            }

            Token::TLmul => {
                todo!()
            }

            Token::TLneg => {
                todo!()
            }

            Token::TLoaload => {
                todo!()
            }

            Token::TLocals => {
                todo!()
            }

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

            Token::TLor => {
                todo!()
            }

            Token::TLrem => {
                todo!()
            }

            Token::TLreturn => {
                todo!()
            }

            Token::TLshl => {
                todo!()
            }

            Token::TLshr => {
                todo!()
            }

            Token::TLstore => {
                todo!()
            }

            Token::TLstore0 => {
                todo!()
            }

            Token::TLstore1 => {
                todo!()
            }

            Token::TLstore2 => {
                todo!()
            }

            Token::TLstore3 => {
                todo!()
            }

            Token::TLsub => {
                todo!()
            }

            Token::TLushr => {
                todo!()
            }

            Token::TLxor => {
                todo!()
            }

            Token::TMethod => {
                todo!()
            }

            Token::TModule => {
                todo!()
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

            Token::TNative => {
                todo!()
            }

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

            Token::TNop => {
                todo!()
            }

            // pop
            Token::TPop => {
                self.advance()?;
                JvmInstruction::Pop
            }

            Token::TPop2 => {
                todo!()
            }

            Token::TPrivate => {
                todo!()
            }

            Token::TProtected => {
                todo!()
            }

            Token::TPublic => {
                todo!()
            }

            Token::TPutfield => {
                todo!()
            }

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

            Token::TReturn => {
                self.advance()?;
                JvmInstruction::Return
            }

            Token::TRightParen => {
                todo!()
            }

            Token::TSaload => {
                todo!()
            }

            Token::TSastore => {
                todo!()
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

            Token::TSource => {
                todo!()
            }

            Token::TStack => {
                todo!()
            }

            Token::TStatic => {
                todo!()
            }

            Token::TStrict => {
                todo!()
            }

            Token::TSuper => {
                todo!()
            }

            // swap
            Token::TSwap => {
                self.advance()?;
                JvmInstruction::Swap
            }

            Token::TSynchronized => {
                todo!()
            }

            Token::TSynthetic => {
                todo!()
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

            Token::TTransient => {
                todo!()
            }

            Token::TVarargs => {
                todo!()
            }

            Token::TVolatile => {
                todo!()
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
            | Token::TClass
            | Token::TColon
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
            | Token::TEnd
            | Token::TEndMethod
            | Token::TEnum
            | Token::TEof
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
            | Token::TInterface
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
            | Token::TLeftParen
            | Token::TLload
            | Token::TLload0
            | Token::TLload1
            | Token::TLload2
            | Token::TLload3
            | Token::TLmul
            | Token::TLneg
            | Token::TLoaload
            | Token::TLocals
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
            | Token::TMethod
            | Token::TModule
            | Token::TMonitorenter
            | Token::TMonitorexit
            | Token::TMultianewarray
            | Token::TNative
            | Token::TNew
            | Token::TNewarray
            | Token::TNop
            | Token::TPop
            | Token::TPop2
            | Token::TPrivate
            | Token::TProtected
            | Token::TPublic
            | Token::TPutfield
            | Token::TPutstatic
            | Token::TRet
            | Token::TReturn
            | Token::TRightParen
            | Token::TSaload
            | Token::TSastore
            | Token::TSipush
            | Token::TSource
            | Token::TStack
            | Token::TStatic
            | Token::TStrict
            | Token::TSuper
            | Token::TSwap
            | Token::TSynchronized
            | Token::TSynthetic
            | Token::TTableswitch
            | Token::TTransient
            | Token::TVarargs
            | Token::TVolatile => PhoronInstruction::JvmInstruction(self.parse_jvm_instruction()?),

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

            let descriptor = self.parse_method_descriptor()?;
            let instructions = self.parse_instructions()?;

            Ok(PhoronMethodDef {
                name,
                access_flags,
                descriptor,
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
