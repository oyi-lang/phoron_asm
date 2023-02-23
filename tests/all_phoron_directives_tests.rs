use phoron_asm::{
    ast::*,
    lexer::Lexer,
    parser::{Parser, ParserResult},
};
use std::error::Error;

const PROLOGUE: &'static str = r#"
    .class InstructionTest
    .super java/lang/Object

    .method public <init>()V
        aload_0
        invokespecial java/lang/Object/<init>()V
        return
    .end method

    .method public static main([Ljava/lamg/String;)V
        .limit stack 10
        .limit locals 10
    "#;

const EPILOGUE: &'static str = r#"
      return
    .end method
"#;

fn parse(src: &'static str) -> ParserResult<PhoronProgram> {
    let src = [PROLOGUE, src, EPILOGUE].concat();
    let mut parser = Parser::new(Lexer::new(&src));
    parser.parse()
}

#[test]
fn test_source_directive() -> Result<(), Box<dyn Error>> {
    Ok(())
}