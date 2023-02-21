use phoron_asm::{
    ast::{
        JvmInstruction::*, PhoronClassOrInterface::*, PhoronDirective::*, PhoronFieldDescriptor::*,
        PhoronInstruction::*, PhoronReturnDescriptor::*, *,
    },
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
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "Malign".to_string(),
                access_flags: vec![PhoronClassOrInterfaceAccessFlag::AccPublic],
            }),
            super_def: PhoronSuperDef {
                super_class_name: "java/lang/Object".to_string(),
            },
        },
        body: PhoronBody {
            field_defs: vec![],
            method_defs: vec![
                PhoronMethodDef {
                    name: "<init>".to_string(),
                    access_flags: vec![PhoronMethodAccessFlag::AccPublic],
                    descriptor: PhoronMethodDescriptor {
                        param_descriptor: None,
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            descriptor: PhoronMethodDescriptor {
                                param_descriptor: None,
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "main".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPublic,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    descriptor: PhoronMethodDescriptor {
                        param_descriptor: None,
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(1)),
                        JvmInstruction(Bipush(100)),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "clone".to_string(),
                            descriptor: PhoronMethodDescriptor {
                                param_descriptor: None,
                                return_descriptor: FieldDescriptor(ObjectType {
                                    class_name: "java/lang/Object".to_string(),
                                }),
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/Malign.pho")?;
    assert_eq!(expected_ast, actual_ast);

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