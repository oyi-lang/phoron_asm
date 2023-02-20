use crate::{
    ast::*,
    lexer::{Lexer, LexerError, Token},
};

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    LexerError(LexerError),
    InvalidToken(String),
    IllegalLabelError,
    InvalidFieldDescriptor,
    EmptyFieldDescriptor,
    InvalidFieldInitValue(String),
    InvalidMethodDescriptor,
    InvalidInstruction(String),
    InvalidJvmInstruction(String),
    MissingClassName,
    MissingInterfaceName,
    MissingEndMethodMarker,
    UnknownClassOrInterfaceAccessFlag(String),
    MissingSuperclassName,
    MissingFieldName,
    MissingMethodName,
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
                ParserError::LexerError(ref err) => err.to_string(),
                ParserError::IllegalLabelError => "malformed label - missing semicolon".into(),
                ParserError::InvalidToken(ref tokstr) =>
                    format!("Invalid token at position: {:#?}", tokstr),
                ParserError::InvalidFieldDescriptor => "invalid field descriptor".into(),
                ParserError::EmptyFieldDescriptor => "empty field descriptor".into(),
                ParserError::InvalidFieldInitValue(ref tokstr) =>
                    format!("invalid field initialisation value: {:#?}", tokstr),
                ParserError::InvalidMethodDescriptor => "invalid method descriptor".into(),
                ParserError::InvalidInstruction(ref instr) =>
                    format!("invalid phpron instuction: {instr}"),
                ParserError::InvalidJvmInstruction(ref instr) =>
                    format!("invalid JVM instruction {instr}"),
                ParserError::MissingClassName => "missing class name".into(),
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
            }
        )
    }
}

