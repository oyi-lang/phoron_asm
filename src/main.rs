use phoron_asm::{cp_analyzer::ConstantPoolAnalyzer, lexer::Lexer, parser::Parser};
use std::fs;

fn usage() {
    eprintln!("Usage: phoron <source-file>");
    std::process::exit(0);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() != 1 {
        usage();
    } else {
        let src = fs::read_to_string(&args[0])?;

        let mut parser = Parser::new(Lexer::new(&src));
        let ast = parser.parse()?;
        //println!("{ast:#?}");

        let mut cp_analyzer = ConstantPoolAnalyzer::new();
        let cp = cp_analyzer.analyze(&ast)?;

        //let mut codegen = Codegen::new();
        //codegen.gen_bytecode(&ast, &cp);
    }

    Ok(())
}
