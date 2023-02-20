use std::{fmt, iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub enum Token {
    TSynthetic,
    TAaload,
    TAastore,
    TAbstract,
    TAconstnull,
    TAload,
    TAload0,
    TAload1,
    TAload2,
    TAload3,
    TAnewarray,
    TAnnotation,
    TAreturn,
    TArraylength,
    TAssign,
    TAstore,
    TAstore0,
    TAstore1,
    TAstore2,
    TAstore3,
    TAthrow,
    TBaload,
    TBastore,
    TBipush,
    TCaload,
    TCastore,
    TCatch,
    TCheckcast,
    TClass,
    TColon,
    TD2f,
    TD2i,
    TD2l,
    TDadd,
    TDaload,
    TDastore,
    TDcmpg,
    TDcmpl,
    TDconst0,
    TDconst1,
    TDdiv,
    TDefault,
    TDload,
    TDload0,
    TDload1,
    TDload2,
    TDload3,
    TDmul,
    TDneg,
    TDot,
    TDrem,
    TDreturn,
    TDstore,
    TDstore0,
    TDstore1,
    TDstore2,
    TDstore3,
    TDsub,
    TDup,
    TDup2,
    TDup2x1,
    TDup2x2,
    TDupx1,
    TDupx2,
    TEnd,
    TEndMethod,
    TEnum,
    TEof,
    TF2d,
    TF2i,
    TF2l,
    TFadd,
    TFaload,
    TFastore,
    TFcmpg,
    TFcmpl,
    TFconst0,
    TFconst1,
    TFconst2,
    TFdiv,
    TField,
    TFinal,
    TFload,
    TFload0,
    TFload1,
    TFload2,
    TFload3,
    TFloat(f64),
    TFmul,
    TFneg,
    TFrem,
    TFreturn,
    TFrom,
    TFstore,
    TFstore0,
    TFstore1,
    TFstore2,
    TFstore3,
    TFsub,
    TGetfield,
    TGetstatic,
    TGoto,
    TGotow,
    TI2b,
    TI2c,
    TI2d,
    TI2f,
    TI2l,
    TI2s,
    TIadd,
    TIaload,
    TIand,
    TIastore,
    TIconst0,
    TIconst1,
    TIconst2,
    TIconst3,
    TIconst4,
    TIconst5,
    TIconstm1,
    TIdent(String),
    TIdiv,
    TIfacmpeq,
    TIfacmpne,
    TIfeq,
    TIfge,
    TIfgt,
    TIficmpeq,
    TIficmpge,
    TIficmpgt,
    TIficmple,
    TIficmplt,
    TIficmpne,
    TIfle,
    TIflt,
    TIfne,
    TIfnonnull,
    TIfnull,
    TIinc,
    TIload,
    TIload0,
    TIload1,
    TIload2,
    TIload3,
    TImul,
    TIneg,
    TInstanceof,
    TInt(i64),
    TInterface,
    TInvokeinterface,
    TInvokespecial,
    TInvokestatic,
    TInvokevirtual,
    TIor,
    TIrem,
    TIreturn,
    TIs,
    TIshl,
    TIshr,
    TIstore,
    TIstore0,
    TIstore1,
    TIstore2,
    TIstore3,
    TIsub,
    TIushr,
    TIxor,
    TJsr,
    TJsrw,
    TL2d,
    TL2f,
    TL2i,
    TLadd,
    TLand,
    TLastore,
    TLcmp,
    TLconst0,
    TLconst1,
    TLdc,
    TLdc2w,
    TLdcw,
    TLdiv,
    TLeftParen,
    TLeftSquareBracket,
    TLimit,
    TLine,
    TLload,
    TLload0,
    TLload1,
    TLload2,
    TLload3,
    TLmul,
    TLneg,
    TLoaload,
    TLocals,
    TLookupswitch,
    TLor,
    TLrem,
    TLreturn,
    TLshl,
    TLshr,
    TLstore,
    TLstore0,
    TLstore1,
    TLstore2,
    TLstore3,
    TLsub,
    TLushr,
    TLxor,
    TMethod,
    TMinus,
    TModule,
    TMonitorenter,
    TMonitorexit,
    TMultianewarray,
    TNative,
    TNew,
    TNewarray,
    TNop,
    TPlus,
    TPop,
    TPop2,
    TPrivate,
    TProtected,
    TPublic,
    TPutfield,
    TPutstatic,
    TRet,
    TReturn,
    TRightParen,
    TSaload,
    TSastore,
    TSipush,
    TSource,
    TStack,
    TStatic,
    TString(String),
    TSuper,
    TSwap,
    TSynchronized,
    TTableswitch,
    TThrows,
    TTo,
    TTransient,
    TUsing,
    TVar,
    TVolatile,
}

pub type LexerResult<T> = Result<T, LexerError>;

#[derive(Debug)]
pub enum LexerError {
    Custom(String),
    IncompleteString,
    InvalidCharacter(char),
    InvalidDirective(String),
    OutOfCharacters,
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
}

impl std::error::Error for LexerError {}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                LexerError::Custom(ref msg) => msg.into(),
                LexerError::IncompleteString => "invalid or malformed string".into(),
                LexerError::InvalidCharacter(ref c) =>
                    format!("Invalid character while lexing: {c}"),
                LexerError::InvalidDirective(ref directive) =>
                    format!("Invalid directive {directive} for Phoron"),
                LexerError::ParseIntError(ref err) =>
                    format!("error while trying to lex an integer: {err}"),
                LexerError::ParseFloatError(ref err) =>
                    format!("error while trying to lex a float: {err}"),
                LexerError::OutOfCharacters => "unexpected end of input stream while lexing".into(),
            }
        )
    }
}

