use phoron_asm::lexer::{Lexer, Token};
use std::io::{self, Write};

const PROMPT: &'static str = ">> ";

fn get_input() -> Result<String, Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("{}", PROMPT);
        io::stdout().lock().flush()?;

        let input = get_input()?;
        let mut lexer = Lexer::new(&input);

        loop {
            let tok = lexer.lex()?;
            println!("{tok:?}");

            if tok == Token::TEof {
                break;
            }
        }
    }
}
