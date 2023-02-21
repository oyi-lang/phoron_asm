use phoron_asm::{
    ast::{
        JvmInstruction::*, PhoronBaseType::*, PhoronClassOrInterface::*, PhoronDirective::*,
        PhoronFieldDescriptor::*, PhoronInstruction::*, PhoronReturnDescriptor::*, PrimitiveType,
        *,
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
                        param_descriptor: Some(ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }),
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
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "FieldsDemo".to_string(),
                access_flags: vec![PhoronClassOrInterfaceAccessFlag::AccPublic],
            }),
            super_def: PhoronSuperDef {
                super_class_name: "java/lang/Object".to_string(),
            },
        },
        body: PhoronBody {
            field_defs: vec![
                PhoronFieldDef {
                    name: "x".to_string(),
                    access_flags: vec![PhoronFieldAccessFlag::AccPrivate],
                    descriptor: BaseType(Integer),
                    init_val: None,
                },
                PhoronFieldDef {
                    name: "y".to_string(),
                    access_flags: vec![PhoronFieldAccessFlag::AccPrivate],
                    descriptor: BaseType(Double),
                    init_val: None,
                },
                PhoronFieldDef {
                    name: "z".to_string(),
                    access_flags: vec![PhoronFieldAccessFlag::AccPrivate],
                    descriptor: ObjectType {
                        class_name: "java/lang/String".to_string(),
                    },
                    init_val: Some(PhoronFieldInitValue::QuotedString("Foo".to_string())),
                },
                PhoronFieldDef {
                    name: "PI".to_string(),
                    access_flags: vec![
                        PhoronFieldAccessFlag::AccPublic,
                        PhoronFieldAccessFlag::AccStatic,
                        PhoronFieldAccessFlag::AccFinal,
                    ],
                    descriptor: BaseType(Double),
                    init_val: Some(PhoronFieldInitValue::Double(3.14159)),
                },
            ],
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
                        param_descriptor: Some(ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }),
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(1)),
                        PhoronDirective(LimitLocals(1)),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/FieldsDemo.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_hola_mundo() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "HolaMundo".to_string(),
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
                        param_descriptor: Some(ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }),
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Hola, Mundo!".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            descriptor: PhoronMethodDescriptor {
                                param_descriptor: Some(ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }),
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/HolaMundo.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_hello_world() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "HelloWorld".to_string(),
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
                        param_descriptor: Some(ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }),
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Hello, world".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            descriptor: PhoronMethodDescriptor {
                                param_descriptor: Some(ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }),
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/HelloWorld.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_malign_jasmin() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "MalignJasmin".to_string(),
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
                        param_descriptor: Some(ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }),
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(1)),
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

    let actual_ast = parse("doc/grammar/MalignJasmin.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_privet_mir() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "PrivetMir".to_string(),
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
                        param_descriptor: Some(ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }),
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Привет, мир!".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            descriptor: PhoronMethodDescriptor {
                                param_descriptor: Some(ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }),
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/PrivetMir.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_areturn() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "Areturn".to_string(),
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
                    name: "makeIntArray".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPublic,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    descriptor: PhoronMethodDescriptor {
                        param_descriptor: Some(BaseType(Integer)),
                        return_descriptor: FieldDescriptor(ArrayType {
                            component_type: Box::new(BaseType(Integer)),
                        }),
                    },
                    instructions: vec![
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(Iload0),
                        JvmInstruction(Newarray {
                            component_type: PrimitiveType::Int,
                        }),
                        JvmInstruction(Areturn),
                    ],
                },
                PhoronMethodDef {
                    name: "main".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPublic,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    descriptor: PhoronMethodDescriptor {
                        param_descriptor: Some(ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }),
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(3)),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Invokestatic {
                            class_name: "Areturn".to_string(),
                            method_name: "makeIntArray".to_string(),
                            descriptor: PhoronMethodDescriptor {
                                param_descriptor: Some(BaseType(Integer)),
                                return_descriptor: FieldDescriptor(ArrayType {
                                    component_type: Box::new(BaseType(Integer)),
                                }),
                            },
                        }),
                        JvmInstruction(Astore1),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/Areturn.pho")?;
    assert_eq!(expected_ast, actual_ast);

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