impl From<LexerError> for ParserError {
    fn from(lex_err: LexerError) -> Self {
        ParserError::LexerError(lex_err)
    }
}

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

    fn see(&self) -> &Token {
        &self.curr_tok
    }

    fn parse_sourcefile_def(&mut self) -> ParserResult<PhoronSourceFileDef> {
        todo!()
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
                "[" => {
                    self.advance()?;
                    let component_type = self.parse_field_descriptor()?;
                    PhoronFieldDescriptor::ArrayType {
                        component_type: Box::new(component_type),
                    }
                }
                _ => return Err(ParserError::InvalidFieldDescriptor),
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

    /// Directive <- (LIMIT_keyword (StackDirective / LocalDirective) / ThrowsDirective / LineNumberDirective / VarDirective / CatchDirective) newline
    /// StackDirective <-  STACK_keyword Integer
    /// LocalDirective <- LOCAL_keyword Integer
    /// ThrowsDirective <- THROWS_keyword ClassName
    /// LineNumberDirective <- LINE_keyword Integer
    /// VarDirective <- VAR_keyword Integer IS_keyword VarName FieldDescriptor FROM_keyword Label TO_keyword Label
    /// CatchDirective <- CATCH_keyword ClassName FROM_keyword Label TO_keyword Label USING_keyword Label
    fn parse_directive(&mut self) -> ParserResult<PhoronDirective> {
        todo!()
    }

    fn parse_jvm_instruction(&mut self) -> ParserResult<JvmInstruction> {
        Ok(match self.see() {
            Token::TAaload => {
                todo!()
            }

            Token::TAastore => {
                todo!()
            }

            Token::TAbstract => {
                todo!()
            }

            Token::TAconstnull => {
                todo!()
            }

            Token::TAload => {
                todo!()
            }

            Token::TAload0 => {
                self.advance()?;
                JvmInstruction::Aload0
            }

            Token::TAload1 => {
                todo!()
            }

            Token::TAload2 => {
                todo!()
            }

            Token::TAload3 => {
                todo!()
            }

            Token::TAnewarray => {
                todo!()
            }

            Token::TAnnotation => {
                todo!()
            }

            Token::TAreturn => {
                todo!()
            }

            Token::TArraylength => {
                todo!()
            }

            Token::TAssign => {
                todo!()
            }

            Token::TAstore => {
                todo!()
            }

            Token::TAstore0 => {
                todo!()
            }

            Token::TAstore1 => {
                todo!()
            }

            Token::TAstore2 => {
                todo!()
            }

            Token::TAstore3 => {
                todo!()
            }

            Token::TAthrow => {
                todo!()
            }

            Token::TBaload => {
                todo!()
            }

            Token::TBastore => {
                todo!()
            }

            Token::TBipush => {
                todo!()
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

            Token::TCatch => {
                todo!()
            }

            Token::TCheckcast => {
                todo!()
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

            Token::TDconst0 => {
                todo!()
            }

            Token::TDconst1 => {
                todo!()
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

            Token::TDup => {
                todo!()
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

            Token::TFconst0 => {
                todo!()
            }

            Token::TFconst1 => {
                todo!()
            }

            Token::TFconst2 => {
                todo!()
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

            Token::TFrom => {
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

            Token::TGetfield => {
                todo!()
            }

            Token::TGetstatic => {
                todo!()
            }

            Token::TGoto => {
                todo!()
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

            Token::TIadd => {
                todo!()
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

            Token::TIconst0 => {
                todo!()
            }

            Token::TIconst1 => {
                todo!()
            }

            Token::TIconst2 => {
                todo!()
            }

            Token::TIconst3 => {
                todo!()
            }

            Token::TIconst4 => {
                todo!()
            }

            Token::TIconst5 => {
                todo!()
            }

            Token::TIconstm1 => {
                todo!()
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

            Token::TIficmpgt => {
                todo!()
            }

            Token::TIficmple => {
                todo!()
            }

            Token::TIficmplt => {
                todo!()
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

            Token::TIfne => {
                todo!()
            }

            Token::TIfnonnull => {
                todo!()
            }

            Token::TIfnull => {
                todo!()
            }

            Token::TIinc => {
                todo!()
            }

            Token::TIload => {
                todo!()
            }

            Token::TIload0 => {
                todo!()
            }

            Token::TIload1 => {
                todo!()
            }

            Token::TIload2 => {
                todo!()
            }

            Token::TIload3 => {
                todo!()
            }

            Token::TImul => {
                todo!()
            }

            Token::TIneg => {
                todo!()
            }

            Token::TInstanceof => {
                todo!()
            }

            Token::TInterface => {
                todo!()
            }

            Token::TInvokeinterface => {
                todo!()
            }

            Token::TInvokespecial => {
                todo!()
            }

            Token::TInvokestatic => {
                todo!()
            }

            Token::TInvokevirtual => {
                todo!()
            }

            Token::TIor => {
                todo!()
            }

            Token::TIrem => {
                todo!()
            }

            Token::TIreturn => {
                todo!()
            }

            Token::TIs => {
                todo!()
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

            Token::TIstore0 => {
                todo!()
            }

            Token::TIstore1 => {
                todo!()
            }

            Token::TIstore2 => {
                todo!()
            }

            Token::TIstore3 => {
                todo!()
            }

            Token::TIsub => {
                todo!()
            }

            Token::TIushr => {
                todo!()
            }

            Token::TIxor => {
                todo!()
            }

            Token::TJsr => {
                todo!()
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

            Token::TLconst0 => {
                todo!()
            }

            Token::TLconst1 => {
                todo!()
            }

            Token::TLdc => {
                todo!()
            }

            Token::TLdc2w => {
                todo!()
            }

            Token::TLdcw => {
                todo!()
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

            Token::TLookupswitch => {
                todo!()
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

            Token::TMinus => {
                todo!()
            }

            Token::TModule => {
                todo!()
            }

            Token::TMonitorenter => {
                todo!()
            }

            Token::TMonitorexit => {
                todo!()
            }

            Token::TMultianewarray => {
                todo!()
            }

            Token::TNative => {
                todo!()
            }

            Token::TNew => {
                todo!()
            }

            Token::TNewarray => {
                todo!()
            }

            Token::TNop => {
                todo!()
            }

            Token::TPlus => {
                todo!()
            }

            Token::TPop => {
                todo!()
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

            Token::TRet => {
                todo!()
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

            Token::TSipush => {
                todo!()
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

            Token::TSwap => {
                todo!()
            }

            Token::TSynchronized => {
                todo!()
            }

            Token::TSynthetic => {
                todo!()
            }

            Token::TTableswitch => {
                todo!()
            }

            Token::TThrows => {
                todo!()
            }

            Token::TTo => {
                todo!()
            }

            Token::TTransient => {
                todo!()
            }

            Token::TUsing => {
                todo!()
            }

            Token::TVar => {
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
            Token::TLimit => {
                self.advance()?;
                PhoronInstruction::PhoronDirective(self.parse_directive()?)
            }

            Token::TAaload
            | Token::TAastore
            | Token::TAbstract
            | Token::TAconstnull
            | Token::TAload
            | Token::TAload0
            | Token::TAload1
            | Token::TAload2
            | Token::TAload3
            | Token::TAnewarray
            | Token::TAnnotation
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
            | Token::TCatch
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
            | Token::TIs
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
            | Token::TLimit
            | Token::TLine
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
            | Token::TMinus
            | Token::TModule
            | Token::TMonitorenter
            | Token::TMonitorexit
            | Token::TMultianewarray
            | Token::TNative
            | Token::TNew
            | Token::TNewarray
            | Token::TNop
            | Token::TPlus
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
            | Token::TThrows
            | Token::TTo
            | Token::TTransient
            | Token::TUsing
            | Token::TVar
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
        // param_descriptor: Option<PhoronFieldDescriptor>,
        // return_descriptor: PhoronReturnDescriptor,
        //
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
            println!("name = {name:#?}, descriptor = {descriptor:#?}");
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

        println!("methods = {method_defs:#?}");

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

    /// Header <- SourceFileDef? (ClassDef / InterfaceDef) SuperDef
    fn parse_header(&mut self) -> ParserResult<PhoronHeader> {
        self.advance()?;

        Ok(match self.see() {
            Token::TSource => {
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