impl From<std::num::ParseIntError> for LexerError {
    fn from(pie: std::num::ParseIntError) -> Self {
        LexerError::ParseIntError(pie)
    }
}

impl From<std::num::ParseFloatError> for LexerError {
    fn from(pfe: std::num::ParseFloatError) -> Self {
        LexerError::ParseFloatError(pfe)
    }
}

enum Number {
    Float(f64),
    Int(i64),
}

pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            src: src.chars().peekable(),
        }
    }

    fn next(&mut self) -> LexerResult<char> {
        Ok(self.src.next().ok_or(LexerError::OutOfCharacters)?)
    }

    fn extract_float_or_int(&mut self) -> LexerResult<Number> {
        let mut numbuf = String::new();
        loop {
            if let Some(d) = self.src.peek() {
                if d.is_digit(10) {
                    numbuf.push(self.next()?);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if let Some('.') = self.src.peek() {
            numbuf.push(self.next()?);

            loop {
                if let Some(d) = self.src.peek() {
                    if d.is_digit(10) {
                        numbuf.push(self.next()?);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            Ok(Number::Float(numbuf.parse::<f64>()?))
        } else {
            Ok(Number::Int(numbuf.parse::<i64>()?))
        }
    }

    fn extract_ident(&mut self) -> LexerResult<String> {
        let is_ident_char = |c| match c {
            '/' | '.' | '<' | '>' | '_' | ';' => true,
            c if c.is_alphabetic() => true,
            c if c.is_digit(10) => true,
            _ => false,
        };

        let mut identbuf = String::new();
        loop {
            if let Some(c) = self.src.peek() {
                if is_ident_char(*c) {
                    identbuf.push(self.next()?);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(identbuf)
    }

    fn extract_directive(&mut self, ident: &str) -> LexerResult<Token> {
        use Token::*;

        Ok(match ident {
            "catch" => TCatch,
            "class" => TClass,
            "end" => TEnd,
            "field" => TField,
            "interface" => TInterface,
            "limit" => TLimit,
            "line" => TLine,
            "method" => TMethod,
            "source" => TSource,
            "super" => TSuper,
            "throws" => TThrows,
            "var" => TVar,
            _ => return Err(LexerError::InvalidDirective(ident.to_owned())),
        })
    }

    fn extract_kw_or_instr(&mut self, ident: &str) -> Option<Token> {
        use Token::*;

        Some(match ident {
            "aaload" => TAaload,
            "aastor" => TAastore,
            "abstract" => TAbstract,
            "aconst_null" => TAconstnull,
            "aload" => TAload,
            "aload_0" => TAload0,
            "aload_1" => TAload1,
            "aload_2" => TAload2,
            "aload_3" => TAload3,
            "anewarray" => TAnewarray,
            "areturn" => TAreturn,
            "arraylength" => TArraylength,
            "astore" => TAstore,
            "astore_0" => TAstore0,
            "astore_1" => TAstore1,
            "astore_2" => TAstore2,
            "astore_3" => TAstore3,
            "athrow" => TAthrow,
            "baload" => TBaload,
            "bastore" => TBastore,
            "bipush" => TBipush,
            "caload" => TCaload,
            "castore" => TCastore,
            "checkcast" => TCheckcast,
            "d2f" => TD2f,
            "d2i" => TD2i,
            "d2l" => TD2l,
            "dadd" => TDadd,
            "daload" => TDaload,
            "dastore" => TDastore,
            "dcmpg" => TDcmpg,
            "dcmpl" => TDcmpl,
            "dconst_0" => TDconst0,
            "dconst_1" => TDconst1,
            "ddiv" => TDdiv,
            "default" => TDefault,
            "dload" => TDload,
            "dload_0" => TDload0,
            "dload_1" => TDload1,
            "dload_2" => TDload2,
            "dload_3" => TDload3,
            "dmul" => TDmul,
            "dneg" => TDneg,
            "drem" => TDrem,
            "dreturn" => TDreturn,
            "dstore" => TDstore,
            "dstore_0" => TDstore0,
            "dstore_1" => TDstore1,
            "dstore_2" => TDstore2,
            "dstore_3" => TDstore3,
            "dsub" => TDsub,
            "dup" => TDup,
            "dup2" => TDup2,
            "dup2_x1" => TDup2x1,
            "dup2_x2" => TDup2x2,
            "dup_x1" => TDupx1,
            "dup_x2" => TDupx2,
            "f2d" => TF2d,
            "f2i" => TF2i,
            "f2l" => TF2l,
            "fadd" => TFadd,
            "faload" => TFaload,
            "fastore" => TFastore,
            "fcmpg" => TFcmpg,
            "fcmpl" => TFcmpl,
            "fconst_0" => TFconst0,
            "fconst_1" => TFconst1,
            "fconst_2" => TFconst2,
            "fdiv" => TFdiv,
            "final" => TFinal,
            "fload" => TFload,
            "fload_0" => TFload0,
            "fload_1" => TFload1,
            "fload_2" => TFload2,
            "fload_3" => TFload3,
            "fmul" => TFmul,
            "fneg" => TFneg,
            "frem" => TFrem,
            "freturn" => TFreturn,
            "from" => TFrom,
            "fstore" => TFstore,
            "fstore_0" => TFstore0,
            "fstore_1" => TFstore1,
            "fstore_2" => TFstore2,
            "fstore_3" => TFstore3,
            "fsub" => TFsub,
            "getfield" => TGetfield,
            "getstatic" => TGetstatic,
            "goto" => TGoto,
            "goto_w" => TGotow,
            "i2b" => TI2b,
            "i2c" => TI2c,
            "i2d" => TI2d,
            "i2f" => TI2f,
            "i2l" => TI2l,
            "i2s" => TI2s,
            "iadd" => TIadd,
            "iaload" => TIaload,
            "iand" => TIand,
            "iastore" => TIastore,
            "iconst_0" => TIconst0,
            "iconst_1" => TIconst1,
            "iconst_2" => TIconst2,
            "iconst_3" => TIconst3,
            "iconst_4" => TIconst4,
            "iconst_5" => TIconst5,
            "iconst_m1" => TIconstm1,
            "idiv" => TIdiv,
            "if_acmpeq" => TIfacmpeq,
            "if_acmpne" => TIfacmpne,
            "if_icmpeq" => TIficmpeq,
            "if_icmpge" => TIficmpge,
            "if_icmpgt" => TIficmpgt,
            "if_icmple" => TIficmple,
            "if_icmplt" => TIficmplt,
            "if_icmpne" => TIficmpne,
            "ifeq" => TIfeq,
            "ifge" => TIfge,
            "ifgt" => TIfgt,
            "ifle" => TIfle,
            "iflt" => TIflt,
            "ifne" => TIfne,
            "ifnonnull" => TIfnonnull,
            "ifnull" => TIfnull,
            "iinc" => TIinc,
            "iload" => TIload,
            "iload_0" => TIload0,
            "iload_1" => TIload1,
            "iload_2" => TIload2,
            "iload_3" => TIload3,
            "imul" => TImul,
            "ineg" => TIneg,
            "instanceof" => TInstanceof,
            "invokeinterface" => TInvokeinterface,
            "invokespecial" => TInvokespecial,
            "invokestatic" => TInvokestatic,
            "invokevirtual" => TInvokevirtual,
            "ior" => TIor,
            "irem" => TIrem,
            "ireturn" => TIreturn,
            "is" => TIs,
            "ishl" => TIshl,
            "ishr" => TIshr,
            "istore" => TIstore,
            "istore_0" => TIstore0,
            "istore_1" => TIstore1,
            "istore_2" => TIstore2,
            "istore_3" => TIstore3,
            "isub" => TIsub,
            "iushr" => TIushr,
            "ixor" => TIxor,
            "jsr" => TJsr,
            "jsr_w" => TJsrw,
            "l2d" => TL2d,
            "l2f" => TL2f,
            "l2i" => TL2i,
            "ladd" => TLadd,
            "land" => TLand,
            "lastore" => TLastore,
            "lcmp" => TLcmp,
            "lconst_0" => TLconst0,
            "lconst_1" => TLconst1,
            "ldc" => TLdc,
            "ldc2_w" => TLdc2w,
            "ldc_w" => TLdcw,
            "ldiv" => TLdiv,
            "lload" => TLload,
            "lload_0" => TLload0,
            "lload_1" => TLload1,
            "lload_2" => TLload2,
            "lload_3" => TLload3,
            "lmul" => TLmul,
            "lneg" => TLneg,
            "loaload" => TLoaload,
            "locals" => TLocals,
            "lookupswitch" => TLookupswitch,
            "lor" => TLor,
            "lrem" => TLrem,
            "lreturn" => TLreturn,
            "lshl" => TLshl,
            "lshr" => TLshr,
            "lstore" => TLstore,
            "lstore_0" => TLstore0,
            "lstore_1" => TLstore1,
            "lstore_2" => TLstore2,
            "lstore_3" => TLstore3,
            "lsub" => TLsub,
            "lushr" => TLushr,
            "lxor" => TLxor,
            "method" => TEndMethod,
            "monitorenter" => TMonitorenter,
            "monitorexit" => TMonitorexit,
            "multianewarray" => TMultianewarray,
            "native" => TNative,
            "new" => TNew,
            "newarray" => TNewarray,
            "nop" => TNop,
            "pop" => TPop,
            "pop2" => TPop2,
            "private" => TPrivate,
            "protected" => TProtected,
            "public" => TPublic,
            "putfield" => TPutfield,
            "putstatic" => TPutstatic,
            "ret" => TRet,
            "return" => TReturn,
            "saload" => TSaload,
            "sastore" => TSastore,
            "sipush" => TSipush,
            "stack" => TStack,
            "static" => TStatic,
            "swap" => TSwap,
            "synchronized" => TSynchronized,
            "tableswitch" => TTableswitch,
            "to" => TTo,
            "transient" => TTransient,
            "using" => TUsing,
            "volatile" => TVolatile,
            _ => return None,
        })
    }

    fn lex_char(&mut self, c: char) -> LexerResult<Token> {
        use Token::*;

        Ok(match c {
            ' ' | '\t' | '\n' => {
                self.next()?;
                self.lex()?
            }

            ';' => {
                loop {
                    if let Some(c) = self.src.peek() {
                        if *c != '\n' {
                            self.next()?;
                        } else {
                            break;
                        }
                    }
                }

                self.lex()?
            }

            '.' => {
                self.next()?;

                if let Some(c) = self.src.peek() {
                    if c.is_alphabetic() {
                        let ident = self.extract_ident()?;
                        self.extract_directive(&ident)?
                    } else {
                        TDot
                    }
                } else {
                    TDot
                }
            }

            ':' => {
                self.next()?;
                TColon
            }

            '(' => {
                self.next()?;
                TLeftParen
            }

            ')' => {
                self.next()?;
                TRightParen
            }
            '[' => {
                self.next()?;
                TLeftSquareBracket
            }

            '=' => {
                self.next()?;
                TAssign
            }

            '+' => {
                self.next()?;
                TPlus
            }

            '-' => {
                self.next()?;
                TMinus
            }

            '"' => {
                self.next()?;

                let mut strbuf = String::new();
                loop {
                    if let Some(c) = self.src.peek() {
                        if *c != '"' {
                            strbuf.push(self.next()?);
                        } else {
                            break;
                        }
                    }
                }

                if self.src.peek().is_none() {
                    return Err(LexerError::IncompleteString);
                }

                self.next()?; // consume the closing double quote
                TString(strbuf)
            }

            '0'..='9' => {
                let number = self.extract_float_or_int()?;

                match number {
                    Number::Float(float) => TFloat(float),
                    Number::Int(int) => TInt(int),
                }
            }

            c if c.is_alphabetic() || c == '_' || c == '<' => {
                let ident = self.extract_ident()?;

                if let Some(kw_or_instr) = self.extract_kw_or_instr(&ident) {
                    kw_or_instr
                } else {
                    TIdent(ident)
                }
            }

            _ => return Err(LexerError::InvalidCharacter(c)),
        })
    }

    pub fn lex(&mut self) -> LexerResult<Token> {
        if self.src.peek().is_none() {
            Ok(Token::TEof)
        } else {
            let c = *self.src.peek().unwrap();
            self.lex_char(c)
        }
    }
}
