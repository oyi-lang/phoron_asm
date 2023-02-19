use phoron_asm::{
    ast::PhoronProgram,
    lexer::{
        Lexer,
        Token::{self, *},
    },
    parser::Parser,
};

use std::{error::Error, fs, path::Path};

fn parse<P>(testfile: P) -> Result<PhoronProgram, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let src = fs::read_to_string(testfile)?;
    let mut parser = Parser::new(Lexer::new(&src));
    let program = parser.parse()?;
    Ok(program)
}

#[test]
fn test_parse_malign() -> Result<(), Box<dyn Error>> {
    println!("{:#?}", parse(Path::new("doc/grammar/Malign.pho"))?);
    Ok(())
}

#[test]
fn test_parse_fields() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_hola_mundo() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_hello_world() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_malign_jasmin() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_privet_mir() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_areturn() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_create_array_of_threads() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_swap_top_two_items() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_create_matrix_of_int() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_count() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_catcher() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_anewarray() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_args_to_main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_count_jasmin2() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_add_nums_jasmin() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_stack_push_jasmin() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_factorial_goto() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_factorial_jasmin() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_check_array_type() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_print_hello_10_times() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_add_nums() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_count_jasmin() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_string_buffer_demo() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_array_demo() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[test]
fn test_parse_all_in_one() -> Result<(), Box<dyn Error>> {
    Ok(())
}