use std::{
    error::Error,
    fmt,
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::{
    diagnostics::DiagnosticManager,
    sourcefile::{Pos, SourceFile, Span},
};

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
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
    TBridge,
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
    TImplements,
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
    TLaload,
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
    TModule,
    TMonitorenter,
    TMonitorexit,
    TMultianewarray,
    TNative,
    TNew,
    TNewarray,
    TNop,
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
    TStrict,
    TString(String),
    TSuper,
    TSwap,
    TSynchronized,
    TSynthetic,
    TTableswitch,
    TThrows,
    TTo,
    TTransient,
    TUsing,
    TVar,
    TVarargs,
    TVolatile,
    Twide,
}

#[derive(Debug)]
pub struct LexerError {
    pub span: Span,
    pub message: String,
}

impl Error for LexerError {}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.message, self.span)
    }
}

pub type LexerResult<T> = Result<T, LexerError>;

#[derive(Debug)]
enum Number {
    Float(f64),
    Int(i64),
}

/// The Phoron Lexer
pub struct Lexer<'a> {
    pub source_file: &'a SourceFile,
    src: Peekable<Enumerate<Chars<'a>>>,
    errored: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source_file: &'a SourceFile) -> Self {
        Lexer {
            source_file,
            src: source_file.src.chars().enumerate().peekable(),
            errored: false,
        }
    }

    pub fn errored(&self) -> bool {
        self.errored
    }

    fn curr_pos(&mut self) -> Pos {
        self.src.peek().map_or(Pos::default(), |p| p.0.into())
    }

    fn extract_float_or_int(&mut self) -> Option<Number> {
        let mut numbuf = String::new();
        loop {
            if let Some((_idx, d)) = self.src.peek() {
                if d.is_digit(10) {
                    numbuf.push(self.src.next()?.1);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let mut saw_e = false;
        let mut saw_sign = false;

        if let Some((_idx, '.')) = self.src.peek() {
            numbuf.push(self.src.next()?.1);

            loop {
                if let Some((_idx, d)) = self.src.peek() {
                    if d.is_digit(10) {
                        numbuf.push(self.src.next()?.1);

                        // check for exponent part
                        match self.src.peek() {
                            Some((_idx, 'e')) | Some((_idx, 'E')) => {
                                if saw_e {
                                    break;
                                }

                                numbuf.push(self.src.next()?.1);
                                saw_e = true;
                            }
                            _ => {}
                        }

                        // check for `-` or `+` sign
                        match self.src.peek() {
                            Some((_idx, '+')) | Some((_idx, '-')) => {
                                if saw_sign {
                                    break;
                                }

                                numbuf.push(self.src.next()?.1);
                                saw_sign = true;
                            }
                            _ => {}
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            numbuf.parse::<f64>().ok().map(|f| Number::Float(f))
        } else {
            numbuf.parse::<i64>().ok().map(|i| Number::Int(i))
        }
    }

    fn extract_ident(&mut self) -> String {
        let is_ident_char = |c| match c {
            '/' | '.' | '<' | '>' | '_' | '[' | ';' => true,
            c if c.is_alphabetic() => true,
            c if c.is_digit(10) => true,
            _ => false,
        };

        let mut identbuf = String::new();
        loop {
            if let Some((_idx, c)) = self.src.peek() {
                if is_ident_char(*c) {
                    identbuf.push(self.src.next().unwrap().1);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        identbuf
    }

    fn extract_directive(&mut self, ident: &str) -> Option<TokenKind> {
        use TokenKind::*;

        Some(match ident {
            "catch" => TCatch,
            "class" => TClass,
            "end" => TEnd,
            "field" => TField,
            "implements" => TImplements,
            "interface" => TInterface,
            "limit" => TLimit,
            "line" => TLine,
            "method" => TMethod,
            "source" => TSource,
            "super" => TSuper,
            "throws" => TThrows,
            "var" => TVar,
            _ => return None,
        })
    }

    fn extract_kw_or_instr(&mut self, ident: &str) -> Option<TokenKind> {
        use TokenKind::*;

        Some(match ident {
            "aaload" => TAaload,
            "aastore" => TAastore,
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
            "invokenonvirtual" | "invokespecial" => TInvokespecial,
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
            "laload" => TLaload,
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
            "wide" => Twide,
            _ => return None,
        })
    }

    fn lex_char(&mut self, c: char) -> LexerResult<TokenKind> {
        use TokenKind::*;

        Ok(match c {
            c if c.is_whitespace() => {
                self.src.next();

                match self.lex() {
                    None => {
                        let (low, high) = (self.curr_pos(), self.curr_pos());

                        return Err(LexerError {
                            span: Span { low, high },
                            message: format!("Missing token"),
                        });
                    }

                    Some(tok) => tok.kind,
                }
            }

            ';' => {
                loop {
                    if let Some((_idx, c)) = self.src.peek() {
                        if *c != '\n' {
                            self.src.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                match self.lex() {
                    None => {
                        let (low, high) = (self.curr_pos(), self.curr_pos());

                        return Err(LexerError {
                            span: Span { low, high },
                            message: format!("missing token"),
                        });
                    }

                    Some(tok) => tok.kind,
                }
            }

            '.' => {
                self.src.next();

                let low = self.curr_pos();

                if let Some((_idx, c)) = self.src.peek() {
                    if c.is_alphabetic() {
                        let ident = self.extract_ident();
                        let high = self.curr_pos();

                        match self.extract_directive(&ident) {
                            None => {
                                return Err(LexerError {
                                    span: Span { low, high },
                                    message: format!("invalid directive `{ident}`"),
                                })
                            }

                            Some(dir) => dir,
                        }
                    } else {
                        TDot
                    }
                } else {
                    TDot
                }
            }

            ':' => {
                self.src.next();
                TColon
            }

            '(' => {
                self.src.next();
                TLeftParen
            }

            ')' => {
                self.src.next();
                TRightParen
            }

            '=' => {
                self.src.next();
                TAssign
            }

            c if c == '+' || c == '-' => {
                self.src.next();

                let (low, high) = (self.curr_pos(), self.curr_pos());

                match self.extract_float_or_int() {
                    None => {
                        return Err(LexerError {
                            span: Span { low, high },
                            message: format!("missing integer or float"),
                        })
                    }

                    Some(number) => match number {
                        Number::Float(float) => {
                            if c == '+' {
                                TFloat(float)
                            } else {
                                TFloat(-float)
                            }
                        }
                        Number::Int(int) => {
                            if c == '+' {
                                TInt(int)
                            } else {
                                TInt(-int)
                            }
                        }
                    },
                }
            }

            '"' => {
                self.src.next();

                let low = self.curr_pos();

                let mut strbuf = String::new();
                loop {
                    if let Some((_idx, c)) = self.src.peek() {
                        if *c != '"' {
                            if *c == '\\' {
                                self.src.next();

                                match self.src.peek().unwrap().1 {
                                    'n' => {
                                        strbuf.push('\n');
                                        self.src.next();
                                    }

                                    'r' => {
                                        strbuf.push('\r');
                                        self.src.next();
                                    }

                                    't' => {
                                        strbuf.push('\t');
                                        self.src.next();
                                    }

                                    'f' => {
                                        strbuf.push_str(r"\f");
                                        self.src.next();
                                    }

                                    'b' => {
                                        strbuf.push_str(r"\b");
                                        self.src.next();
                                    }

                                    '\'' => {
                                        strbuf.push('\'');
                                        self.src.next();
                                    }

                                    '"' => {
                                        strbuf.push('\"');
                                        self.src.next();
                                    }

                                    '\\' => {
                                        strbuf.push('\\');
                                        self.src.next();
                                    }

                                    d => {
                                        let high = self.curr_pos();

                                        return Err(LexerError {
                                            span: Span { low, high },
                                            message: format!(
                                                "invalid escape sequence character: `{d}`"
                                            ),
                                        });
                                    }
                                }
                            } else {
                                strbuf.push(self.src.next().unwrap().1);
                            }
                        } else {
                            break;
                        }
                    }
                }

                if self.src.peek().is_none() {
                    let high = self.curr_pos();
                    return Err(LexerError {
                        span: Span { low, high },
                        message: format!("unterminated string"),
                    });
                }

                self.src.next(); // consume the closing double quote
                TString(strbuf)
            }

            '0'..='9' => {
                let number = self.extract_float_or_int().unwrap();

                match number {
                    Number::Float(float) => TFloat(float),
                    Number::Int(int) => TInt(int),
                }
            }

            c if c.is_alphabetic() || c == '_' || c == '<' || c == '[' => {
                let ident = self.extract_ident();

                if let Some(kw_or_instr) = self.extract_kw_or_instr(&ident) {
                    kw_or_instr
                } else {
                    TIdent(ident)
                }
            }
            _ => {
                let low = self.curr_pos();
                let high = self.curr_pos();

                return Err(LexerError {
                    span: Span { low, high },
                    message: format!("invalid character {c:?}"),
                });
            }
        })
    }

    pub fn src_file(&self) -> &str {
        &self.source_file.src_file
    }

    pub fn lex(&mut self) -> Option<Token> {
        if self.src.peek().is_none() {
            Some(Token {
                kind: TokenKind::TEof,
                span: Span {
                    low: Pos::default(),
                    high: Pos::default(),
                },
            })
        } else {
            let (low, c) = self.src.peek().unwrap();
            let low = if *low == 0 {
                Pos::default()
            } else {
                Pos::new(*low + 1)
            };

            let c = *c;
            match self.lex_char(c) {
                Err(err) => {
                    DiagnosticManager::report_diagnostic(&self.source_file, err.span, err.message);
                    self.errored |= true;

                    self.src.next();
                    return self.lex();
                }

                Ok(tok_kind) => {
                    let mut high = self.curr_pos();

                    if high < low {
                        high = low;
                    }

                    Some(Token {
                        kind: tok_kind,
                        span: Span { low, high },
                    })
                }
            }
        }
    }
}
