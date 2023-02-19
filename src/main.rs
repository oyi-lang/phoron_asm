use phoron_asm::lexer::{Lexer, Token};
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
        let mut lexer = Lexer::new(&src);

        loop {
            let tok = lexer.lex()?;
            println!("{tok:?}");

            if tok == Token::TEof {
                break;
            }
        }
    }

    Ok(())
}
