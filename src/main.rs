use phoron_asm::{
    codegen::Codegen, cp_analyzer::ConstantPoolAnalyzer, lexer::Lexer, parser::Parser,
};
use std::{fs, io::BufWriter};

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

        let mut cp_analyzer = ConstantPoolAnalyzer::new();
        let cp = cp_analyzer.analyze(&ast)?;
        println!("{cp:#?}");

        let mut outfile = BufWriter::new(fs::File::create(match args[0].find('.') {
            Some(pos) => format!("{}.class", &args[0][..pos]),
            None => "out.class".into(),
        })?);

        let mut codegen = Codegen::new(&mut outfile);
        codegen.gen_bytecode(&ast, &cp)?;
    }

    Ok(())
}
