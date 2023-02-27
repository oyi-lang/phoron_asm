use phoron_asm::{
    ast::*,
    cp_analyzer::{ConstantPoolKind::*, *},
    lexer::Lexer,
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

#[ignore]
#[test]
fn test_parse_malign() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_fields() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_hola_mundo() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[test]
fn test_parse_hello_world() -> Result<(), Box<dyn Error>> {
    let mut expected_cp = ConstantPool::new();
    expected_cp.insert(Utf8("HelloWorld".to_string()), 1);
    expected_cp.insert(Utf8("java/lang/Object".to_string()), 2);
    expected_cp.insert(Utf8("<init>".to_string()), 3);
    expected_cp.insert(Utf8("()V".to_string()), 4);
    expected_cp.insert(Utf8("main".to_string()), 5);
    expected_cp.insert(Utf8("([Ljava/lang/String;)V".to_string()), 6);
    expected_cp.insert(Utf8("java/lang/System".to_string()), 7);
    expected_cp.insert(Class { name_idx: 7 }, 8);
    expected_cp.insert(Utf8("out".to_string()), 9);
    expected_cp.insert(Utf8("Ljava/io/PrintStream;".to_string()), 10);
    expected_cp.insert(
        NameAndType {
            name_idx: 9,
            descriptor_idx: 10,
        },
        11,
    );
    expected_cp.insert(
        Fieldref {
            class_idx: 8,
            name_and_type_idx: 11,
        },
        12,
    );
    expected_cp.insert(Utf8("java/io/PrintStream".to_string()), 13);
    expected_cp.insert(Class { name_idx: 13 }, 14);
    expected_cp.insert(Utf8("println".to_string()), 15);
    expected_cp.insert(Utf8("(Ljava/lang/String;)V".to_string()), 16);
    expected_cp.insert(
        NameAndType {
            name_idx: 15,
            descriptor_idx: 16,
        },
        17,
    );
    expected_cp.insert(
        Methodref {
            class_idx: 14,
            name_and_type_idx: 17,
        },
        18,
    );

    let program = parse("doc/grammar/HelloWorld.pho")?;
    let mut cp_analyzer = ConstantPoolAnalyzer::new();
    let actual_cp = cp_analyzer.analyze(&program)?;
    assert_eq!(expected_cp, actual_cp);

    Ok(())
}

#[ignore]
#[test]
fn test_parse_malign_jasmin() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_privet_mir() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_areturn() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_create_array_of_threads() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_swap_top_two_items() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_create_matrix_of_int() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_count() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_catcher() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_anewarray() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_args_to_main() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_count_jasmin2() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_add_nums_jasmin() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_stack_push_jasmin() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_factorial_goto() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_factorial_jasmin() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_check_array_type() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_print_hello_10_times() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_add_nums() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_count_jasmin() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_string_buffer_demo() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_array_demo() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_parse_all_in_one() -> Result<(), Box<dyn Error>> {
    todo!()
}
