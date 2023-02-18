pub type Int = i64;
pub type Float = f64;

#[derive(Debug, PartialEq)]
pub enum Token {
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
    TFloat(Float),
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
    TInt(Int),
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
    TSemicolon,
    TSipush,
    TSource,
    TStack,
    TStatic,
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
    Tstring(String),
}

pub type LexerResult<T> = Result<T, LexerError>;

#[derive(Debug)]
pub enum LexerError {
    Custom(String),
    InvalidCharacter(char),
    InvalidDirective(String),
    ParseIntError(std::num::ParseIntError),
    OutOfCharacters,
}

impl std::error::Error for LexerError {}

use std::fmt;
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                LexerError::Custom(ref msg) => msg.into(),
                LexerError::InvalidCharacter(ref c) =>
                    format!("Invalid character while lexing: {c}"),
                LexerError::InvalidDirective(ref directive) =>
                    format!("Invalid directive {directive} for Phoron"),
                LexerError::ParseIntError(ref err) =>
                    format!("error while trying to lex an integer: {err}"),
                LexerError::OutOfCharacters => "unexpected end of input stream while lexing".into(),
            }
        )
    }
}

pub struct Lexer {
    src: String,
    curr_pos: usize,
    size: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        let size = src.len();

        Lexer {
            src: src.to_owned(),
            curr_pos: 0,
            size,
        }
    }

    fn curr_char(&self) -> LexerResult<char> {
        Ok(self
            .src
            .chars()
            .nth(self.curr_pos)
            .ok_or(LexerError::OutOfCharacters)?)
    }

    fn forward(&mut self) {
        self.curr_pos += 1;
    }

    fn extract_pred<P>(&mut self, pred: P) -> LexerResult<&str>
    where
        P: Fn(char) -> bool,
    {
        let start_pos = self.curr_pos;
        let mut running_pos = self.curr_pos;

        while running_pos < self.size
            && pred(
                self.src
                    .chars()
                    .nth(running_pos)
                    .ok_or(LexerError::OutOfCharacters)?,
            )
        {
            running_pos += 1;
        }

        self.curr_pos = running_pos;
        Ok(&self.src[start_pos..running_pos])
    }

    fn extract_int(&mut self) -> LexerResult<Int> {
        Ok(self
            .extract_pred(|c| c.is_digit(10))?
            .parse::<Int>()
            .or_else(|parse_err| Err(LexerError::ParseIntError(parse_err)))?)
    }

    fn extract_float(&mut self) -> LexerResult<Float> {
        let before_dec = self.extract_int()? as Float;
        if self.curr_char()? != '.' {
            return Err(LexerError::Custom(format!(
                "expected to lex a Float, but couldn't find decimal point"
            )));
        }

        self.forward(); // consume the decimal point
        let after_dec = self.extract_int()? as Float;

        Ok(before_dec + after_dec)
    }

    fn extract_ident(&mut self) -> LexerResult<String> {
        Ok(self.extract_pred(|c| c.is_alphabetic())?.to_owned())
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
            "abstract" => TAbstract,
            "default" => TDefault,
            "final" => TFinal,
            "from" => TFrom,
            "is" => TIs,
            "locals" => TLocals,
            "method" => TMethod,
            "native" => TNative,
            "private" => TPrivate,
            "protected" => TProtected,
            "public" => TPublic,
            "stack" => TStack,
            "static" => TStatic,
            "synchronized" => TSynchronized,
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
                self.forward();
                self.lex()?
            }

            '.' => {
                self.forward();

                if self.curr_char()?.is_alphabetic() {
                    let ident = self.extract_ident()?;
                    self.extract_directive(&ident)?
                } else {
                    TDot
                }
            }

            ';' => {
                self.forward();
                TSemicolon
            }

            ':' => {
                self.forward();
                TColon
            }

            '(' => {
                self.forward();
                TLeftParen
            }

            ')' => {
                self.forward();
                TRightParen
            }

            '=' => {
                self.forward();
                TAssign
            }

            '+' => {
                self.forward();
                TPlus
            }

            '-' => {
                self.forward();
                TMinus
            }

            '0'..='9' => TInt(self.extract_int()?),
            'a'..='z' | 'A'..='Z' | '_' => {
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
        if self.curr_pos >= self.size {
            Ok(Token::TEof)
        } else {
            self.lex_char(self.curr_char()?)
        }
    }
}

#[test]
mod tests {
    use super::Lexer;
}
