use phoron_asm::{
    lexer::{Lexer, Token, TokenKind::*},
    sourcefile::{SourceFile, Span},
};

use std::{error::Error, path::Path};

fn lex<P>(testfile: P) -> Result<Vec<Token>, Box<dyn Error>>
where
    P: AsRef<Path> + Copy,
{
    let source_file = SourceFile::new(testfile.as_ref()).map_err(|err| Box::new(err))?;
    let mut lexer = Lexer::new(&source_file);
    let mut tokens = Vec::new();

    loop {
        let tok = lexer.lex().unwrap_or(Token {
            kind: TEof,
            span: Span::default(),
        });

        if tok.kind == TEof {
            break;
        }

        tokens.push(tok);
    }

    Ok(tokens)
}

#[test]
fn test_lex_malign() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Malign".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(100),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/clone".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/Object;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/Malign.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_fields() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("FieldsDemo".to_string().to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string().to_string()),
            span: Span::default(),
        },
        Token {
            kind: TField,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TIdent("x".to_string().to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string().to_string()),
            span: Span::default(),
        },
        Token {
            kind: TField,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TIdent("y".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("D".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TField,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TIdent("z".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAssign,
            span: Span::default(),
        },
        Token {
            kind: TString("Foo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TField,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TFinal,
            span: Span::default(),
        },
        Token {
            kind: TIdent("PI".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("F".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAssign,
            span: Span::default(),
        },
        Token {
            kind: TFloat(3.14159),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/FieldsDemo.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_hola_mundo() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("HolaMundo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("Hola, Mundo!".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/HolaMundo.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_hello_world() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("HelloWorld".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("Hello, world".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/HelloWorld.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_malign_jasmin() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("MalignJasmin".to_string().to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string().to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(100),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/clone".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/Object;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/MalignJasmin.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_privet_mir() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("PrivetMir".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("Привет, мир!".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/PrivetMir.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_areturn() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Areturn".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("makeIntArray".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TIload0,
            span: Span::default(),
        },
        Token {
            kind: TNewarray,
            span: Span::default(),
        },
        Token {
            kind: TIdent("int".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAreturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Areturn/makeIntArray".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/Areturn.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_create_array_of_threads() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("CreateArrayOfThreads".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TAnewarray,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Thread".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TInstanceof,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/Thread;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/CreateArrayOfThreads.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_swap_top_two_items() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("SwapTopTwoItems".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIconst1,
            span: Span::default(),
        },
        Token {
            kind: TIconst2,
            span: Span::default(),
        },
        Token {
            kind: TSwap,
            span: Span::default(),
        },
        Token {
            kind: TPop,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/SwapTopTwoItems.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_create_matrix_of_int() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("CreateMatrixOfInt".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(5),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(7),
            span: Span::default(),
        },
        Token {
            kind: TMultianewarray,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[[[I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TInstanceof,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[[[I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/CreateMatrixOfInt.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_count() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Count".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TIstore2,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Loop".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TIload2,
            span: Span::default(),
        },
        Token {
            kind: TIsub,
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/String/valueOf".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore3,
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TAload3,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIinc,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TInt(-1),
            span: Span::default(),
        },
        Token {
            kind: TIload2,
            span: Span::default(),
        },
        Token {
            kind: TIfne,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Loop".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/Count.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_catcher() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Catcher".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TCatch,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Exception".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TFrom,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Label1".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TTo,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Label2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TUsing,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Handler".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Label1".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TNew,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Exception".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TDup,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Exception/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAthrow,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Label2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Handler".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TPop,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("Exception Caught".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/Catcher.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_anewarray() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Anewarray".to_string().to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string().to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string().to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string().to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string().to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TAnewarray,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Thread".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TIconst2,
            span: Span::default(),
        },
        Token {
            kind: TAnewarray,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore2,
            span: Span::default(),
        },
        Token {
            kind: TAload2,
            span: Span::default(),
        },
        Token {
            kind: TIconst0,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(5),
            span: Span::default(),
        },
        Token {
            kind: TAnewarray,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/String".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAastore,
            span: Span::default(),
        },
        Token {
            kind: TAload2,
            span: Span::default(),
        },
        Token {
            kind: TIconst1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(5),
            span: Span::default(),
        },
        Token {
            kind: TAnewarray,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/String".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAastore,
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/Anewarray.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_args_to_main() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("ArgsToMain".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TArraylength,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/ArgsToMain.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_count_jasmin2() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("CountJasmin2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TIconst0,
            span: Span::default(),
        },
        Token {
            kind: TIstore1,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore2,
            span: Span::default(),
        },
        Token {
            kind: TIdent("loop".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TAload2,
            span: Span::default(),
        },
        Token {
            kind: TIload1,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIinc,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TIload,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TIficmplt,
            span: Span::default(),
        },
        Token {
            kind: TIdent("loop".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/CountJasmin2.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_add_nums_jasmin() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AddNumsJasmin".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("addNums".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("II".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TIload0,
            span: Span::default(),
        },
        Token {
            kind: TIload1,
            span: Span::default(),
        },
        Token {
            kind: TIadd,
            span: Span::default(),
        },
        Token {
            kind: TIreturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIconst1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(99),
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AddNumsJasmin/addNums".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("II".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/AddNumsJasmin.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_stack_push_jasmin() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("StackPushJasmin".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(40),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TIconstm1,
            span: Span::default(),
        },
        Token {
            kind: TIconst0,
            span: Span::default(),
        },
        Token {
            kind: TIconst1,
            span: Span::default(),
        },
        Token {
            kind: TIconst2,
            span: Span::default(),
        },
        Token {
            kind: TIconst3,
            span: Span::default(),
        },
        Token {
            kind: TIconst4,
            span: Span::default(),
        },
        Token {
            kind: TIconst5,
            span: Span::default(),
        },
        Token {
            kind: TLconst0,
            span: Span::default(),
        },
        Token {
            kind: TLconst1,
            span: Span::default(),
        },
        Token {
            kind: TFconst0,
            span: Span::default(),
        },
        Token {
            kind: TFconst1,
            span: Span::default(),
        },
        Token {
            kind: TDconst0,
            span: Span::default(),
        },
        Token {
            kind: TDconst1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TSipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(1000),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("Hello, world".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdcw,
            span: Span::default(),
        },
        Token {
            kind: TString("Hola, mundo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdc2w,
            span: Span::default(),
        },
        Token {
            kind: TInt(12345),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/StackPushJasmin.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_factorial_goto() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("FactorialGoto".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("factorial".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TIconst1,
            span: Span::default(),
        },
        Token {
            kind: TIstore1,
            span: Span::default(),
        },
        Token {
            kind: TIconst2,
            span: Span::default(),
        },
        Token {
            kind: TIstore2,
            span: Span::default(),
        },
        Token {
            kind: TIdent("floop".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIload2,
            span: Span::default(),
        },
        Token {
            kind: TIload0,
            span: Span::default(),
        },
        Token {
            kind: TIficmpgt,
            span: Span::default(),
        },
        Token {
            kind: TIdent("back".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIload1,
            span: Span::default(),
        },
        Token {
            kind: TIload2,
            span: Span::default(),
        },
        Token {
            kind: TImul,
            span: Span::default(),
        },
        Token {
            kind: TIstore1,
            span: Span::default(),
        },
        Token {
            kind: TIinc,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TGoto,
            span: Span::default(),
        },
        Token {
            kind: TIdent("floop".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("back".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIload1,
            span: Span::default(),
        },
        Token {
            kind: TIreturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("FactorialGoto/factorial".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/FactorialGoto.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_factorial_jasmin() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("FactorialJasmin".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("factorial".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TIconst1,
            span: Span::default(),
        },
        Token {
            kind: TIstore1,
            span: Span::default(),
        },
        Token {
            kind: TIconst2,
            span: Span::default(),
        },
        Token {
            kind: TIstore2,
            span: Span::default(),
        },
        Token {
            kind: TIdent("loop".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIload2,
            span: Span::default(),
        },
        Token {
            kind: TIload1,
            span: Span::default(),
        },
        Token {
            kind: TImul,
            span: Span::default(),
        },
        Token {
            kind: TIstore1,
            span: Span::default(),
        },
        Token {
            kind: TIinc,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TIload2,
            span: Span::default(),
        },
        Token {
            kind: TIload0,
            span: Span::default(),
        },
        Token {
            kind: TIficmple,
            span: Span::default(),
        },
        Token {
            kind: TIdent("loop".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIload1,
            span: Span::default(),
        },
        Token {
            kind: TIreturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("FactorialJasmin/factorial".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/FactorialJasmin.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_check_array_type() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("CheckArrayType".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(5),
            span: Span::default(),
        },
        Token {
            kind: TNewarray,
            span: Span::default(),
        },
        Token {
            kind: TIdent("int".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TInstanceof,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TNewarray,
            span: Span::default(),
        },
        Token {
            kind: TIdent("char".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore2,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload2,
            span: Span::default(),
        },
        Token {
            kind: TInstanceof,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/CheckArrayType.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_print_hello_10_times() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("PrintHello10Times".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TIconst1,
            span: Span::default(),
        },
        Token {
            kind: TIstore1,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore2,
            span: Span::default(),
        },
        Token {
            kind: TIdent("loop".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TAload2,
            span: Span::default(),
        },
        Token {
            kind: TIload1,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/print".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload2,
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString(" - ".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/print".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload2,
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("Hello".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIinc,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TIload1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TIficmple,
            span: Span::default(),
        },
        Token {
            kind: TIdent("loop".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/PrintHello10Times.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_add_nums() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AddNums".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(5),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(8),
            span: Span::default(),
        },
        Token {
            kind: TNew,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/util/Scanner".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TDup,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/in".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/InputStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/util/Scanner/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/InputStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TJsr,
            span: Span::default(),
        },
        Token {
            kind: TIdent("ReadNum".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIstore3,
            span: Span::default(),
        },
        Token {
            kind: TJsr,
            span: Span::default(),
        },
        Token {
            kind: TIdent("ReadNum".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIstore,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TIload3,
            span: Span::default(),
        },
        Token {
            kind: TIload,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TJsr,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AddNum".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIstore,
            span: Span::default(),
        },
        Token {
            kind: TInt(5),
            span: Span::default(),
        },
        Token {
            kind: TIload,
            span: Span::default(),
        },
        Token {
            kind: TInt(5),
            span: Span::default(),
        },
        Token {
            kind: TJsr,
            span: Span::default(),
        },
        Token {
            kind: TIdent("PrintSum".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TIdent("PrintSum".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TAstore,
            span: Span::default(),
        },
        Token {
            kind: TInt(7),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSwap,
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/String/valueOf".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRet,
            span: Span::default(),
        },
        Token {
            kind: TInt(7),
            span: Span::default(),
        },
        Token {
            kind: TIdent("AddNum".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TAstore,
            span: Span::default(),
        },
        Token {
            kind: TInt(6),
            span: Span::default(),
        },
        Token {
            kind: TIadd,
            span: Span::default(),
        },
        Token {
            kind: TRet,
            span: Span::default(),
        },
        Token {
            kind: TInt(6),
            span: Span::default(),
        },
        Token {
            kind: TIdent("ReadNum".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TAstore2,
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/util/Scanner/nextInt".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRet,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/AddNums.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_count_jasmin() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("CountJasmin".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TIstore2,
            span: Span::default(),
        },
        Token {
            kind: TIdent("loop".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TIload2,
            span: Span::default(),
        },
        Token {
            kind: TIsub,
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/String/valueOf".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore3,
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TAload3,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIinc,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TInt(-1),
            span: Span::default(),
        },
        Token {
            kind: TIload2,
            span: Span::default(),
        },
        Token {
            kind: TIfne,
            span: Span::default(),
        },
        Token {
            kind: TIdent("loop".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/CountJasmin.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_string_buffer_demo() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("StringBufferDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("sbDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/Object;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TCheckcast,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/StringBuffer".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("Hello, mundo!".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/StringBuffer/append".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/StringBuffer;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TNew,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/StringBuffer".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TDup,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/StringBuffer/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("StringBufferDemo/sbDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/Object;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/StringBuffer/toString".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/StringBufferDemo.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_array_demo() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("ArrayDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("setArr".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[III".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TIload1,
            span: Span::default(),
        },
        Token {
            kind: TIload2,
            span: Span::default(),
        },
        Token {
            kind: TIastore,
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("printArr".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[II".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TIload1,
            span: Span::default(),
        },
        Token {
            kind: TIaload,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(5),
            span: Span::default(),
        },
        Token {
            kind: TNewarray,
            span: Span::default(),
        },
        Token {
            kind: TIdent("int".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TIaload,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(0),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("ArrayDemo/setArr".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[III".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TIastore,
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TIastore,
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TIastore,
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(5),
            span: Span::default(),
        },
        Token {
            kind: TIastore,
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(0),
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("ArrayDemo/printArr".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[II".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TIaload,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TIaload,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TIaload,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TIaload,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/ArrayDemo.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_lex_all_in_one() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        Token {
            kind: TSource,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AllInOne.pho".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TClass,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AllInOne".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSuper,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Thread".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TField,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TIdent("x".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TField,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TIdent("y".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("D".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAssign,
            span: Span::default(),
        },
        Token {
            kind: TFloat(1.2345),
            span: Span::default(),
        },
        Token {
            kind: TField,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("z".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAssign,
            span: Span::default(),
        },
        Token {
            kind: TInt(12345),
            span: Span::default(),
        },
        Token {
            kind: TField,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TFinal,
            span: Span::default(),
        },
        Token {
            kind: TIdent("PREFIX".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAssign,
            span: Span::default(),
        },
        Token {
            kind: TString("FooBar".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Thread/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("exceptionsDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TCatch,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Exception".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TFrom,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Label1".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TTo,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Label2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TUsing,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Handler".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Label1".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TNew,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Exception".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TDup,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Exception/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAthrow,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Label2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Handler".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TPop,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("Exception caught".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("finallyDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(4),
            span: Span::default(),
        },
        Token {
            kind: TCatch,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/FileNotFoundException".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TFrom,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Start".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TTo,
            span: Span::default(),
        },
        Token {
            kind: TIdent("End1".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TUsing,
            span: Span::default(),
        },
        Token {
            kind: TIdent("NotFound".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TCatch,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/IOException".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TFrom,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Start".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TTo,
            span: Span::default(),
        },
        Token {
            kind: TIdent("End2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TUsing,
            span: Span::default(),
        },
        Token {
            kind: TIdent("IOE".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TCatch,
            span: Span::default(),
        },
        Token {
            kind: TIdent("all".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TFrom,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Start".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TTo,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Done".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TUsing,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Other_Exception".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Start".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TNew,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/FileInputStream".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TDup,
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("myfile".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/FileInputStream/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TIdent("End1".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TGoto,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Done".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("NotFound".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TPop,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("No such file".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TGoto,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Done".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("IOE".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TPop,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("IO Exception occurred".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TGoto,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Done".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("End2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Done".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TJsr,
            span: Span::default(),
        },
        Token {
            kind: TIdent("FinalSub".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Other_Exception".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TAstore2,
            span: Span::default(),
        },
        Token {
            kind: TJsr,
            span: Span::default(),
        },
        Token {
            kind: TIdent("FinalSub".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload2,
            span: Span::default(),
        },
        Token {
            kind: TAthrow,
            span: Span::default(),
        },
        Token {
            kind: TIdent("FinalSub".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TAstore3,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("Done".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRet,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TSynchronized,
            span: Span::default(),
        },
        Token {
            kind: TIdent("synchronizedMethoDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TIdent("monitoDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/Object;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TMonitorenter,
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TMonitorexit,
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TIdent("checkCastDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TCheckcast,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Object".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TIdent("instanceofDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TAload0,
            span: Span::default(),
        },
        Token {
            kind: TInstanceof,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/Thread".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSwap,
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/String/valueOf".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("subroutinesDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("Hello".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TJsr,
            span: Span::default(),
        },
        Token {
            kind: TIdent("PrintString".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString(", world".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TJsr,
            span: Span::default(),
        },
        Token {
            kind: TIdent("PrintString".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TIdent("PrintString".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSwap,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRet,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("lookupswitchDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TIstore1,
            span: Span::default(),
        },
        Token {
            kind: TIload1,
            span: Span::default(),
        },
        Token {
            kind: TLookupswitch,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIdent("R1".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIdent("R2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInt(100),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIdent("R3".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TDefault,
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIdent("R4".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("R1".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIconst1,
            span: Span::default(),
        },
        Token {
            kind: TIreturn,
            span: Span::default(),
        },
        Token {
            kind: TIdent("R2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIconst2,
            span: Span::default(),
        },
        Token {
            kind: TIreturn,
            span: Span::default(),
        },
        Token {
            kind: TIdent("R3".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIconst3,
            span: Span::default(),
        },
        Token {
            kind: TIreturn,
            span: Span::default(),
        },
        Token {
            kind: TIdent("R4".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIconst0,
            span: Span::default(),
        },
        Token {
            kind: TIreturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("tableswitchDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TIconst3,
            span: Span::default(),
        },
        Token {
            kind: TIstore1,
            span: Span::default(),
        },
        Token {
            kind: TIload1,
            span: Span::default(),
        },
        Token {
            kind: TTableswitch,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TIdent("R1".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("R2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("R3".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TDefault,
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIdent("R4".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("R1".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIconst1,
            span: Span::default(),
        },
        Token {
            kind: TIreturn,
            span: Span::default(),
        },
        Token {
            kind: TIdent("R2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIconst2,
            span: Span::default(),
        },
        Token {
            kind: TIreturn,
            span: Span::default(),
        },
        Token {
            kind: TIdent("R3".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIconst3,
            span: Span::default(),
        },
        Token {
            kind: TIreturn,
            span: Span::default(),
        },
        Token {
            kind: TIdent("R4".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TIconst0,
            span: Span::default(),
        },
        Token {
            kind: TIreturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPrivate,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("varDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(1),
            span: Span::default(),
        },
        Token {
            kind: TVar,
            span: Span::default(),
        },
        Token {
            kind: TInt(0),
            span: Span::default(),
        },
        Token {
            kind: TIs,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Count".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TFrom,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Label1".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TTo,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Label2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Label1".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TBipush,
            span: Span::default(),
        },
        Token {
            kind: TInt(10),
            span: Span::default(),
        },
        Token {
            kind: TIstore0,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Label2".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
        Token {
            kind: TMethod,
            span: Span::default(),
        },
        Token {
            kind: TPublic,
            span: Span::default(),
        },
        Token {
            kind: TStatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("main".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("[Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TStack,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TLimit,
            span: Span::default(),
        },
        Token {
            kind: TLocals,
            span: Span::default(),
        },
        Token {
            kind: TInt(3),
            span: Span::default(),
        },
        Token {
            kind: TThrows,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/RuntimeException".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLdc,
            span: Span::default(),
        },
        Token {
            kind: TString("Hello, world".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AllInOne/exceptionsDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AllInOne/finallyDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TNew,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AllInOne".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TDup,
            span: Span::default(),
        },
        Token {
            kind: TInvokespecial,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AllInOne/<init>".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAstore1,
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AllInOne/instanceofDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TAload1,
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AllInOne/checkCastDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AllInOne/subroutinesDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AllInOne/lookupswitchDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TJsr,
            span: Span::default(),
        },
        Token {
            kind: TIdent("PrintInt".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("AllInOne/tableswitchDemo".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TJsr,
            span: Span::default(),
        },
        Token {
            kind: TIdent("PrintInt".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TReturn,
            span: Span::default(),
        },
        Token {
            kind: TIdent("PrintInt".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TColon,
            span: Span::default(),
        },
        Token {
            kind: TAstore2,
            span: Span::default(),
        },
        Token {
            kind: TGetstatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/System/out".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/io/PrintStream;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TSwap,
            span: Span::default(),
        },
        Token {
            kind: TInvokestatic,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/lang/String/valueOf".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("I".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TInvokevirtual,
            span: Span::default(),
        },
        Token {
            kind: TIdent("java/io/PrintStream/println".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TLeftParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("Ljava/lang/String;".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRightParen,
            span: Span::default(),
        },
        Token {
            kind: TIdent("V".to_string()),
            span: Span::default(),
        },
        Token {
            kind: TRet,
            span: Span::default(),
        },
        Token {
            kind: TInt(2),
            span: Span::default(),
        },
        Token {
            kind: TEnd,
            span: Span::default(),
        },
        Token {
            kind: TEndMethod,
            span: Span::default(),
        },
    ];

    let actual_tokens = lex(Path::new("doc/grammar/AllInOne.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}
