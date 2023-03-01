use phoron_asm::{
    ast::*,
    cp_analyzer::{PhoronConstantPoolKind::*, *},
    lexer::Lexer,
    parser::Parser,
};

use std::collections::HashMap;
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
fn test_gen_constant_pool_malign() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_fields() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_hola_mundo() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[test]
fn test_gen_constant_pool_hello_world() -> Result<(), Box<dyn Error>> {
    let mut expected_cp = PhoronConstantPool::new();

    expected_cp.insert(Utf8("java/lang/Object".to_string()), 3);
    expected_cp.insert(String { string_index: 18 }, 19);
    expected_cp.insert(Class { name_index: 1 }, 2);
    expected_cp.insert(Utf8("java/io/PrintStream".to_string()), 20);
    expected_cp.insert(
        NameAndType {
            name_index: 22,
            descriptor_index: 23,
        },
        24,
    );
    expected_cp.insert(Class { name_index: 12 }, 13);
    expected_cp.insert(Utf8("println".to_string()), 22);
    expected_cp.insert(Utf8("java/lang/System".to_string()), 12);
    expected_cp.insert(Utf8("out".to_string()), 14);
    expected_cp.insert(Class { name_index: 3 }, 4);
    expected_cp.insert(
        Fieldref {
            class_index: 13,
            name_and_type_index: 16,
        },
        17,
    );
    expected_cp.insert(Utf8("([Ljava/lang/String;)V".to_string()), 11);
    expected_cp.insert(
        NameAndType {
            name_index: 14,
            descriptor_index: 15,
        },
        16,
    );
    expected_cp.insert(Class { name_index: 20 }, 21);
    expected_cp.insert(Utf8("(Ljava/lang/String;)V".to_string()), 23);
    expected_cp.insert(
        Methodref {
            class_index: 21,
            name_and_type_index: 24,
        },
        25,
    );
    expected_cp.insert(Utf8("Hello, world".to_string()), 18);
    expected_cp.insert(
        NameAndType {
            name_index: 5,
            descriptor_index: 6,
        },
        8,
    );
    expected_cp.insert(Utf8("()V".to_string()), 6);
    expected_cp.insert(Utf8("Ljava/io/PrintStream;".to_string()), 15);
    expected_cp.insert(Utf8("HelloWorld".to_string()), 1);
    expected_cp.insert(Utf8("<init>".to_string()), 5);
    expected_cp.insert(
        Methodref {
            class_index: 4,
            name_and_type_index: 8,
        },
        9,
    );
    expected_cp.insert(Utf8("main".to_string()), 10);
    expected_cp.insert(Utf8("Code".to_string()), 7);

    let program = parse("doc/grammar/HelloWorld.pho")?;
    let mut cp_analyzer = ConstantPoolAnalyzer::new();
    let actual_cp = cp_analyzer.analyze(&program)?;
    assert_eq!(expected_cp, actual_cp);

    Ok(())
}

#[ignore]
#[test]
fn test_gen_constant_pool_malign_jasmin() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_privet_mir() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_areturn() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_create_array_of_threads() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_swap_top_two_items() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_create_matrix_of_int() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_count() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_catcher() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_anewarray() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_args_to_main() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_count_jasmin2() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_add_nums_jasmin() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_stack_push_jasmin() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_factorial_goto() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_factorial_jasmin() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_check_array_type() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_print_hello_10_times() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_add_nums() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_count_jasmin() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_string_buffer_demo() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_array_demo() -> Result<(), Box<dyn Error>> {
    todo!()
}

#[ignore]
#[test]
fn test_gen_constant_pool_all_in_one() -> Result<(), Box<dyn Error>> {
    todo!()
}
