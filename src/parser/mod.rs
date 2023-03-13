#![cfg(target_of = "linux")]

use crate::{
    ast::*,
    lexer::{Lexer, LexerError, Token, TokenKind},
    sourcefile::{Pos, Span},
};

mod type_descriptor_parser;

use type_descriptor_parser as tdp;

#[derive(Debug)]
pub enum ParserError {
    Missing {
        instr: &'static str,
        component: &'static str,
    },

    IncorrectTypeOrValue {
        instr: &'static str,
        type_or_val: &'static str,
    },

    Malformed {
        component: &'static str,
        details: &'static str,
    },

    Unknown {
        component: &'static str,
    },

    LexerError(LexerError),
}

impl std::error::Error for ParserError {}

use std::fmt;
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParserError::*;

        write!(
            f,
            "{}",
            match *self {
                Missing {
                    ref instr,
                    ref component,
                } => format!("{} : {} {}", instr, "missing or invalid ", component),

                IncorrectTypeOrValue {
                    ref instr,
                    ref type_or_val,
                } => format!(
                    "{} : {} {}",
                    instr, "incorrect type or value of ", type_or_val
                ),

                Malformed {
                    ref component,
                    ref details,
                } => format!("Malformed {}: {}", component, details),
                Unknown { ref component } => format!("Unknown {component}"),

                LexerError(ref err) => err.to_string(),
            }
        )
    }
}

impl From<LexerError> for ParserError {
    fn from(lex_err: LexerError) -> Self {
        ParserError::LexerError(lex_err)
    }
}

pub type ParserResult<T> = Result<T, ParserError>;

use TokenKind::*;

