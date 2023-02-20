use crate::{
    ast::*,
    lexer::{Lexer, LexerError, Token},
};

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    LexerError(LexerError),
    InvalidToken(String),
    InvalidFieldDescriptor,
    EmptyFieldDescriptor,
    InvalidFieldInitValue(String),
    InvalidMethodDescriptor,
    MissingClassName,
    MissingInterfaceName,
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
                ParserError::InvalidToken(ref tokstr) =>
                    format!("Invalid token at position: {:#?}", tokstr),
                ParserError::InvalidFieldDescriptor => "invalid field descriptor".into(),
                ParserError::EmptyFieldDescriptor => "empty field descriptor".into(),
                ParserError::InvalidFieldInitValue(ref tokstr) =>
                    format!("invalid field initialisation value: {:#?}", tokstr),
                ParserError::InvalidMethodDescriptor => "invalid method descriptor".into(),
                ParserError::MissingClassName => "missing class name".into(),
                ParserError::MissingInterfaceName => "missing interface name".into(),
                ParserError::MissingSuperclassName => "missing super class name".into(),
                ParserError::MissingFieldName => "field name missing in definition".into(),
                ParserError::MissingMethodName => "method name missing in definition".into(),
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

    //PhoronDirective,
    //JvmInstruction,
    //PhoronLabel(String),

    /// Instruction <- line_comment* (Directive / JvmInstruction / Label) line_comment?  newline
    fn parse_instructions(&mut self) -> ParserResult<Vec<PhoronInstruction>> {
        println!("curr tok = {:#?}", self.see());
        todo!()
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
