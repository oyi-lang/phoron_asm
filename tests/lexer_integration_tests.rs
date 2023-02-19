use phoron_asm::lexer::{
    Lexer,
    Token::{self, *},
};

use std::{error::Error, fs, path::Path};

fn lex<P>(testfile: P) -> Result<Vec<Token>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let src = fs::read_to_string(testfile)?;
    let mut lexer = Lexer::new(&src);
    let mut tokens = Vec::new();

    loop {
        let tok = lexer.lex()?;
        if tok == Token::TEof {
            break;
        }

        tokens.push(tok);
    }

    Ok(tokens)
}

#[test]
fn test_malign() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        TClass,
        TPublic,
        TIdent("Malign".to_string()),
        TSuper,
        TIdent("java/lang/Object".to_string()),
        TMethod,
        TPublic,
        TIdent("<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TAload0,
        TInvokespecial,
        TIdent("java/lang/Object/<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TReturn,
        TEnd,
        TEndMethod,
        TMethod,
        TPublic,
        TStatic,
        TIdent("main".to_string()),
        TLeftParen,
        TLeftSquareBracket,
        TIdent("Ljava/lang/String;".to_string()),
        TRightParen,
        TIdent("V".to_string()),
        TLimit,
        TStack,
        TInt(2),
        TLimit,
        TLocals,
        TInt(1),
        TBipush,
        TInt(100),
        TInvokevirtual,
        TIdent("java/lang/Object/clone".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("Ljava/lang/Object;".to_string()),
        TReturn,
        TEnd,
        TEndMethod,
    ];

    let actual_tokens = lex(Path::new("doc/grammar/Malign.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_fields() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        TClass,
        TPublic,
        TIdent("FieldsDemo".to_string().to_string()),
        TSuper,
        TIdent("java/lang/Object".to_string().to_string()),
        TField,
        TPrivate,
        TIdent("x".to_string().to_string()),
        TIdent("I".to_string().to_string()),
        TField,
        TPrivate,
        TIdent("y".to_string()),
        TIdent("D".to_string()),
        TField,
        TPrivate,
        TIdent("z".to_string()),
        TIdent("Ljava/lang/String;".to_string()),
        TAssign,
        TString("Foo".to_string()),
        TField,
        TPublic,
        TStatic,
        TFinal,
        TIdent("PI".to_string()),
        TIdent("D".to_string()),
        TAssign,
        TFloat(3.14159),
        TMethod,
        TPublic,
        TIdent("<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TAload0,
        TInvokespecial,
        TIdent("java/lang/Object/<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TReturn,
        TEnd,
        TEndMethod,
        TMethod,
        TPublic,
        TStatic,
        TIdent("main".to_string()),
        TLeftParen,
        TLeftSquareBracket,
        TIdent("Ljava/lang/String;".to_string()),
        TRightParen,
        TIdent("V".to_string()),
        TLimit,
        TStack,
        TInt(1),
        TLimit,
        TLocals,
        TInt(1),
        TEnd,
        TEndMethod,
    ];

    let actual_tokens = lex(Path::new("doc/grammar/FieldsDemo.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_hola_mundo() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        TClass,
        TPublic,
        TIdent("HolaMundo".to_string()),
        TSuper,
        TIdent("java/lang/Object".to_string()),
        TMethod,
        TPublic,
        TIdent("<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TAload0,
        TInvokespecial,
        TIdent("java/lang/Object/<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TReturn,
        TEnd,
        TEndMethod,
        TMethod,
        TPublic,
        TStatic,
        TIdent("main".to_string()),
        TLeftParen,
        TLeftSquareBracket,
        TIdent("Ljava/lang/String;".to_string()),
        TRightParen,
        TIdent("V".to_string()),
        TLimit,
        TStack,
        TInt(2),
        TGetstatic,
        TIdent("java/lang/System/out".to_string()),
        TIdent("Ljava/io/PrintStream;".to_string()),
        TLdc,
        TString("Hola, Mundo!".to_string()),
        TInvokevirtual,
        TIdent("java/io/PrintStream/println".to_string()),
        TLeftParen,
        TIdent("Ljava/lang/String;".to_string()),
        TRightParen,
        TIdent("V".to_string()),
        TReturn,
        TEnd,
        TEndMethod,
    ];

    let actual_tokens = lex(Path::new("doc/grammar/HolaMundo.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_hello_world() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        TClass,
        TPublic,
        TIdent("HelloWorld".to_string()),
        TSuper,
        TIdent("java/lang/Object".to_string()),
        TMethod,
        TPublic,
        TIdent("<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TAload0,
        TInvokespecial,
        TIdent("java/lang/Object/<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TReturn,
        TEnd,
        TEndMethod,
        TMethod,
        TPublic,
        TStatic,
        TIdent("main".to_string()),
        TLeftParen,
        TLeftSquareBracket,
        TIdent("Ljava/lang/String;".to_string()),
        TRightParen,
        TIdent("V".to_string()),
        TLimit,
        TStack,
        TInt(2),
        TGetstatic,
        TIdent("java/lang/System/out".to_string()),
        TIdent("Ljava/io/PrintStream;".to_string()),
        TLdc,
        TString("Hello, world".to_string()),
        TInvokevirtual,
        TIdent("java/io/PrintStream/println".to_string()),
        TLeftParen,
        TIdent("Ljava/lang/String;".to_string()),
        TRightParen,
        TIdent("V".to_string()),
        TReturn,
        TEnd,
        TEndMethod,
    ];

    let actual_tokens = lex(Path::new("doc/grammar/HelloWorld.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_malign_jasmin() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        TClass,
        TPublic,
        TIdent("MalignJasmin".to_string().to_string()),
        TSuper,
        TIdent("java/lang/Object".to_string().to_string()),
        TMethod,
        TPublic,
        TIdent("<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TAload0,
        TInvokespecial,
        TIdent("java/lang/Object/<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TReturn,
        TEnd,
        TEndMethod,
        TMethod,
        TPublic,
        TStatic,
        TIdent("main".to_string()),
        TLeftParen,
        TLeftSquareBracket,
        TIdent("Ljava/lang/String;".to_string()),
        TRightParen,
        TIdent("V".to_string()),
        TLimit,
        TStack,
        TInt(1),
        TLimit,
        TLocals,
        TInt(1),
        TBipush,
        TInt(100),
        TInvokevirtual,
        TIdent("java/lang/Object/clone".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("Ljava/lang/Object;".to_string()),
        TReturn,
        TEnd,
        TEndMethod,
    ];

    let actual_tokens = lex(Path::new("doc/grammar/MalignJasmin.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_privet_mir() -> Result<(), Box<dyn Error>> {
    // todo - unicode not supported
    //let actual_tokens = lex(Path::new("doc/grammar/PrivetMir.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_areturn() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        TClass,
        TPublic,
        TIdent("Areturn".to_string()),
        TSuper,
        TIdent("java/lang/Object".to_string()),
        TMethod,
        TPublic,
        TIdent("<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TAload0,
        TInvokespecial,
        TIdent("java/lang/Object/<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TReturn,
        TEnd,
        TEndMethod,
        TMethod,
        TPublic,
        TStatic,
        TIdent("makeIntArray".to_string()),
        TLeftParen,
        TIdent("I".to_string()),
        TRightParen,
        TLeftSquareBracket,
        TIdent("I".to_string()),
        TLimit,
        TLocals,
        TInt(2),
        TIload0,
        TNewarray,
        TIdent("int".to_string()),
        TAreturn,
        TEnd,
        TEndMethod,
        TMethod,
        TPublic,
        TStatic,
        TIdent("main".to_string()),
        TLeftParen,
        TLeftSquareBracket,
        TIdent("Ljava/lang/String;".to_string()),
        TRightParen,
        TIdent("V".to_string()),
        TLimit,
        TStack,
        TInt(3),
        TLimit,
        TLocals,
        TInt(3),
        TBipush,
        TInt(10),
        TInvokestatic,
        TIdent("Areturn/makeIntArray".to_string()),
        TLeftParen,
        TIdent("I".to_string()),
        TRightParen,
        TLeftSquareBracket,
        TIdent("I".to_string()),
        TAstore1,
        TReturn,
        TEnd,
        TEndMethod,
    ];

    let actual_tokens = lex(Path::new("doc/grammar/Areturn.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_create_array_of_threads() -> Result<(), Box<dyn Error>> {
    let expected_tokens = vec![
        TClass,
        TPublic,
        TIdent("CreateArrayOfThreads".to_string()),
        TSuper,
        TIdent("java/lang/Object".to_string()),
        TMethod,
        TPublic,
        TIdent("<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TAload0,
        TInvokespecial,
        TIdent("java/lang/Object/<init>".to_string()),
        TLeftParen,
        TRightParen,
        TIdent("V".to_string()),
        TReturn,
        TEnd,
        TEndMethod,
        TMethod,
        TPublic,
        TStatic,
        TIdent("main".to_string()),
        TLeftParen,
        TLeftSquareBracket,
        TIdent("Ljava/lang/String;".to_string()),
        TRightParen,
        TIdent("V".to_string()),
        TLimit,
        TStack,
        TInt(3),
        TLimit,
        TLocals,
        TInt(2),
        TBipush,
        TInt(10),
        TAnewarray,
        TIdent("java/lang/Thread".to_string()),
        TAstore1,
        TGetstatic,
        TIdent("java/lang/System/out".to_string()),
        TIdent("Ljava/io/PrintStream;".to_string()),
        TAload1,
        TInstanceof,
        TLeftSquareBracket,
        TIdent("Ljava/lang/Thread;".to_string()),
        TInvokevirtual,
        TIdent("java/io/PrintStream/println".to_string()),
        TLeftParen,
        TIdent("I".to_string()),
        TRightParen,
        TIdent("V".to_string()),
        TReturn,
        TEnd,
        TEndMethod,
    ];

    let actual_tokens = lex(Path::new("doc/grammar/CreateArrayOfThreads.pho"))?;
    assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_swap_top_two_items() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/SwapTopTwoItems.pho"))?;
    println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_create_matrix_of_int() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/HolaMundo.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_count() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/Count.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_catcher() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/Catcher.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_anewarray() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/Anewarray.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_args_to_main() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/ArgsToMain.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_count_jasmin2() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/CountJasmin2.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_add_nums_jasmin() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/AddNumsJasmin.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_stack_push_jasmin() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/HolaMundo.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_factorial_goto() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/FactorialGoto.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_factorial_jasmin() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/FactorialJasmin.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_check_array_type() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/CheckArrayType.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_print_hello_10_times() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/PrintHello10Times.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_add_nums() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/AddNums.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_count_jasmin() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/HolaMundo.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_string_buffer_demo() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/StringBufferDemo.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_array_demo() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/ArrayDemo.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}

#[test]
fn test_all_in_one() -> Result<(), Box<dyn Error>> {
    let actual_tokens = lex(Path::new("doc/grammar/AllInOne.pho"))?;
    //println!("{actual_tokens:#?}");
    //assert_eq!(expected_tokens, actual_tokens);

    Ok(())
}
