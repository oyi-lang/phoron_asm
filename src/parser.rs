use crate::{
    ast::{
        PhoronBody, PhoronClassDef, PhoronClassOrInterface, PhoronClassOrInterfaceAccessFlag,
        PhoronFieldDef, PhoronHeader, PhoronInterfaceDef, PhoronMethodDef, PhoronProgram,
        PhoronSourceFileDef, PhoronSuperDef,
    },
    lexer::{Lexer, LexerError, Token},
};

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    LexerError(LexerError),
    InvalidToken(String),
    MissingClassName,
    MissingInterfaceName,
    UnknownClassOrInterfaceAccessFlag(String),
    MissingSuperclassName,
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
                ParserError::MissingClassName => "missing class name".into(),
                ParserError::MissingInterfaceName => "missing interface name".into(),
                ParserError::MissingSuperclassName => "missing super class name".into(),
                ParserError::UnknownClassOrInterfaceAccessFlag(ref flag) =>
                    format!("Invalid flag for class/interface: {flag}"),
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

    fn consume(&mut self) -> ParserResult<()> {
        self.curr_tok = self.lexer.lex()?;
        Ok(())
    }

    fn see(&self) -> &Token {
        &self.curr_tok
    }

    fn parse_header(&mut self) -> ParserResult<PhoronHeader> {
        self.consume()?;

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

    fn parse_class_def(&mut self) -> ParserResult<PhoronClassDef> {
        self.consume()?;

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
                    self.consume()?;
                }

                if let Token::TIdent(ident) = self.see() {
                    let name = ident.to_string();
                    self.consume()?;

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

    fn parse_interface_def(&mut self) -> ParserResult<PhoronInterfaceDef> {
        todo!()
    }

    fn parse_super_def(&mut self) -> ParserResult<PhoronSuperDef> {
        self.consume()?;

        if let Token::TIdent(ident) = self.see() {
            let super_class_name = ident.to_string();
            self.consume()?;

            Ok(PhoronSuperDef { super_class_name })
        } else {
            Err(ParserError::MissingSuperclassName)
        }
    }

    fn parse_field_defs(&mut self) -> ParserResult<Vec<PhoronFieldDef>> {
        let mut field_defs = Vec::new();

        while self.see() == Token::TField {
            self.consume()?;

            let mut field_access_flags = Vec::new();

        }

        todo!()
    }

    fn parse_method_defs(&mut self) -> ParserResult<Vec<PhoronMethodDef> {
        todo!()
    }

    fn parse_body(&mut self) -> ParserResult<PhoronBody> {
        let field_defs = self.parse_field_defs()?;
        let method_defs = self.parse_method_defs()?;

        Ok(PhoronBody {
            field_defs,
            method_defs,
        })
    }

    pub fn parse(&mut self) -> ParserResult<PhoronProgram> {
        let header = self.parse_header()?;
        let body = self.parse_body()?;

        Ok(PhoronProgram { header, body })
    }
}