/// The Phoron parser
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_tok: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer,
            curr_tok: Token {
                kind: TokenKind::TEof,
                span: Span::default(),
            },
        }
    }

    fn advance(&mut self) -> ParserResult<()> {
        self.curr_tok = self.lexer.lex()?;
        Ok(())
    }

    fn advance_if(&mut self, expected_token: &TokenKind) -> ParserResult<()> {
        if self.see() != expected_token {
            Err(ParserError::IncorrectTypeOrValue {
                instr: "parsing tokens",
                type_or_val: "token",
            })
        } else {
            self.advance()?;
            Ok(())
        }
    }

    fn see(&self) -> &Token {
        &self.curr_tok
    }

    fn is_class_or_interface_access_flag(&self, tok: &TokenKind) -> bool {
        match tok {
            TPublic | TFinal | TSuper | TInterface | TAbstract | TSynthetic | TAnnotation
            | TEnum | TModule => true,
            _ => false,
        }
    }

    fn is_field_access_flag(&self, tok: &TokenKind) -> bool {
        match tok {
            TPublic | TPrivate | TProtected | TStatic | TFinal | TVolatile | TTransient
            | TSynthetic | TEnum => true,
            _ => false,
        }
    }

    fn is_method_access_flag(&self, tok: &TokenKind) -> bool {
        match tok {
            TPublic | TPrivate | TProtected | TStatic | TFinal | TSynthetic | TSynchronized
            | TBridge | TVarargs | TNative | TAbstract | TStrict => true,
            _ => false,
        }
    }

    fn get_class_or_interface_access_flag(
        &self,
        tok: &TokenKind,
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
                return Err(ParserError::Unknown {
                    component: "class or interface flag",
                })
            }
        })
    }

    fn get_field_access_flags(&self, tok: &TokenKind) -> ParserResult<PhoronFieldAccessFlag> {
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
            _ => {
                return Err(ParserError::Unknown {
                    component: "field access flag",
                })
            }
        })
    }

    fn get_method_acess_flags(&self, tok: &TokenKind) -> ParserResult<PhoronMethodAccessFlag> {
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
            _ => {
                return Err(ParserError::Unknown {
                    component: "method access flag",
                })
            }
        })
    }

    /// ClassDef <- CLASS_keyword AccessFlag* ClassName newline
    fn parse_class_def(&mut self) -> ParserResult<PhoronClassDef> {
        self.advance()?;

        let mut access_flags = vec![self.get_class_or_interface_access_flag(&TSuper)?];
        Ok(match self.see() {
            TPublic | TFinal | TSuper | TInterface | TAbstract | TSynthetic | TAnnotation
            | TEnum | TModule => {
                while self.is_class_or_interface_access_flag(self.see()) {
                    access_flags.push(self.get_class_or_interface_access_flag(&self.see())?);
                    self.advance()?;
                }

                if let TokenKind::TIdent(name) = self.see() {
                    let name = name.to_string();
                    self.advance()?;

                    PhoronClassDef { name, access_flags }
                } else {
                    return Err(ParserError::Missing {
                        instr: "parsing class definition",
                        component: "class name",
                    });
                }
            }

            TokenKind::TIdent(name) => {
                let name = name.to_string();
                self.advance()?;

                PhoronClassDef { name, access_flags }
            }

            _ => {
                return Err(ParserError::IncorrectTypeOrValue {
                    instr: "parsing class defiition",
                    type_or_val: "token",
                })
            }
        })
    }

    /// InterfaceDef <- INTERFACE_keyword AccessFlag* ClassName newline
    fn parse_interface_def(&mut self) -> ParserResult<PhoronInterfaceDef> {
        self.advance()?;

        let mut access_flags = vec![self.get_class_or_interface_access_flag(&TAbstract)?];

        Ok(match self.see() {
            TPublic | TFinal | TSuper | TInterface | TAbstract | TSynthetic | TAnnotation
            | TEnum | TModule => {
                while self.is_class_or_interface_access_flag(self.see()) {
                    access_flags.push(self.get_class_or_interface_access_flag(&self.see())?);
                    self.advance()?;
                }

                if let TokenKind::TIdent(ident) = self.see() {
                    let name = ident.to_string();
                    self.advance()?;

                    PhoronInterfaceDef { name, access_flags }
                } else {
                    return Err(ParserError::Missing {
                        instr: "parsing interface definition",
                        component: "interface name",
                    });
                }
            }

            TokenKind::TIdent(ident) => {
                let name = ident.to_string();
                PhoronInterfaceDef { name, access_flags }
            }

            tok => {
                return Err(ParserError::IncorrectTypeOrValue {
                    instr: "parsing interface definition",
                    type_or_val: "token",
                })
            }
        })
    }

    /// ImplementsDef <- IMPLEMENTS_keyword ClassName newline
    fn parse_implements_def(&mut self) -> ParserResult<PhoronImplementsDef> {
        self.advance()?;

        if let TokenKind::TIdent(ident) = self.see() {
            let class_name = ident.to_string();
            self.advance()?;

            Ok(PhoronImplementsDef { class_name })
        } else {
            Err(ParserError::Missing {
                instr: "parsing implements defintiion",
                component: "implements class name",
            })
        }
    }

    fn parse_implements_defs(&mut self) -> ParserResult<Vec<PhoronImplementsDef>> {
        let mut impl_defs = Vec::new();

        while let TokenKind::TImplements = self.see() {
            impl_defs.push(self.parse_implements_def()?);
        }

        Ok(impl_defs)
    }

    /// SuperDef <- SUPER_keyword ClassName newline
    fn parse_super_def(&mut self) -> ParserResult<PhoronSuperDef> {
        self.advance()?;

        if let TokenKind::TIdent(ident) = self.see() {
            let super_class_name = ident.to_string();
            self.advance()?;

            Ok(PhoronSuperDef { super_class_name })
        } else {
            Err(ParserError::Missing {
                instr: "parsing superclass definition",
                component: "super class name",
            })
        }
    }

    /// FieldIniValue <- Double / Integer / QuotedString
    fn parse_field_init_value(&mut self) -> ParserResult<Option<PhoronFieldInitValue>> {
        if let TokenKind::TAssign = self.see() {
            self.advance()?;

            Ok(if let TokenKind::TInt(int) = self.see() {
                let ival = *int;
                self.advance()?;
                Some(PhoronFieldInitValue::Integer(ival))
            } else if let TokenKind::TFloat(float) = self.see() {
                let fval = *float;
                self.advance()?;
                Some(PhoronFieldInitValue::Double(fval))
            } else if let TokenKind::TString(s) = self.see() {
                let sval = s.to_owned();
                self.advance()?;
                Some(PhoronFieldInitValue::QuotedString(sval))
            } else {
                return Err(ParserError::IncorrectTypeOrValue {
                    instr: "parsing field initialisation value",
                    type_or_val: "init value",
                });
            })
        } else {
            Ok(None)
        }
    }

    fn parse_field_descriptor(&mut self) -> ParserResult<PhoronFieldDescriptor> {
        if let TokenKind::TIdent(ident) = self.see() {
            let mut field_desc_parser = tdp::TypeParser::new(&ident);
            let field_desc = field_desc_parser.parse_field_descriptor().map_err(|err| {
                ParserError::Malformed {
                    component: "field descriptor",
                    details: "invalid or malfomed field type descriptor",
                }
            })?;
            self.advance()?;

            Ok(field_desc)
        } else {
            Err(ParserError::Malformed {
                component: "field descriptor",
                details: "nvalid token for field descriptor",
            })
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

        if let TokenKind::TIdent(ident) = self.see() {
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
            Err(ParserError::Malformed {
                component: "field definition",
                details: "malformed field definition",
            })
        }
    }

    fn parse_field_defs(&mut self) -> ParserResult<Vec<PhoronFieldDef>> {
        let mut field_defs = Vec::new();
        while let TokenKind::TField = self.see() {
            field_defs.push(self.parse_field_def()?);
        }

        Ok(field_defs)
    }

    fn parse_class_name(&mut self) -> ParserResult<String> {
        if let TokenKind::TIdent(classname) = self.see() {
            let classname = classname.to_owned();
            self.advance()?;

            Ok(classname)
        } else {
            Err(ParserError::Malformed {
                component: "class name",
                details: "malformed class name",
            })
        }
    }

    fn parse_label(&mut self) -> ParserResult<String> {
        if let TokenKind::TIdent(label) = self.see() {
            let label = label.to_owned();
            self.advance()?;

            Ok(label)
        } else {
            Err(ParserError::Malformed {
                component: "label",
                details: "malformed label",
            })
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
            TokenKind::TLimit => {
                self.advance()?;

                match self.see() {
                    TokenKind::TStack => {
                        self.advance()?;

                        if let TokenKind::TInt(n) = self.see() {
                            let max_stack = *n as u16;
                            self.advance()?;
                            PhoronDirective::LimitStack(max_stack)
                        } else {
                            return Err(ParserError::Missing {
                                instr: ".limit stack",
                                component: "count",
                            });
                        }
                    }

                    TokenKind::TLocals => {
                        self.advance()?;

                        if let TokenKind::TInt(n) = self.see() {
                            let max_locals = *n as u16;
                            self.advance()?;
                            PhoronDirective::LimitLocals(max_locals)
                        } else {
                            return Err(ParserError::Missing {
                                instr: ".limit locals",
                                component: "count",
                            });
                        }
                    }
                    _ => {
                        return Err(ParserError::IncorrectTypeOrValue {
                            instr: ".limit",
                            type_or_val: "stack and/or locals",
                        })
                    }
                }
            }

            TokenKind::TThrows => {
                self.advance()?;

                let class_name = self.parse_class_name().map_err(|_| ParserError::Missing {
                    instr: ".throws",
                    component: "class name",
                })?;

                PhoronDirective::Throws { class_name }
            }

            TokenKind::TLine => {
                self.advance()?;

                let line_number = self.parse_us().map_err(|_| ParserError::Missing {
                    instr: ".line",
                    component: "line number",
                })?;

                PhoronDirective::LineNumber(line_number)
            }

            TokenKind::TVar => {
                self.advance()?;

                let varnum = self.parse_us().map_err(|_| ParserError::Missing {
                    instr: ".var",
                    component: "var num",
                })?;

                self.advance_if(&TokenKind::TIs)
                    .map_err(|_| ParserError::Missing {
                        instr: ".var",
                        component: "`is` keyword",
                    })?;

                let name = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: ".var",
                    component: "name",
                })?;

                let field_descriptor =
                    self.parse_field_descriptor()
                        .map_err(|_| ParserError::Missing {
                            instr: ".var",
                            component: "field descriptor",
                        })?;

                self.advance_if(&TokenKind::TFrom)
                    .map_err(|_| ParserError::Missing {
                        instr: ".var",
                        component: "`from` keyword",
                    })?;

                let from_label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: ".var",
                    component: "from label",
                })?;

                self.advance_if(&TokenKind::TTo)
                    .map_err(|_| ParserError::Missing {
                        instr: ".var",
                        component: "`to` keyword",
                    })?;

                let to_label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: ".var",
                    component: "to label",
                })?;

                PhoronDirective::Var {
                    varnum,
                    name,
                    field_descriptor,
                    from_label,
                    to_label,
                }
            }

            TokenKind::TCatch => {
                self.advance()?;

                let class_name = self.parse_class_name()?;
                self.advance_if(&TokenKind::TFrom)?;
                let from_label = self.parse_label()?;
                self.advance_if(&TokenKind::TTo)?;
                let to_label = self.parse_label()?;
                self.advance_if(&TokenKind::TUsing)?;
                let handler_label = self.parse_label()?;

                PhoronDirective::Catch {
                    class_name,
                    from_label,
                    to_label,
                    handler_label,
                }
            }
            _ => {
                return Err(ParserError::Malformed {
                    component: "Phoron directive",
                    details: "invalid Phoron directive",
                })
            }
        })
    }

    fn parse_ub(&mut self) -> ParserResult<u8> {
        if let TokenKind::TInt(n) = self.see() {
            let n = *n as u8;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::Malformed {
                component: "unsigned byte",
                details: "invalid unsigned byte",
            })
        }
    }

    fn parse_sb(&mut self) -> ParserResult<i8> {
        if let TokenKind::TInt(n) = self.see() {
            let n = *n as i8;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::Malformed {
                component: "signed byte",
                details: "invalid signed byte",
            })
        }
    }

    fn parse_us(&mut self) -> ParserResult<u16> {
        if let TokenKind::TInt(n) = self.see() {
            let n = *n as u16;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::Malformed {
                component: "unsigned short",
                details: "invalid unsigned short",
            })
        }
    }

    fn parse_ss(&mut self) -> ParserResult<i16> {
        if let TokenKind::TInt(n) = self.see() {
            let n = *n as i16;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::Malformed {
                component: "signed short",
                details: "invalid signed short",
            })
        }
    }

    fn parse_si(&mut self) -> ParserResult<i32> {
        if let TokenKind::TInt(n) = self.see() {
            let n = *n as i32;
            self.advance()?;

            Ok(n)
        } else {
            Err(ParserError::Malformed {
                component: "signed integer",
                details: "invalid signed integer",
            })
        }
    }

    fn parse_table_switches(&mut self) -> ParserResult<Vec<String>> {
        let mut switches = Vec::new();

        while let TokenKind::TIdent(label) = self.see() {
            let label = label.to_string();
            self.advance()?;

            switches.push(label);
        }

        Ok(switches)
    }

    fn parse_lookup_switches(&mut self) -> ParserResult<Vec<LookupSwitchPair>> {
        let mut switches = Vec::new();

        while let TokenKind::TInt(key) = self.see() {
            let key = *key as i32;
            self.advance()?;

            if let TokenKind::TColon = self.see() {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "lookupswitch",
                    component: "label for switch entry",
                })?;
                switches.push(LookupSwitchPair { key, label })
            } else {
                return Err(ParserError::Malformed {
                    component: "lookupswitch",
                    details: "missing : in lookupswitch pair",
                });
            }
        }

        Ok(switches)
    }

    fn parse_default_switch_pair(&mut self) -> ParserResult<String> {
        if let TokenKind::TDefault = self.see() {
            self.advance()?;

            if let TokenKind::TColon = self.see() {
                self.advance()?;

                let label = self.parse_label()?;
                Ok(label)
            } else {
                Err(ParserError::Malformed {
                    component: "default switch pair",
                    details: "misasing : in default switch pair",
                })
            }
        } else {
            Err(ParserError::Missing {
                instr: "parsing default switch pair",
                component: "`default` keyword",
            })
        }
    }

    fn parse_jvm_instruction(&mut self) -> ParserResult<JvmInstruction> {
        Ok(match self.see() {
            // aaload
            TokenKind::TAaload => {
                self.advance()?;
                JvmInstruction::Aaload
            }

            // aastore
            TokenKind::TAastore => {
                self.advance()?;
                JvmInstruction::Aastore
            }

            // aconst_null
            TokenKind::TAconstnull => {
                self.advance()?;
                JvmInstruction::Aconstnull
            }

            // aload <varnum>
            TokenKind::TAload => {
                self.advance()?;

                let varnum = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "aload",
                    component: "varnum",
                })?;
                JvmInstruction::Aload { varnum }
            }

            // aload_0
            TokenKind::TAload0 => {
                self.advance()?;
                JvmInstruction::Aload0
            }

            // aload_1
            TokenKind::TAload1 => {
                self.advance()?;
                JvmInstruction::Aload1
            }

            // aload_2
            TokenKind::TAload2 => {
                self.advance()?;
                JvmInstruction::Aload2
            }

            // aload_3
            TokenKind::TAload3 => {
                self.advance()?;
                JvmInstruction::Aload3
            }

            // anewarray <type>
            // AnewarrayTypeDescriptor <- ClassType / ArrayType
            TokenKind::TAnewarray => {
                self.advance()?;

                let component_type =
                    self.parse_field_descriptor()
                        .map_err(|_| ParserError::Missing {
                            instr: "anewarray",
                            component: "compoenent type",
                        })?;
                JvmInstruction::Anewarray { component_type }
            }

            // areturn
            TokenKind::TAreturn => {
                self.advance()?;
                JvmInstruction::Areturn
            }

            // arraylength
            TokenKind::TArraylength => {
                self.advance()?;
                JvmInstruction::Arraylength
            }

            // astore <varnum>
            TokenKind::TAstore => {
                self.advance()?;

                let varnum = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "astore",
                    component: "varnum",
                })?;
                JvmInstruction::Astore { varnum }
            }

            // astore_0
            TokenKind::TAstore0 => {
                self.advance()?;
                JvmInstruction::Astore0
            }

            // astore_1
            TokenKind::TAstore1 => {
                self.advance()?;
                JvmInstruction::Astore1
            }

            // astore_2
            TokenKind::TAstore2 => {
                self.advance()?;
                JvmInstruction::Astore2
            }

            // astore_3
            TokenKind::TAstore3 => {
                self.advance()?;
                JvmInstruction::Astore3
            }

            // athrow
            TokenKind::TAthrow => {
                self.advance()?;
                JvmInstruction::Athrow
            }

            // baload
            TokenKind::TBaload => {
                self.advance()?;
                JvmInstruction::Baload
            }

            // bastore
            TokenKind::TBastore => {
                self.advance()?;
                JvmInstruction::Bastore
            }

            // bipush i8
            TokenKind::TBipush => {
                self.advance()?;

                let sb = self.parse_sb().map_err(|_| ParserError::Missing {
                    instr: "bipush",
                    component: "numeric signed byte constant",
                })?;

                JvmInstruction::Bipush(sb)
            }

            // caload
            TokenKind::TCaload => {
                self.advance()?;
                JvmInstruction::Caload
            }

            // castore
            TokenKind::TCastore => {
                self.advance()?;
                JvmInstruction::Castore
            }

            // checkcast <type>
            TokenKind::TCheckcast => {
                self.advance()?;

                let cast_type =
                    self.parse_field_descriptor()
                        .map_err(|_| ParserError::Missing {
                            instr: "checkcast",
                            component: "cast type",
                        })?;
                JvmInstruction::Checkcast { cast_type }
            }

            // d2f
            TokenKind::TD2f => {
                self.advance()?;
                JvmInstruction::D2f
            }

            // d2i
            TokenKind::TD2i => {
                self.advance()?;
                JvmInstruction::D2i
            }

            // d2l
            TokenKind::TD2l => {
                self.advance()?;
                JvmInstruction::D2l
            }

            // dadd
            TokenKind::TDadd => {
                self.advance()?;
                JvmInstruction::Dadd
            }

            // daload
            TokenKind::TDaload => {
                self.advance()?;
                JvmInstruction::Daload
            }

            // dastore
            TokenKind::TDastore => {
                self.advance()?;
                JvmInstruction::Dastore
            }

            // dcmpg
            TokenKind::TDcmpg => {
                self.advance()?;
                JvmInstruction::Dcmpg
            }

            // dcmpl
            TokenKind::TDcmpl => {
                self.advance()?;
                JvmInstruction::Dcmpl
            }

            // dconst_0
            TokenKind::TDconst0 => {
                self.advance()?;
                JvmInstruction::Dconst0
            }

            // dconst_1
            TokenKind::TDconst1 => {
                self.advance()?;
                JvmInstruction::Dconst1
            }

            // ddiv
            TokenKind::TDdiv => {
                self.advance()?;
                JvmInstruction::Ddiv
            }

            // dload <arnum>
            TokenKind::TDload => {
                self.advance()?;

                let varnum = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "doad",
                    component: "varnum",
                })?;

                JvmInstruction::Dload { varnum }
            }

            // dload_0
            TokenKind::TDload0 => {
                self.advance()?;
                JvmInstruction::Dload0
            }

            // dload_1
            TokenKind::TDload1 => {
                self.advance()?;
                JvmInstruction::Dload1
            }

            // dload_2
            TokenKind::TDload2 => {
                self.advance()?;
                JvmInstruction::Dload2
            }

            // dload_3
            TokenKind::TDload3 => {
                self.advance()?;
                JvmInstruction::Dload3
            }

            // dmul
            TokenKind::TDmul => {
                self.advance()?;
                JvmInstruction::Dmul
            }

            // dneg
            TokenKind::TDneg => {
                self.advance()?;
                JvmInstruction::Dneg
            }

            // drem
            TokenKind::TDrem => {
                self.advance()?;
                JvmInstruction::Drem
            }

            // dreturn
            TokenKind::TDreturn => {
                self.advance()?;
                JvmInstruction::Dreturn
            }

            // dstore< <varnum>
            TokenKind::TDstore => {
                self.advance()?;

                let varnum = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "dstor",
                    component: "varnum",
                })?;

                JvmInstruction::Dstore { varnum }
            }

            // dstore_0
            TokenKind::TDstore0 => {
                self.advance()?;
                JvmInstruction::Dstore0
            }

            // dstore_1
            TokenKind::TDstore1 => {
                self.advance()?;
                JvmInstruction::Dstore1
            }

            // dstore_2
            TokenKind::TDstore2 => {
                self.advance()?;
                JvmInstruction::Dstore2
            }

            // dstore_3
            TokenKind::TDstore3 => {
                self.advance()?;
                JvmInstruction::Dstore3
            }

            // dsub
            TokenKind::TDsub => {
                self.advance()?;
                JvmInstruction::Dsub
            }

            // dup
            TokenKind::TDup => {
                self.advance()?;
                JvmInstruction::Dup
            }

            // dup2
            TokenKind::TDup2 => {
                self.advance()?;
                JvmInstruction::Dup2
            }

            // dup2_x1
            TokenKind::TDup2x1 => {
                self.advance()?;
                JvmInstruction::Dup2x1
            }

            // dup2_x2
            TokenKind::TDup2x2 => {
                self.advance()?;
                JvmInstruction::Dup2x2
            }

            // dup_x1
            TokenKind::TDupx1 => {
                self.advance()?;
                JvmInstruction::Dupx1
            }

            // dup_x2
            TokenKind::TDupx2 => {
                self.advance()?;
                JvmInstruction::Dupx2
            }

            // f2d
            TokenKind::TF2d => {
                self.advance()?;
                JvmInstruction::F2d
            }

            // f2i
            TokenKind::TF2i => {
                self.advance()?;
                JvmInstruction::F2i
            }

            // f2l
            TokenKind::TF2l => {
                self.advance()?;
                JvmInstruction::F2l
            }

            // fadd
            TokenKind::TFadd => {
                self.advance()?;
                JvmInstruction::Fadd
            }

            // faload
            TokenKind::TFaload => {
                self.advance()?;
                JvmInstruction::Faload
            }

            // fastore
            TokenKind::TFastore => {
                self.advance()?;
                JvmInstruction::Fastore
            }

            // fcmpg
            TokenKind::TFcmpg => {
                self.advance()?;
                JvmInstruction::Fcmpg
            }

            // fcmpl
            TokenKind::TFcmpl => {
                self.advance()?;
                JvmInstruction::Fcmpl
            }

            // fconst_0
            TokenKind::TFconst0 => {
                self.advance()?;
                JvmInstruction::Fconst0
            }

            // fconst_1
            TokenKind::TFconst1 => {
                self.advance()?;
                JvmInstruction::Fconst1
            }

            // fconst_2
            TokenKind::TFconst2 => {
                self.advance()?;
                JvmInstruction::Fconst2
            }

            // fdiv
            TokenKind::TFdiv => {
                self.advance()?;
                JvmInstruction::Fdiv
            }

            // fload <varnum>
            TokenKind::TFload => {
                self.advance()?;

                let varnum = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "fload",
                    component: "varnum",
                })?;
                JvmInstruction::Fload { varnum }
            }

            // fload_0
            TokenKind::TFload0 => {
                self.advance()?;
                JvmInstruction::Fload0
            }

            // fload_1
            TokenKind::TFload1 => {
                self.advance()?;
                JvmInstruction::Fload1
            }

            // fload_2
            TokenKind::TFload2 => {
                self.advance()?;
                JvmInstruction::Fload2
            }

            // fload_3
            TokenKind::TFload3 => {
                self.advance()?;
                JvmInstruction::Fload3
            }

            // fmul
            TokenKind::TFmul => {
                self.advance()?;
                JvmInstruction::Fmul
            }

            // fneg
            TokenKind::TFneg => {
                self.advance()?;
                JvmInstruction::Fneg
            }

            // frem
            TokenKind::TFrem => {
                self.advance()?;
                JvmInstruction::Frem
            }

            // freturn
            TokenKind::TFreturn => {
                self.advance()?;
                JvmInstruction::Freturn
            }

            // fstore <varnum>
            TokenKind::TFstore => {
                self.advance()?;

                let varnum = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "fstore",
                    component: "varnum",
                })?;
                JvmInstruction::Fstore { varnum }
            }

            // fstore_0
            TokenKind::TFstore0 => {
                self.advance()?;
                JvmInstruction::Fstore0
            }

            // fstore_1
            TokenKind::TFstore1 => {
                self.advance()?;
                JvmInstruction::Fstore1
            }

            // fstore_2
            TokenKind::TFstore2 => {
                self.advance()?;
                JvmInstruction::Fstore2
            }

            // fstore_3
            TokenKind::TFstore3 => {
                self.advance()?;
                JvmInstruction::Fstore3
            }

            // fsub
            TokenKind::TFsub => {
                self.advance()?;
                JvmInstruction::Fsub
            }

            // getfield <field-spec> <descriptor>
            TokenKind::TGetfield => {
                self.advance()?;

                if let TokenKind::TIdent(gf_str) = self.see() {
                    if let Some(pos) = gf_str.rfind('/') {
                        let class_name = gf_str[..pos].to_owned();
                        let field_name = gf_str[pos + 1..].to_owned();

                        self.advance()?;

                        let field_descriptor =
                            self.parse_field_descriptor()
                                .map_err(|_| ParserError::Missing {
                                    instr: "getfield",
                                    component: "field descriptor",
                                })?;

                        JvmInstruction::Getfield {
                            class_name,
                            field_name,
                            field_descriptor,
                        }
                    } else {
                        return Err(ParserError::Missing {
                            instr: "getfield",
                            component: "field name",
                        });
                    }
                } else {
                    return Err(ParserError::Missing {
                        instr: "getfield",
                        component: "class name",
                    });
                }
            }

            // getstatic <field-spec> <descriptor>
            TokenKind::TGetstatic => {
                self.advance()?;

                if let TokenKind::TIdent(gs_str) = self.see() {
                    if let Some(pos) = gs_str.rfind('/') {
                        let class_name = gs_str[..pos].to_owned();
                        let field_name = gs_str[pos + 1..].to_owned();

                        self.advance()?;

                        let field_descriptor =
                            self.parse_field_descriptor()
                                .map_err(|_| ParserError::Missing {
                                    instr: "getstatic",
                                    component: "field descriptor",
                                })?;

                        JvmInstruction::Getstatic {
                            class_name,
                            field_name,
                            field_descriptor,
                        }
                    } else {
                        return Err(ParserError::Missing {
                            instr: "getstatic",
                            component: "field name",
                        });
                    }
                } else {
                    return Err(ParserError::Missing {
                        instr: "getstatic",
                        component: "class name",
                    });
                }
            }

            // goto <label>
            TokenKind::TGoto => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "goto",
                    component: "label",
                })?;
                JvmInstruction::Goto { label }
            }

            // goto_w <label>
            TokenKind::TGotow => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "gotow",
                    component: "label",
                })?;
                JvmInstruction::Gotow { label }
            }

            // i2b
            TokenKind::TI2b => {
                self.advance()?;
                JvmInstruction::I2b
            }

            // i2c
            TokenKind::TI2c => {
                self.advance()?;
                JvmInstruction::I2c
            }

            // i2d
            TokenKind::TI2d => {
                self.advance()?;
                JvmInstruction::I2c
            }

            // i2f
            TokenKind::TI2f => {
                self.advance()?;
                JvmInstruction::I2f
            }

            // i2l
            TokenKind::TI2l => {
                self.advance()?;
                JvmInstruction::I2l
            }

            // i2s
            TokenKind::TI2s => {
                self.advance()?;
                JvmInstruction::I2s
            }

            // iadd
            TokenKind::TIadd => {
                self.advance()?;
                JvmInstruction::Iadd
            }

            // iaload
            TokenKind::TIaload => {
                self.advance()?;
                JvmInstruction::Iaload
            }

            // iand
            TokenKind::TIand => {
                self.advance()?;
                JvmInstruction::Iand
            }

            // iastore
            TokenKind::TIastore => {
                self.advance()?;
                JvmInstruction::Iastore
            }

            // iconst_m1
            TokenKind::TIconstm1 => {
                self.advance()?;
                JvmInstruction::Iconstm1
            }

            // iconst_0
            TokenKind::TIconst0 => {
                self.advance()?;
                JvmInstruction::Iconst0
            }

            // iconst_1
            TokenKind::TIconst1 => {
                self.advance()?;
                JvmInstruction::Iconst1
            }

            // iconst_2
            TokenKind::TIconst2 => {
                self.advance()?;
                JvmInstruction::Iconst2
            }

            // iconst-3
            TokenKind::TIconst3 => {
                self.advance()?;
                JvmInstruction::Iconst3
            }

            // iconst_4
            TokenKind::TIconst4 => {
                self.advance()?;
                JvmInstruction::Iconst4
            }

            // iconst_5
            TokenKind::TIconst5 => {
                self.advance()?;
                JvmInstruction::Iconst5
            }

            // idiv
            TokenKind::TIdiv => {
                self.advance()?;
                JvmInstruction::Idiv
            }

            // if_acmpeq <label>
            TokenKind::TIfacmpeq => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ifacmpeq",
                    component: "label",
                })?;

                JvmInstruction::Ifacmpeq { label }
            }

            // if_acmpne <label>
            TokenKind::TIfacmpne => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ifacmpne",
                    component: "label",
                })?;

                JvmInstruction::Ifacmpne { label }
            }

            // if_icmpeq <label>
            TokenKind::TIficmpeq => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ificmpeq",
                    component: "label",
                })?;

                JvmInstruction::Ificmpeq { label }
            }

            // if_icmpge <label>
            TokenKind::TIficmpge => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ificmpge",
                    component: "label",
                })?;

                JvmInstruction::Ificmpge { label }
            }

            // if_icmpgt <label>
            TokenKind::TIficmpgt => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ifcimpgt",
                    component: "label",
                })?;
                JvmInstruction::Ificmpgt { label }
            }

            // if_icmple <label>
            TokenKind::TIficmple => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ificmple",
                    component: "label",
                })?;
                JvmInstruction::Ificmple { label }
            }

            // if_icmplt <label>
            TokenKind::TIficmplt => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ificmplt",
                    component: "label",
                })?;
                JvmInstruction::Ificmplt { label }
            }

            // ifne <label>
            TokenKind::TIfne => {
                self.advance()?;

                if let TokenKind::TIdent(label) = self.see() {
                    let label = label.to_string();
                    self.advance()?;

                    JvmInstruction::Ifne { label }
                } else {
                    return Err(ParserError::Missing {
                        instr: "ifne",
                        component: "label",
                    });
                }
            }

            // if_icmpne <label>
            TokenKind::TIficmpne => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ificmpne",
                    component: "label",
                })?;

                JvmInstruction::Ificmpne { label }
            }

            // ifeq <label>
            TokenKind::TIfeq => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "iceq",
                    component: "label",
                })?;

                JvmInstruction::Ifeq { label }
            }

            // ifge <label>
            TokenKind::TIfge => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ifge",
                    component: "label",
                })?;

                JvmInstruction::Ifge { label }
            }

            // ifgt <label>
            TokenKind::TIfgt => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ifgt",
                    component: "label",
                })?;

                JvmInstruction::Ifgt { label }
            }

            // ifle <label>
            TokenKind::TIfle => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ifle",
                    component: "label",
                })?;

                JvmInstruction::Ifle { label }
            }

            // iflt <label>
            TokenKind::TIflt => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "iflt",
                    component: "label",
                })?;

                JvmInstruction::Iflt { label }
            }

            // ifnonnull <label>
            TokenKind::TIfnonnull => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ifnonnull",
                    component: "label",
                })?;

                JvmInstruction::Ifnonnull { label }
            }

            // ifnull <label>
            TokenKind::TIfnull => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "ifnull",
                    component: "label",
                })?;

                JvmInstruction::Ifnull { label }
            }

            // iinc <varnum> <n>
            TokenKind::TIinc => {
                self.advance()?;

                let varnum = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "iinc",
                    component: "varnum",
                })?;

                let delta = self.parse_sb().map_err(|_| ParserError::Missing {
                    instr: "iinc",
                    component: "delta",
                })?;

                JvmInstruction::Iinc { varnum, delta }
            }

            // iload <varnum>
            TokenKind::TIload => {
                self.advance()?;

                let varnum = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "iload",
                    component: "varnum",
                })?;

                JvmInstruction::Iload { varnum }
            }

            // iload_0
            TokenKind::TIload0 => {
                self.advance()?;
                JvmInstruction::Iload0
            }

            // iload_1
            TokenKind::TIload1 => {
                self.advance()?;
                JvmInstruction::Iload1
            }

            // iload_2
            TokenKind::TIload2 => {
                self.advance()?;
                JvmInstruction::Iload2
            }

            // iload_3
            TokenKind::TIload3 => {
                self.advance()?;
                JvmInstruction::Iload3
            }

            // imul
            TokenKind::TImul => {
                self.advance()?;
                JvmInstruction::Imul
            }

            // ineg
            TokenKind::TIneg => {
                self.advance()?;
                JvmInstruction::Ineg
            }

            // instanceof
            TokenKind::TInstanceof => {
                self.advance()?;

                let check_type =
                    self.parse_field_descriptor()
                        .map_err(|_| ParserError::Missing {
                            instr: "instanceof",
                            component: "check type",
                        })?;
                JvmInstruction::Instanceof { check_type }
            }

            // invokeinterface <method-spec> <n>
            TokenKind::TInvokeinterface => {
                self.advance()?;

                if let TokenKind::TIdent(is_str) = self.see() {
                    if let Some(pos) = is_str.rfind('/') {
                        let interface_name = is_str[..pos].to_owned();
                        let method_name = is_str[pos + 1..].to_owned();

                        self.advance()?;

                        let ub = self.parse_ub().map_err(|_| ParserError::Missing {
                            instr: "invokeinterface",
                            component: "numeric unsigned byte constant",
                        })?;

                        if let Ok(method_descriptor) = self.parse_method_descriptor() {
                            JvmInstruction::Invokeinterface {
                                interface_name,
                                method_name,
                                method_descriptor,
                                ub,
                            }
                        } else {
                            return Err(ParserError::Missing {
                                instr: "invokeinterface",
                                component: "method descriptor",
                            });
                        }
                    } else {
                        return Err(ParserError::Missing {
                            instr: "invokeinterface",
                            component: "method name",
                        });
                    }
                } else {
                    return Err(ParserError::Missing {
                        instr: "invokeinterface",
                        component: "class name",
                    });
                }
            }

            // invokespecial <method-spec>
            TokenKind::TInvokespecial => {
                self.advance()?;

                if let TokenKind::TIdent(is_str) = self.see() {
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
                            return Err(ParserError::Missing {
                                instr: "invokespecial",
                                component: "method descriptor",
                            });
                        }
                    } else {
                        return Err(ParserError::Missing {
                            instr: "invokespecial",
                            component: "method name",
                        });
                    }
                } else {
                    return Err(ParserError::Missing {
                        instr: "invokespecial",
                        component: "class name",
                    });
                }
            }

            // invokestatic <method-spec>
            TokenKind::TInvokestatic => {
                self.advance()?;

                if let TokenKind::TIdent(is_str) = self.see() {
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
                            return Err(ParserError::Missing {
                                instr: "invokestatic",
                                component: "method descriptor",
                            });
                        }
                    } else {
                        return Err(ParserError::Missing {
                            instr: "invokestatic",
                            component: "method name",
                        });
                    }
                } else {
                    return Err(ParserError::Missing {
                        instr: "invokestatic",
                        component: "class name",
                    });
                }
            }

            // invokevirtual <method-spec>
            TokenKind::TInvokevirtual => {
                self.advance()?;

                if let TokenKind::TIdent(is_str) = self.see() {
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
                            return Err(ParserError::Missing {
                                instr: "invokevirtual",
                                component: "method descriptor",
                            });
                        }
                    } else {
                        return Err(ParserError::Missing {
                            instr: "invokevirtual",
                            component: "method name",
                        });
                    }
                } else {
                    return Err(ParserError::Missing {
                        instr: "invokevirtual",
                        component: "class name",
                    });
                }
            }

            // ior
            TokenKind::TIor => {
                self.advance()?;
                JvmInstruction::Ior
            }

            // irem
            TokenKind::TIrem => {
                self.advance()?;
                JvmInstruction::Irem
            }

            // ireturn
            TokenKind::TIreturn => {
                self.advance()?;
                JvmInstruction::Ireturn
            }

            // ishl
            TokenKind::TIshl => {
                self.advance()?;
                JvmInstruction::Ishl
            }

            // lshr
            TokenKind::TIshr => {
                self.advance()?;
                JvmInstruction::Ishr
            }

            // istore <varnum>
            TokenKind::TIstore => {
                self.advance()?;

                let varnum = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "istore",
                    component: "varnum",
                })?;

                JvmInstruction::Istore { varnum }
            }

            // istore_0
            TokenKind::TIstore0 => {
                self.advance()?;
                JvmInstruction::Istore0
            }

            // istore_1
            TokenKind::TIstore1 => {
                self.advance()?;
                JvmInstruction::Istore1
            }

            // istore_2
            TokenKind::TIstore2 => {
                self.advance()?;
                JvmInstruction::Istore2
            }

            // istore_3
            TokenKind::TIstore3 => {
                self.advance()?;
                JvmInstruction::Istore3
            }

            // isub
            TokenKind::TIsub => {
                self.advance()?;
                JvmInstruction::Isub
            }

            // iushr
            TokenKind::TIushr => {
                self.advance()?;
                JvmInstruction::Iushr
            }

            // ixor
            TokenKind::TIxor => {
                self.advance()?;
                JvmInstruction::Ixor
            }

            // jsr <label>
            TokenKind::TJsr => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "jsr",
                    component: "label",
                })?;
                JvmInstruction::Jsr { label }
            }

            // jsr_w <label>
            TokenKind::TJsrw => {
                self.advance()?;

                let label = self.parse_label().map_err(|_| ParserError::Missing {
                    instr: "jsrw",
                    component: "label",
                })?;
                JvmInstruction::Jsrw { label }
            }

            // l2d
            TokenKind::TL2d => {
                self.advance()?;
                JvmInstruction::L2d
            }

            // l2f
            TokenKind::TL2f => {
                self.advance()?;
                JvmInstruction::L2f
            }

            // l2i
            TokenKind::TL2i => {
                self.advance()?;
                JvmInstruction::L2i
            }

            // ladd
            TokenKind::TLadd => {
                self.advance()?;
                JvmInstruction::Ladd
            }

            // laload
            TokenKind::TLaload => {
                self.advance()?;
                JvmInstruction::Laload
            }

            // land
            TokenKind::TLand => {
                self.advance()?;
                JvmInstruction::Land
            }

            // lastore
            TokenKind::TLastore => {
                self.advance()?;
                JvmInstruction::Lastore
            }

            // lcmp
            TokenKind::TLcmp => {
                self.advance()?;
                JvmInstruction::Lcmp
            }

            // locaonst_0
            TokenKind::TLconst0 => {
                self.advance()?;
                JvmInstruction::Lconst0
            }

            // lconst_1
            TokenKind::TLconst1 => {
                self.advance()?;
                JvmInstruction::Lconst1
            }

            // ldc <integer / float / quoted string>
            TokenKind::TLdc => {
                self.advance()?;

                match self.see() {
                    TokenKind::TInt(n) => {
                        let ival = *n as i32;
                        self.advance()?;
                        JvmInstruction::Ldc(LdcValue::Integer(ival))
                    }

                    TokenKind::TFloat(f) => {
                        let fval = *f as f32;
                        self.advance()?;
                        JvmInstruction::Ldc(LdcValue::Float(fval))
                    }

                    TokenKind::TString(s) => {
                        let sval = s.to_owned();
                        self.advance()?;
                        JvmInstruction::Ldc(LdcValue::QuotedString(sval))
                    }

                    _ => {
                        return Err(ParserError::IncorrectTypeOrValue {
                            instr: "lcd",
                            type_or_val: "integer, float, or quoted string constant",
                        })
                    }
                }
            }

            // ldcw <integer / float / quoted string>
            TokenKind::TLdcw => {
                self.advance()?;

                match self.see() {
                    TokenKind::TInt(n) => {
                        let ival = *n as i32;
                        self.advance()?;
                        JvmInstruction::Ldcw(LdcwValue::Integer(ival))
                    }

                    TokenKind::TFloat(f) => {
                        let fval = *f as f32;
                        self.advance()?;
                        JvmInstruction::Ldcw(LdcwValue::Float(fval))
                    }

                    TokenKind::TString(s) => {
                        let sval = s.to_owned();
                        self.advance()?;
                        JvmInstruction::Ldcw(LdcwValue::QuotedString(sval))
                    }

                    _ => {
                        return Err(ParserError::IncorrectTypeOrValue {
                            instr: "ldcw",
                            type_or_val: "integer or float constant",
                        })
                    }
                }
            }

            // ldc2_w <Long / Double>
            TokenKind::TLdc2w => {
                self.advance()?;

                match self.see() {
                    TokenKind::TInt(n) => {
                        let lval = *n as i64;
                        self.advance()?;
                        JvmInstruction::Ldc2w(Ldc2wValue::Long(lval))
                    }

                    TokenKind::TFloat(f) => {
                        let dval = *f as f64;
                        self.advance()?;
                        JvmInstruction::Ldc2w(Ldc2wValue::Double(dval))
                    }

                    _ => {
                        return Err(ParserError::IncorrectTypeOrValue {
                            instr: "lcd2w",
                            type_or_val: "long or double constant",
                        })
                    }
                }
            }

            // ldiv
            TokenKind::TLdiv => {
                self.advance()?;
                JvmInstruction::Ldiv
            }

            // lload <varnum>
            TokenKind::TLload => {
                self.advance()?;

                let varnum = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "iload",
                    component: "varnum",
                })?;

                JvmInstruction::Lload { varnum }
            }

            // lload_0
            TokenKind::TLload0 => {
                self.advance()?;
                JvmInstruction::Lload0
            }

            // lload_1
            TokenKind::TLload1 => {
                self.advance()?;
                JvmInstruction::Lload1
            }

            // lload_2
            TokenKind::TLload2 => {
                self.advance()?;
                JvmInstruction::Lload2
            }

            //lload_3
            TokenKind::TLload3 => {
                self.advance()?;
                JvmInstruction::Lload3
            }

            // lmul
            TokenKind::TLmul => {
                self.advance()?;
                JvmInstruction::Lmul
            }

            // lneg
            TokenKind::TLneg => {
                self.advance()?;
                JvmInstruction::Lneg
            }

            // lookupswitch          <-  'lookupswitch'   LookupSwitchPair*  DefaultSwitchPair
            // LookupSwitchPair      <-  Integer          COLON_symbol       Label
            // DefaultSwitchPair     <-  DEFAULT_keyword  COLON_symbol       Label
            TokenKind::TLookupswitch => {
                self.advance()?;

                let mut switches = self.parse_lookup_switches()?;
                // sort the switches in order of keys
                switches.sort_by_key(|p| p.key);

                let default = self.parse_default_switch_pair()?;

                JvmInstruction::Lookupswitch { switches, default }
            }

            // lor
            TokenKind::TLor => {
                self.advance()?;
                JvmInstruction::Lor
            }

            // lrem
            TokenKind::TLrem => {
                self.advance()?;
                JvmInstruction::Lrem
            }

            // lreturn
            TokenKind::TLreturn => {
                self.advance()?;
                JvmInstruction::Lreturn
            }

            // lshl
            TokenKind::TLshl => {
                self.advance()?;
                JvmInstruction::Lshl
            }

            // lshr
            TokenKind::TLshr => {
                self.advance()?;
                JvmInstruction::Lshr
            }

            // lstore <varnum>
            TokenKind::TLstore => {
                self.advance()?;

                let varnum = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "lstore",
                    component: "varnum",
                })?;

                JvmInstruction::Lstore { varnum }
            }

            // lstore_0
            TokenKind::TLstore0 => {
                self.advance()?;
                JvmInstruction::Lstore0
            }

            // lstore_1
            TokenKind::TLstore1 => {
                self.advance()?;
                JvmInstruction::Lstore1
            }

            // lstore_2
            TokenKind::TLstore2 => {
                self.advance()?;
                JvmInstruction::Lstore2
            }

            // lstore_3
            TokenKind::TLstore3 => {
                self.advance()?;
                JvmInstruction::Lstore3
            }

            // lsub
            TokenKind::TLsub => {
                self.advance()?;
                JvmInstruction::Lsub
            }

            // lushr
            TokenKind::TLushr => {
                self.advance()?;
                JvmInstruction::Lushr
            }

            // lxor
            TokenKind::TLxor => {
                self.advance()?;
                JvmInstruction::Lxor
            }

            // monitorenter
            TokenKind::TMonitorenter => {
                self.advance()?;
                JvmInstruction::Monitorenter
            }

            // monitorexit
            TokenKind::TMonitorexit => {
                self.advance()?;
                JvmInstruction::Monitorexit
            }

            TokenKind::TMultianewarray => {
                self.advance()?;

                let component_type =
                    self.parse_field_descriptor()
                        .map_err(|_| ParserError::Missing {
                            instr: "multianewarray",
                            component: "component type",
                        })?;

                let dimensions = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "multianewarray",
                    component: "dimensions",
                })?;

                JvmInstruction::Multianewarray {
                    component_type,
                    dimensions,
                }
            }

            // new <class>
            TokenKind::TNew => {
                self.advance()?;

                let class_name = self.parse_class_name().map_err(|_| ParserError::Missing {
                    instr: "new",
                    component: "class name",
                })?;

                JvmInstruction::New { class_name }
            }

            // newarray <primitive_type>
            TokenKind::TNewarray => {
                self.advance()?;

                if let TokenKind::TIdent(prim_type) = self.see() {
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
                            return Err(ParserError::IncorrectTypeOrValue {
                                instr: "newarray",
                                type_or_val: "primitive type",
                            })
                        }
                    }
                } else {
                    return Err(ParserError::Malformed {
                        component: "newarray",
                        details: "malformed newarray instruction",
                    });
                }
            }

            // nop
            TokenKind::TNop => {
                self.advance()?;
                JvmInstruction::Nop
            }

            // pop
            TokenKind::TPop => {
                self.advance()?;
                JvmInstruction::Pop
            }

            // pop2
            TokenKind::TPop2 => {
                self.advance()?;
                JvmInstruction::Pop2
            }

            // putfield <field-sepc> <descriptor>
            TokenKind::TPutfield => {
                self.advance()?;

                if let TokenKind::TIdent(gf_str) = self.see() {
                    if let Some(pos) = gf_str.rfind('/') {
                        let class_name = gf_str[..pos].to_owned();
                        let field_name = gf_str[pos + 1..].to_owned();

                        self.advance()?;

                        let field_descriptor =
                            self.parse_field_descriptor()
                                .map_err(|_| ParserError::Missing {
                                    instr: "putfield",
                                    component: "field descriptor",
                                })?;

                        JvmInstruction::Putfield {
                            class_name,
                            field_name,
                            field_descriptor,
                        }
                    } else {
                        return Err(ParserError::Missing {
                            instr: "putfield",
                            component: "field name",
                        });
                    }
                } else {
                    return Err(ParserError::Missing {
                        instr: "putfield",
                        component: "class name",
                    });
                }
            }

            // putstatic <field-spec> <descriptor>
            TokenKind::TPutstatic => {
                self.advance()?;

                if let TokenKind::TIdent(gf_str) = self.see() {
                    if let Some(pos) = gf_str.rfind('/') {
                        let class_name = gf_str[..pos].to_owned();
                        let field_name = gf_str[pos + 1..].to_owned();

                        self.advance()?;

                        let field_descriptor =
                            self.parse_field_descriptor()
                                .map_err(|_| ParserError::Missing {
                                    instr: "putstatic",
                                    component: "field descriptor",
                                })?;

                        JvmInstruction::Putstatic {
                            class_name,
                            field_name,
                            field_descriptor,
                        }
                    } else {
                        return Err(ParserError::Missing {
                            instr: "putstatic",
                            component: "field name",
                        });
                    }
                } else {
                    return Err(ParserError::Missing {
                        instr: "putfield",
                        component: "class name",
                    });
                }
            }

            // ret <varnum>
            TokenKind::TRet => {
                self.advance()?;

                let varnum = self.parse_ub().map_err(|_| ParserError::Missing {
                    instr: "ret",
                    component: "varnum",
                })?;

                JvmInstruction::Ret { varnum }
            }

            // return
            TokenKind::TReturn => {
                self.advance()?;
                JvmInstruction::Return
            }

            // saload
            TokenKind::TSaload => {
                self.advance()?;
                JvmInstruction::Saload
            }

            // sastore
            TokenKind::TSastore => {
                self.advance()?;
                JvmInstruction::Sastore
            }

            // sipush i16
            TokenKind::TSipush => {
                self.advance()?;

                let ss = self.parse_ss().map_err(|_| ParserError::Missing {
                    instr: "sipush",
                    component: "numeric signed short constant",
                })?;

                JvmInstruction::Sipush(ss)
            }

            // swap
            TokenKind::TSwap => {
                self.advance()?;
                JvmInstruction::Swap
            }

            // tableswitch    <-  'tableswitch'   Low   High  TableSwitchSingleton*  DefaultSwitchPair
            // TableSwitchSingleton  <-  Label
            TokenKind::TTableswitch => {
                self.advance()?;

                let low = self.parse_si().map_err(|_| ParserError::Missing {
                    instr: "tableswitch",
                    component: "low",
                })?;

                let high = self.parse_si().map_err(|_| ParserError::Missing {
                    instr: "tableswitch",
                    component: "high",
                })?;

                let switches = self.parse_table_switches()?;
                let default =
                    self.parse_default_switch_pair()
                        .map_err(|_| ParserError::Missing {
                            instr: "tableswitch",
                            component: "default",
                        })?;

                JvmInstruction::Tableswitch {
                    low,
                    high,
                    switches,
                    default,
                }
            }

            // The `wide` instruction must be followed by one of the
            // following instructions:
            // iload, fload, aload, lload, dload, istore, fstore, astore,
            // lstore, dstore, or iinc.
            TokenKind::Twide => {
                self.advance()?;

                match self.see() {
                    TokenKind::TIload => {
                        self.advance()?;

                        let varnum = self.parse_us().map_err(|_| ParserError::Missing {
                            instr: "wide iload",
                            component: "varnum",
                        })?;

                        JvmInstruction::Wide(WideInstruction::Iload { varnum })
                    }

                    TokenKind::TFload => {
                        self.advance()?;

                        let varnum = self.parse_us().map_err(|_| ParserError::Missing {
                            instr: "wide fload",
                            component: "varnum",
                        })?;

                        JvmInstruction::Wide(WideInstruction::Fload { varnum })
                    }

                    TokenKind::TAload => {
                        self.advance()?;

                        let varnum = self.parse_us().map_err(|_| ParserError::Missing {
                            instr: "wide aload",
                            component: "varnum",
                        })?;

                        JvmInstruction::Wide(WideInstruction::Aload { varnum })
                    }

                    TokenKind::TLload => {
                        self.advance()?;

                        let varnum = self.parse_us().map_err(|_| ParserError::Missing {
                            instr: "wide lload",
                            component: "varnum",
                        })?;

                        JvmInstruction::Wide(WideInstruction::Lload { varnum })
                    }

                    TokenKind::TDload => {
                        self.advance()?;

                        let varnum = self.parse_us().map_err(|_| ParserError::Missing {
                            instr: "wide dload",
                            component: "varnum",
                        })?;

                        JvmInstruction::Wide(WideInstruction::Dload { varnum })
                    }

                    TokenKind::TIstore => {
                        self.advance()?;

                        let varnum = self.parse_us().map_err(|_| ParserError::Missing {
                            instr: "wide istore",
                            component: "varnum",
                        })?;

                        JvmInstruction::Wide(WideInstruction::Istore { varnum })
                    }

                    TokenKind::TFstore => {
                        self.advance()?;

                        let varnum = self.parse_us().map_err(|_| ParserError::Missing {
                            instr: "wide fstore",
                            component: "varnum",
                        })?;

                        JvmInstruction::Wide(WideInstruction::Fstore { varnum })
                    }

                    TokenKind::TAstore => {
                        self.advance()?;

                        let varnum = self.parse_us().map_err(|_| ParserError::Missing {
                            instr: "wide astore",
                            component: "varnum",
                        })?;

                        JvmInstruction::Wide(WideInstruction::Astore { varnum })
                    }

                    TokenKind::TLstore => {
                        self.advance()?;

                        let varnum = self.parse_us().map_err(|_| ParserError::Missing {
                            instr: "wide lstore",
                            component: "varnum",
                        })?;

                        JvmInstruction::Wide(WideInstruction::Lstore { varnum })
                    }

                    TokenKind::TDstore => {
                        self.advance()?;

                        let varnum = self.parse_us().map_err(|_| ParserError::Missing {
                            instr: "wide dstore",
                            component: "varnum",
                        })?;

                        JvmInstruction::Wide(WideInstruction::Dstore { varnum })
                    }

                    TokenKind::TIinc => {
                        self.advance()?;

                        let varnum = self.parse_us().map_err(|_| ParserError::Missing {
                            instr: "wide iinc",
                            component: "varnum",
                        })?;

                        let delta = self.parse_ss().map_err(|_| ParserError::Missing {
                            instr: "wide iinc",
                            component: "delta",
                        })?;

                        JvmInstruction::Wide(WideInstruction::IInc { varnum, delta })
                    }

                    _ => {
                        return Err(ParserError::IncorrectTypeOrValue {
                            instr: "wide",
                            type_or_val: "inocrrect instruction following `wide` instruction",
                        })
                    }
                }
            }

            _ => {
                return Err(ParserError::Unknown {
                    component: "jvm instruction",
                })
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

            TokenKind::TIdent(label_str) => {
                let label = label_str.to_string();
                self.advance()?;

                if let TokenKind::TColon = self.see() {
                    self.advance()?;
                    PhoronInstruction::PhoronLabel(label)
                } else {
                    return Err(ParserError::Malformed {
                        component: "label",
                        details: "malformed label",
                    });
                }
            }
            _ => {
                return Err(ParserError::Malformed {
                    component: "Phoron instruction",
                    details: "malformed instruction",
                })
            }
        })
    }

    fn parse_instructions(&mut self) -> ParserResult<Vec<PhoronInstruction>> {
        let mut instructions = Vec::new();

        while self.see() != &TokenKind::TEof {
            if let TokenKind::TEnd = self.see() {
                self.advance()?;
                if let TokenKind::TEndMethod = self.see() {
                    self.advance()?;
                    break;
                } else {
                    return Err(ParserError::Missing {
                        instr: "parsing instructions",
                        component: "end method marker",
                    });
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
        if let TokenKind::TLeftParen = self.see() {
            self.advance()?;

            let ident_tok = self.see();
            let param_descriptor = if let TokenKind::TIdent(ident) = ident_tok {
                let mut param_parser = tdp::TypeParser::new(ident);
                let param_desc =
                    param_parser
                        .parse_param_descriptor()
                        .map_err(|_| ParserError::Missing {
                            instr: "method descriptor",
                            component: "param descriptor",
                        })?;
                self.advance()?;

                param_desc
            } else {
                vec![]
            };

            if let TokenKind::TRightParen = self.see() {
                self.advance()?;

                let ident_tok = self.see();
                if let TokenKind::TIdent(ret) = ident_tok {
                    let mut ret_parser = tdp::TypeParser::new(ret);
                    let return_descriptor =
                        ret_parser
                            .parse_return_descriptor()
                            .map_err(|_| ParserError::Missing {
                                instr: "method descriptor",
                                component: "return descriptor",
                            })?;

                    self.advance()?;

                    Ok(PhoronMethodDescriptor {
                        param_descriptor,
                        return_descriptor,
                    })
                } else {
                    Err(ParserError::Missing {
                        instr: "method descriptor",
                        component: "return descriptor",
                    })
                }
            } else {
                Err(ParserError::Malformed {
                    component: "method descriptor",
                    details: "malformed method descriptor",
                })
            }
        } else {
            Err(ParserError::Malformed {
                component: "method descriptor",
                details: "malformed method descriptor",
            })
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

        if let TokenKind::TIdent(name_str) = self.see() {
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
            Err(ParserError::Missing {
                instr: "parsing method definition",
                component: "method name",
            })
        }
    }

    fn parse_method_defs(&mut self) -> ParserResult<Vec<PhoronMethodDef>> {
        let mut method_defs = Vec::new();

        while let TokenKind::TMethod = self.see() {
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
        if let TokenKind::TIdent(source_file_str) = self.see() {
            let source_file = source_file_str.to_string();
            self.advance()?;

            Ok(PhoronSourceFileDef { source_file })
        } else {
            Err(ParserError::Missing {
                instr: "parsing source file definition",
                component: "source file name",
            })
        }
    }

    /// Header <- SourceFileDef? (ClassDef / InterfaceDef) SuperDef
    fn parse_header(&mut self) -> ParserResult<PhoronHeader> {
        self.advance()?;

        Ok(match self.see() {
            TokenKind::TSource => {
                self.advance()?;

                let sourcefile_def = self.parse_sourcefile_def()?;

                let class_or_interface_def = match self.see() {
                    TokenKind::TClass => PhoronClassOrInterface::Class(self.parse_class_def()?),
                    TokenKind::TInterface => {
                        PhoronClassOrInterface::Interface(self.parse_interface_def()?)
                    }
                    _ => {
                        return Err(ParserError::IncorrectTypeOrValue {
                            instr: "parsing header",
                            type_or_val: "class or interface definition token",
                        })
                    }
                };

                let super_def = self.parse_super_def()?;
                let implements_defs = self.parse_implements_defs()?;

                PhoronHeader {
                    sourcefile_def,
                    class_or_interface_def,
                    super_def,
                    implements_defs,
                }
            }

            TokenKind::TClass => {
                let sourcefile_def = PhoronSourceFileDef {
                    source_file: self.lexer.src_file()?.to_string(),
                };

                let class_or_interface_def = PhoronClassOrInterface::Class(self.parse_class_def()?);
                let super_def = self.parse_super_def()?;
                let implements_defs = self.parse_implements_defs()?;

                PhoronHeader {
                    sourcefile_def,
                    class_or_interface_def,
                    super_def,
                    implements_defs,
                }
            }

            TokenKind::TInterface => {
                let sourcefile_def = PhoronSourceFileDef {
                    source_file: self.lexer.src_file()?.to_string(),
                };

                let class_or_interface_def =
                    PhoronClassOrInterface::Interface(self.parse_interface_def()?);
                let super_def = self.parse_super_def()?;
                let implements_defs = self.parse_implements_defs()?;

                PhoronHeader {
                    sourcefile_def,
                    class_or_interface_def,
                    super_def,
                    implements_defs,
                }
            }

            _ => {
                return Err(ParserError::IncorrectTypeOrValue {
                    instr: "parsing header",
                    type_or_val: "token",
                })
            }
        })
    }

    /// PhoronProgram <- line_comment* Header Body eof
    pub fn parse(&mut self) -> ParserResult<PhoronProgram> {
        let header = self.parse_header()?;
        let body = self.parse_body()?;

        Ok(PhoronProgram { header, body })
    }
}
