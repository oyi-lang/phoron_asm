use phoron_asm::{
    ast::{
        JvmInstruction::*, PhoronBaseType::*, PhoronClassOrInterface::*, PhoronDirective::*,
        PhoronFieldDescriptor::*, PhoronInstruction::*, PhoronReturnDescriptor::*, *,
    },
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

#[test]
fn test_parse_malign() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "Malign".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(1)),
                        JvmInstruction(Bipush(100)),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "clone".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    field_descriptor: BaseType(Integer),
                    init_val: None,
                },
                PhoronFieldDef {
                    name: "y".to_string(),
                    access_flags: vec![PhoronFieldAccessFlag::AccPrivate],
                    field_descriptor: BaseType(Double),
                    init_val: None,
                },
                PhoronFieldDef {
                    name: "z".to_string(),
                    access_flags: vec![PhoronFieldAccessFlag::AccPrivate],
                    field_descriptor: ObjectType {
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
                    field_descriptor: BaseType(Float),
                    init_val: Some(PhoronFieldInitValue::Double(3.14159)),
                },
            ],
            method_defs: vec![
                PhoronMethodDef {
                    name: "<init>".to_string(),
                    access_flags: vec![PhoronMethodAccessFlag::AccPublic],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(1)),
                        PhoronDirective(LimitLocals(1)),
                        JvmInstruction(Return),
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
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Hola, Mundo!".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
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
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Hello, world".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
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
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(1)),
                        PhoronDirective(LimitLocals(1)),
                        JvmInstruction(Bipush(100)),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "clone".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Привет, мир!".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
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
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![BaseType(Integer)],
                        return_descriptor: FieldDescriptor(ArrayType {
                            component_type: Box::new(BaseType(Integer)),
                        }),
                    },
                    instructions: vec![
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(Iload0),
                        JvmInstruction(Newarray {
                            component_type: PhoronBaseType::Integer,
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(3)),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Invokestatic {
                            class_name: "Areturn".to_string(),
                            method_name: "makeIntArray".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
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
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "CreateArrayOfThreads".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Anewarray {
                            component_type: ClassOrArrayTypeDescriptor::ClassType {
                                class_name: "java/lang/Thread".to_string(),
                            },
                        }),
                        JvmInstruction(Astore1),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Aload1),
                        JvmInstruction(Instanceof {
                            check_type: ClassOrArrayTypeDescriptor::ArrayType {
                                component_type: Box::new(ClassOrArrayTypeDescriptor::ClassType {
                                    class_name: "java/lang/Thread".to_string(),
                                }),
                            },
                        }),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/CreateArrayOfThreads.pho")?;
    assert_eq!(expected_ast, actual_ast);
    Ok(())
}

#[test]
fn test_parse_swap_top_two_items() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "SwapTopTwoItems".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(1)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Iconst1),
                        JvmInstruction(Iconst2),
                        JvmInstruction(Swap),
                        JvmInstruction(Pop),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };
    let actual_ast = parse("doc/grammar/SwapTopTwoItems.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_create_matrix_of_int() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "CreateMatrixOfInt".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(5)),
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(Bipush(2)),
                        JvmInstruction(Bipush(3)),
                        JvmInstruction(Bipush(7)),
                        JvmInstruction(Multianewarray {
                            component_type: ArrayType {
                                component_type: Box::new(ArrayType {
                                    component_type: Box::new(ArrayType {
                                        component_type: Box::new(BaseType(Integer)),
                                    }),
                                }),
                            },
                            dimensions: 3,
                        }),
                        JvmInstruction(Astore1),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Aload1),
                        JvmInstruction(Instanceof {
                            check_type: ClassOrArrayTypeDescriptor::ArrayType {
                                component_type: Box::new(ClassOrArrayTypeDescriptor::ArrayType {
                                    component_type: Box::new(
                                        ClassOrArrayTypeDescriptor::ArrayType {
                                            component_type: Box::new(
                                                ClassOrArrayTypeDescriptor::ClassType {
                                                    class_name: "I".to_string(),
                                                },
                                            ),
                                        },
                                    ),
                                }),
                            },
                        }),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/CreateMatrixOfInt.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_count() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "Count".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(4)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Astore1),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Istore2),
                        PhoronLabel("Loop".to_string()),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Iload2),
                        JvmInstruction(Isub),
                        JvmInstruction(Invokestatic {
                            class_name: "java/lang/String".to_string(),
                            method_name: "valueOf".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: FieldDescriptor(ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }),
                            },
                        }),
                        JvmInstruction(Astore3),
                        JvmInstruction(Aload1),
                        JvmInstruction(Aload3),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Iinc {
                            varnum: 2,
                            delta: -1,
                        }),
                        JvmInstruction(Iload2),
                        JvmInstruction(Ifne {
                            label: "Loop".to_string(),
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/Count.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_catcher() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "Catcher".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(3)),
                        PhoronDirective(Catch {
                            class_name: "java/lang/Exception".to_string(),
                            from_label: "Label1".to_string(),
                            to_label: "Label2".to_string(),
                            handler_label: "Handler".to_string(),
                        }),
                        PhoronLabel("Label1".to_string()),
                        JvmInstruction(New {
                            class_name: "java/lang/Exception".to_string(),
                        }),
                        JvmInstruction(Dup),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Exception".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Athrow),
                        PhoronLabel("Label2".to_string()),
                        PhoronLabel("Handler".to_string()),
                        JvmInstruction(Pop),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Exception Caught".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/Catcher.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_anewarray() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "Anewarray".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(4)),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Anewarray {
                            component_type: ClassOrArrayTypeDescriptor::ClassType {
                                class_name: "java/lang/Thread".to_string(),
                            },
                        }),
                        JvmInstruction(Astore1),
                        JvmInstruction(Iconst2),
                        JvmInstruction(Anewarray {
                            component_type: ClassOrArrayTypeDescriptor::ArrayType {
                                component_type: Box::new(ClassOrArrayTypeDescriptor::ClassType {
                                    class_name: "java/lang/String".to_string(),
                                }),
                            },
                        }),
                        JvmInstruction(Astore2),
                        JvmInstruction(Aload2),
                        JvmInstruction(Iconst0),
                        JvmInstruction(Bipush(5)),
                        JvmInstruction(Anewarray {
                            component_type: ClassOrArrayTypeDescriptor::ClassType {
                                class_name: "java/lang/String".to_string(),
                            },
                        }),
                        JvmInstruction(Aastore),
                        JvmInstruction(Aload2),
                        JvmInstruction(Iconst1),
                        JvmInstruction(Bipush(5)),
                        JvmInstruction(Anewarray {
                            component_type: ClassOrArrayTypeDescriptor::ClassType {
                                class_name: "java/lang/String".to_string(),
                            },
                        }),
                        JvmInstruction(Aastore),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/Anewarray.pho")?;
    assert_eq!(expected_ast, actual_ast);
    Ok(())
}

#[test]
fn test_parse_args_to_main() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "ArgsToMain".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(1)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Aload0),
                        JvmInstruction(Arraylength),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/ArgsToMain.pho")?;
    assert_eq!(expected_ast, actual_ast);
    Ok(())
}

#[test]
fn test_parse_count_jasmin2() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "CountJasmin2".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(4)),
                        JvmInstruction(Iconst0),
                        JvmInstruction(Istore1),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Astore2),
                        PhoronLabel("loop".to_string()),
                        JvmInstruction(Aload2),
                        JvmInstruction(Iload1),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Iinc {
                            varnum: 1,
                            delta: 1,
                        }),
                        JvmInstruction(Iload { varnum: 1 }),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Ificmplt {
                            label: "loop".to_string(),
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };
    let actual_ast = parse("doc/grammar/CountJasmin2.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_add_nums_jasmin() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "AddNumsJasmin".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "addNums".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPrivate,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![BaseType(Integer)],
                        return_descriptor: FieldDescriptor(BaseType(Integer)),
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(3)),
                        JvmInstruction(Iload0),
                        JvmInstruction(Iload1),
                        JvmInstruction(Iadd),
                        JvmInstruction(Ireturn),
                    ],
                },
                PhoronMethodDef {
                    name: "main".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPublic,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(1)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Iconst1),
                        JvmInstruction(Bipush(99)),
                        JvmInstruction(Invokestatic {
                            class_name: "AddNumsJasmin".to_string(),
                            method_name: "addNums".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: FieldDescriptor(BaseType(Integer)),
                            },
                        }),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/AddNumsJasmin.pho")?;
    assert_eq!(expected_ast, actual_ast);
    Ok(())
}

#[test]
fn test_parse_stack_push_jasmin() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "StackPushJasmin".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(40)),
                        PhoronDirective(LimitLocals(1)),
                        JvmInstruction(Iconstm1),
                        JvmInstruction(Iconst0),
                        JvmInstruction(Iconst1),
                        JvmInstruction(Iconst2),
                        JvmInstruction(Iconst3),
                        JvmInstruction(Iconst4),
                        JvmInstruction(Iconst5),
                        JvmInstruction(Lconst0),
                        JvmInstruction(Lconst1),
                        JvmInstruction(Fconst0),
                        JvmInstruction(Fconst1),
                        JvmInstruction(Dconst0),
                        JvmInstruction(Dconst1),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Sipush(1000)),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Hello, world".to_string()))),
                        JvmInstruction(Ldcw(LdcValue::QuotedString("Hola, mundo".to_string()))),
                        JvmInstruction(Ldc2w(Ldc2wValue::Long(12345))),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/StackPushJasmin.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_factorial_goto() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "FactorialGoto".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "factorial".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPrivate,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![BaseType(Integer)],
                        return_descriptor: FieldDescriptor(BaseType(Integer)),
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(3)),
                        JvmInstruction(Iconst1),
                        JvmInstruction(Istore1),
                        JvmInstruction(Iconst2),
                        JvmInstruction(Istore2),
                        PhoronLabel("floop".to_string()),
                        JvmInstruction(Iload2),
                        JvmInstruction(Iload0),
                        JvmInstruction(Ificmpgt {
                            label: "back".to_string(),
                        }),
                        JvmInstruction(Iload1),
                        JvmInstruction(Iload2),
                        JvmInstruction(Imul),
                        JvmInstruction(Istore1),
                        JvmInstruction(Iinc {
                            varnum: 2,
                            delta: 1,
                        }),
                        JvmInstruction(Goto {
                            label: "floop".to_string(),
                        }),
                        PhoronLabel("back".to_string()),
                        JvmInstruction(Iload1),
                        JvmInstruction(Ireturn),
                    ],
                },
                PhoronMethodDef {
                    name: "main".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPublic,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(1)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Invokestatic {
                            class_name: "FactorialGoto".to_string(),
                            method_name: "factorial".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: FieldDescriptor(BaseType(Integer)),
                            },
                        }),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/FactorialGoto.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_factorial_jasmin() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "FactorialJasmin".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "factorial".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPrivate,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![BaseType(Integer)],
                        return_descriptor: FieldDescriptor(BaseType(Integer)),
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(3)),
                        JvmInstruction(Iconst1),
                        JvmInstruction(Istore1),
                        JvmInstruction(Iconst2),
                        JvmInstruction(Istore2),
                        PhoronLabel("loop".to_string()),
                        JvmInstruction(Iload2),
                        JvmInstruction(Iload1),
                        JvmInstruction(Imul),
                        JvmInstruction(Istore1),
                        JvmInstruction(Iinc {
                            varnum: 2,
                            delta: 1,
                        }),
                        JvmInstruction(Iload2),
                        JvmInstruction(Iload0),
                        JvmInstruction(Ificmple {
                            label: "loop".to_string(),
                        }),
                        JvmInstruction(Iload1),
                        JvmInstruction(Ireturn),
                    ],
                },
                PhoronMethodDef {
                    name: "main".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPublic,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(1)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Invokestatic {
                            class_name: "FactorialJasmin".to_string(),
                            method_name: "factorial".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: FieldDescriptor(BaseType(Integer)),
                            },
                        }),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/FactorialJasmin.pho".to_string())?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_check_array_type() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "CheckArrayType".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(3)),
                        JvmInstruction(Bipush(5)),
                        JvmInstruction(Newarray {
                            component_type: Integer,
                        }),
                        JvmInstruction(Astore1),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Aload1),
                        JvmInstruction(Instanceof {
                            check_type: ClassOrArrayTypeDescriptor::ArrayType {
                                component_type: Box::new(ClassOrArrayTypeDescriptor::ClassType {
                                    class_name: "I".to_string(),
                                }),
                            },
                        }),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Newarray {
                            component_type: Character,
                        }),
                        JvmInstruction(Astore2),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Aload2),
                        JvmInstruction(Instanceof {
                            check_type: ClassOrArrayTypeDescriptor::ArrayType {
                                component_type: Box::new(ClassOrArrayTypeDescriptor::ClassType {
                                    class_name: "I".to_string(),
                                }),
                            },
                        }),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/CheckArrayType.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_print_hello_10_times() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "PrintHello10Times".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(3)),
                        JvmInstruction(Iconst1),
                        JvmInstruction(Istore1),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Astore2),
                        PhoronLabel("loop".to_string()),
                        JvmInstruction(Aload2),
                        JvmInstruction(Iload1),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "print".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Aload2),
                        JvmInstruction(Ldc(LdcValue::QuotedString(" - ".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "print".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Aload2),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Hello".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Iinc {
                            varnum: 1,
                            delta: 1,
                        }),
                        JvmInstruction(Iload1),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Ificmple {
                            label: "loop".to_string(),
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/PrintHello10Times.pho")?;
    assert_eq!(expected_ast, actual_ast);
    Ok(())
}

#[test]
fn test_parse_add_nums() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "AddNums".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(5)),
                        PhoronDirective(LimitLocals(8)),
                        JvmInstruction(New {
                            class_name: "java/util/Scanner".to_string(),
                        }),
                        JvmInstruction(Dup),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "in".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/InputStream".to_string(),
                            },
                        }),
                        JvmInstruction(Invokespecial {
                            class_name: "java/util/Scanner".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/io/InputStream".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Astore1),
                        JvmInstruction(Jsr {
                            label: "ReadNum".to_string(),
                        }),
                        JvmInstruction(Istore3),
                        JvmInstruction(Jsr {
                            label: "ReadNum".to_string(),
                        }),
                        JvmInstruction(Istore { varnum: 4 }),
                        JvmInstruction(Iload3),
                        JvmInstruction(Iload { varnum: 4 }),
                        JvmInstruction(Jsr {
                            label: "AddNum".to_string(),
                        }),
                        JvmInstruction(Istore { varnum: 5 }),
                        JvmInstruction(Iload { varnum: 5 }),
                        JvmInstruction(Jsr {
                            label: "PrintSum".to_string(),
                        }),
                        JvmInstruction(Return),
                        PhoronLabel("PrintSum".to_string()),
                        JvmInstruction(Astore { varnum: 7 }),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Swap),
                        JvmInstruction(Invokestatic {
                            class_name: "java/lang/String".to_string(),
                            method_name: "valueOf".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: FieldDescriptor(ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }),
                            },
                        }),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Ret { varnum: 7 }),
                        PhoronLabel("AddNum".to_string()),
                        JvmInstruction(Astore { varnum: 6 }),
                        JvmInstruction(Iadd),
                        JvmInstruction(Ret { varnum: 6 }),
                        PhoronLabel("ReadNum".to_string()),
                        JvmInstruction(Astore2),
                        JvmInstruction(Aload1),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/util/Scanner".to_string(),
                            method_name: "nextInt".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: FieldDescriptor(BaseType(Integer)),
                            },
                        }),
                        JvmInstruction(Ret { varnum: 2 }),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/AddNums.pho")?;
    assert_eq!(expected_ast, actual_ast);
    Ok(())
}

#[test]
fn test_parse_count_jasmin() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "CountJasmin".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(4)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Astore1),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Istore2),
                        PhoronLabel("loop".to_string()),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Iload2),
                        JvmInstruction(Isub),
                        JvmInstruction(Invokestatic {
                            class_name: "java/lang/String".to_string(),
                            method_name: "valueOf".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: FieldDescriptor(ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }),
                            },
                        }),
                        JvmInstruction(Astore3),
                        JvmInstruction(Aload1),
                        JvmInstruction(Aload3),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Iinc {
                            varnum: 2,
                            delta: -1,
                        }),
                        JvmInstruction(Iload2),
                        JvmInstruction(Ifne {
                            label: "loop".to_string(),
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/CountJasmin.pho")?;
    assert_eq!(expected_ast, actual_ast);
    Ok(())
}

#[test]
fn test_parse_string_buffer_demo() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "StringBufferDemo".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "sbDemo".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPrivate,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ObjectType {
                            class_name: "java/lang/Object".to_string(),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(Aload0),
                        JvmInstruction(Checkcast {
                            cast_type: ClassOrArrayTypeDescriptor::ClassType {
                                class_name: "java/lang/StringBuffer".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Hello, mundo!".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/lang/StringBuffer".to_string(),
                            method_name: "append".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: FieldDescriptor(ObjectType {
                                    class_name: "java/lang/StringBuffer".to_string(),
                                }),
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(New {
                            class_name: "java/lang/StringBuffer".to_string(),
                        }),
                        JvmInstruction(Dup),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/StringBuffer".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Astore1),
                        JvmInstruction(Aload1),
                        JvmInstruction(Invokestatic {
                            class_name: "StringBufferDemo".to_string(),
                            method_name: "sbDemo".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/Object".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Aload1),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/lang/StringBuffer".to_string(),
                            method_name: "toString".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: FieldDescriptor(ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }),
                            },
                        }),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/StringBufferDemo.pho")?;
    assert_eq!(expected_ast, actual_ast);
    Ok(())
}

#[test]
fn test_parse_array_demo() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: None,
            class_or_interface_def: Class(PhoronClassDef {
                name: "ArrayDemo".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Object".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "setArr".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPrivate,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(BaseType(Integer)),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(4)),
                        PhoronDirective(LimitLocals(4)),
                        JvmInstruction(Aload0),
                        JvmInstruction(Iload1),
                        JvmInstruction(Iload2),
                        JvmInstruction(Iastore),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "printArr".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPrivate,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(BaseType(Integer)),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(4)),
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Aload0),
                        JvmInstruction(Iload1),
                        JvmInstruction(Iaload),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
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
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(Bipush(5)),
                        JvmInstruction(Newarray {
                            component_type: Integer,
                        }),
                        JvmInstruction(Astore1),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Aload1),
                        JvmInstruction(Bipush(4)),
                        JvmInstruction(Iaload),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Aload1),
                        JvmInstruction(Bipush(0)),
                        JvmInstruction(Bipush(1)),
                        JvmInstruction(Invokestatic {
                            class_name: "ArrayDemo".to_string(),
                            method_name: "setArr".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ArrayType {
                                    component_type: Box::new(BaseType(Integer)),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Aload1),
                        JvmInstruction(Bipush(1)),
                        JvmInstruction(Bipush(2)),
                        JvmInstruction(Iastore),
                        JvmInstruction(Aload1),
                        JvmInstruction(Bipush(2)),
                        JvmInstruction(Bipush(3)),
                        JvmInstruction(Iastore),
                        JvmInstruction(Aload1),
                        JvmInstruction(Bipush(3)),
                        JvmInstruction(Bipush(4)),
                        JvmInstruction(Iastore),
                        JvmInstruction(Aload1),
                        JvmInstruction(Bipush(4)),
                        JvmInstruction(Bipush(5)),
                        JvmInstruction(Iastore),
                        JvmInstruction(Aload1),
                        JvmInstruction(Bipush(0)),
                        JvmInstruction(Invokestatic {
                            class_name: "ArrayDemo".to_string(),
                            method_name: "printArr".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ArrayType {
                                    component_type: Box::new(BaseType(Integer)),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Aload1),
                        JvmInstruction(Bipush(1)),
                        JvmInstruction(Iaload),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Aload1),
                        JvmInstruction(Bipush(2)),
                        JvmInstruction(Iaload),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Aload1),
                        JvmInstruction(Bipush(3)),
                        JvmInstruction(Iaload),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Aload1),
                        JvmInstruction(Bipush(4)),
                        JvmInstruction(Iaload),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/ArrayDemo.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}

#[test]
fn test_parse_all_in_one() -> Result<(), Box<dyn Error>> {
    let expected_ast = PhoronProgram {
        header: PhoronHeader {
            sourcefile_def: Some(PhoronSourceFileDef {
                source_file: "AllInOne.j".to_string(),
            }),
            class_or_interface_def: Class(PhoronClassDef {
                name: "AllInOne".to_string(),
                access_flags: vec![
                    PhoronClassOrInterfaceAccessFlag::AccSuper,
                    PhoronClassOrInterfaceAccessFlag::AccPublic,
                ],
            }),
            super_def: PhoronSuperDef {
                super_class_name: "java/lang/Thread".to_string(),
            },
        },
        body: PhoronBody {
            field_defs: vec![
                PhoronFieldDef {
                    name: "x".to_string(),
                    access_flags: vec![PhoronFieldAccessFlag::AccPrivate],
                    field_descriptor: BaseType(Integer),
                    init_val: None,
                },
                PhoronFieldDef {
                    name: "y".to_string(),
                    access_flags: vec![PhoronFieldAccessFlag::AccPrivate],
                    field_descriptor: BaseType(Double),
                    init_val: Some(PhoronFieldInitValue::Double(1.2345)),
                },
                PhoronFieldDef {
                    name: "z".to_string(),
                    access_flags: vec![PhoronFieldAccessFlag::AccPublic],
                    field_descriptor: BaseType(Integer),
                    init_val: Some(PhoronFieldInitValue::Integer(12345)),
                },
                PhoronFieldDef {
                    name: "PREFIX".to_string(),
                    access_flags: vec![
                        PhoronFieldAccessFlag::AccPublic,
                        PhoronFieldAccessFlag::AccStatic,
                        PhoronFieldAccessFlag::AccFinal,
                    ],
                    field_descriptor: ObjectType {
                        class_name: "java/lang/String".to_string(),
                    },
                    init_val: Some(PhoronFieldInitValue::QuotedString("FooBar".to_string())),
                },
            ],
            method_defs: vec![
                PhoronMethodDef {
                    name: "<init>".to_string(),
                    access_flags: vec![PhoronMethodAccessFlag::AccPublic],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        JvmInstruction(Aload0),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Thread".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "exceptionsDemo".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPrivate,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(1)),
                        PhoronDirective(Catch {
                            class_name: "java/lang/Exception".to_string(),
                            from_label: "Label1".to_string(),
                            to_label: "Label2".to_string(),
                            handler_label: "Handler".to_string(),
                        }),
                        PhoronLabel("Label1".to_string()),
                        JvmInstruction(New {
                            class_name: "java/lang/Exception".to_string(),
                        }),
                        JvmInstruction(Dup),
                        JvmInstruction(Invokespecial {
                            class_name: "java/lang/Exception".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Athrow),
                        PhoronLabel("Label2".to_string()),
                        PhoronLabel("Handler".to_string()),
                        JvmInstruction(Pop),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Exception caught".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "finallyDemo".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPrivate,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(3)),
                        PhoronDirective(LimitLocals(4)),
                        PhoronDirective(Catch {
                            class_name: "java/io/FileNotFoundException".to_string(),
                            from_label: "Start".to_string(),
                            to_label: "End1".to_string(),
                            handler_label: "NotFound".to_string(),
                        }),
                        PhoronDirective(Catch {
                            class_name: "java/io/IOException".to_string(),
                            from_label: "Start".to_string(),
                            to_label: "End2".to_string(),
                            handler_label: "IOE".to_string(),
                        }),
                        PhoronDirective(Catch {
                            class_name: "all".to_string(),
                            from_label: "Start".to_string(),
                            to_label: "Done".to_string(),
                            handler_label: "Other_Exception".to_string(),
                        }),
                        PhoronLabel("Start".to_string()),
                        JvmInstruction(New {
                            class_name: "java/io/FileInputStream".to_string(),
                        }),
                        JvmInstruction(Dup),
                        JvmInstruction(Ldc(LdcValue::QuotedString("myfile".to_string()))),
                        JvmInstruction(Invokespecial {
                            class_name: "java/io/FileInputStream".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Astore1),
                        PhoronLabel("End1".to_string()),
                        JvmInstruction(Goto {
                            label: "Done".to_string(),
                        }),
                        PhoronLabel("NotFound".to_string()),
                        JvmInstruction(Pop),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString("No such file".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Goto {
                            label: "Done".to_string(),
                        }),
                        PhoronLabel("IOE".to_string()),
                        JvmInstruction(Pop),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString(
                            "IO Exception occurred".to_string(),
                        ))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Goto {
                            label: "Done".to_string(),
                        }),
                        PhoronLabel("End2".to_string()),
                        PhoronLabel("Done".to_string()),
                        JvmInstruction(Jsr {
                            label: "FinalSub".to_string(),
                        }),
                        JvmInstruction(Return),
                        PhoronLabel("Other_Exception".to_string()),
                        JvmInstruction(Astore2),
                        JvmInstruction(Jsr {
                            label: "FinalSub".to_string(),
                        }),
                        JvmInstruction(Aload2),
                        JvmInstruction(Athrow),
                        PhoronLabel("FinalSub".to_string()),
                        JvmInstruction(Astore3),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Done".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Ret { varnum: 3 }),
                    ],
                },
                PhoronMethodDef {
                    name: "synchronizedMethoDemo".to_string(),
                    access_flags: vec![PhoronMethodAccessFlag::AccSynchronized],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(1)),
                        PhoronDirective(LimitLocals(1)),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "monitoDemo".to_string(),
                    access_flags: vec![PhoronMethodAccessFlag::AccPrivate],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ObjectType {
                            class_name: "java/lang/Object".to_string(),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(Aload1),
                        JvmInstruction(Monitorenter),
                        JvmInstruction(Aload1),
                        JvmInstruction(Monitorexit),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "checkCastDemo".to_string(),
                    access_flags: vec![PhoronMethodAccessFlag::AccPrivate],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(Aload0),
                        JvmInstruction(Checkcast {
                            cast_type: ClassOrArrayTypeDescriptor::ClassType {
                                class_name: "java/lang/Object".to_string(),
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "instanceofDemo".to_string(),
                    access_flags: vec![PhoronMethodAccessFlag::AccPrivate],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(Aload0),
                        JvmInstruction(Instanceof {
                            check_type: ClassOrArrayTypeDescriptor::ClassType {
                                class_name: "java/lang/Thread".to_string(),
                            },
                        }),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Swap),
                        JvmInstruction(Invokestatic {
                            class_name: "java/lang/String".to_string(),
                            method_name: "valueOf".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: FieldDescriptor(ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }),
                            },
                        }),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "subroutinesDemo".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPrivate,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Hello".to_string()))),
                        JvmInstruction(Jsr {
                            label: "PrintString".to_string(),
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString(", world".to_string()))),
                        JvmInstruction(Jsr {
                            label: "PrintString".to_string(),
                        }),
                        JvmInstruction(Return),
                        PhoronLabel("PrintString".to_string()),
                        JvmInstruction(Astore1),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Swap),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Ret { varnum: 1 }),
                    ],
                },
                PhoronMethodDef {
                    name: "lookupswitchDemo".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPrivate,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: FieldDescriptor(BaseType(Integer)),
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(2)),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Istore1),
                        JvmInstruction(Iload1),
                        JvmInstruction(Lookupswitch {
                            switches: vec![
                                LookupSwitchPair {
                                    key: 1,
                                    label: "R1".to_string(),
                                },
                                LookupSwitchPair {
                                    key: 10,
                                    label: "R2".to_string(),
                                },
                                LookupSwitchPair {
                                    key: 100,
                                    label: "R3".to_string(),
                                },
                            ],
                            default: "R4".to_string(),
                        }),
                        PhoronLabel("R1".to_string()),
                        JvmInstruction(Iconst1),
                        JvmInstruction(Ireturn),
                        PhoronLabel("R2".to_string()),
                        JvmInstruction(Iconst2),
                        JvmInstruction(Ireturn),
                        PhoronLabel("R3".to_string()),
                        JvmInstruction(Iconst3),
                        JvmInstruction(Ireturn),
                        PhoronLabel("R4".to_string()),
                        JvmInstruction(Iconst0),
                        JvmInstruction(Ireturn),
                    ],
                },
                PhoronMethodDef {
                    name: "tableswitchDemo".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPrivate,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: FieldDescriptor(BaseType(Integer)),
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(3)),
                        JvmInstruction(Iconst3),
                        JvmInstruction(Istore1),
                        JvmInstruction(Iload1),
                        JvmInstruction(Tableswitch {
                            low: 1,
                            high: 3,
                            switches: vec!["R1".to_string(), "R2".to_string(), "R3".to_string()],
                            default: "R4".to_string(),
                        }),
                        PhoronLabel("R1".to_string()),
                        JvmInstruction(Iconst1),
                        JvmInstruction(Ireturn),
                        PhoronLabel("R2".to_string()),
                        JvmInstruction(Iconst2),
                        JvmInstruction(Ireturn),
                        PhoronLabel("R3".to_string()),
                        JvmInstruction(Iconst3),
                        JvmInstruction(Ireturn),
                        PhoronLabel("R4".to_string()),
                        JvmInstruction(Iconst0),
                        JvmInstruction(Ireturn),
                    ],
                },
                PhoronMethodDef {
                    name: "varDemo".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPrivate,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitLocals(1)),
                        PhoronDirective(Var {
                            varnum: 0,
                            name: "Count".to_string(),
                            field_descriptor: BaseType(Integer),
                            from_label: "Label1".to_string(),
                            to_label: "Label2".to_string(),
                        }),
                        PhoronLabel("Label1".to_string()),
                        JvmInstruction(Bipush(10)),
                        JvmInstruction(Istore0),
                        PhoronLabel("Label2".to_string()),
                        JvmInstruction(Return),
                    ],
                },
                PhoronMethodDef {
                    name: "main".to_string(),
                    access_flags: vec![
                        PhoronMethodAccessFlag::AccPublic,
                        PhoronMethodAccessFlag::AccStatic,
                    ],
                    method_descriptor: PhoronMethodDescriptor {
                        param_descriptor: vec![ArrayType {
                            component_type: Box::new(ObjectType {
                                class_name: "java/lang/String".to_string(),
                            }),
                        }],
                        return_descriptor: VoidDescriptor,
                    },
                    instructions: vec![
                        PhoronDirective(LimitStack(2)),
                        PhoronDirective(LimitLocals(3)),
                        PhoronDirective(Throws {
                            class_name: "java/lang/RuntimeException".to_string(),
                        }),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Ldc(LdcValue::QuotedString("Hello, world".to_string()))),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Invokestatic {
                            class_name: "AllInOne".to_string(),
                            method_name: "exceptionsDemo".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Invokestatic {
                            class_name: "AllInOne".to_string(),
                            method_name: "finallyDemo".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(New {
                            class_name: "AllInOne".to_string(),
                        }),
                        JvmInstruction(Dup),
                        JvmInstruction(Invokespecial {
                            class_name: "AllInOne".to_string(),
                            method_name: "<init>".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Astore1),
                        JvmInstruction(Aload1),
                        JvmInstruction(Invokevirtual {
                            class_name: "AllInOne".to_string(),
                            method_name: "instanceofDemo".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Aload1),
                        JvmInstruction(Invokevirtual {
                            class_name: "AllInOne".to_string(),
                            method_name: "checkCastDemo".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Invokestatic {
                            class_name: "AllInOne".to_string(),
                            method_name: "subroutinesDemo".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Invokestatic {
                            class_name: "AllInOne".to_string(),
                            method_name: "lookupswitchDemo".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: FieldDescriptor(BaseType(Integer)),
                            },
                        }),
                        JvmInstruction(Jsr {
                            label: "PrintInt".to_string(),
                        }),
                        JvmInstruction(Invokestatic {
                            class_name: "AllInOne".to_string(),
                            method_name: "tableswitchDemo".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![],
                                return_descriptor: FieldDescriptor(BaseType(Integer)),
                            },
                        }),
                        JvmInstruction(Jsr {
                            label: "PrintInt".to_string(),
                        }),
                        JvmInstruction(Return),
                        PhoronLabel("PrintInt".to_string()),
                        JvmInstruction(Astore2),
                        JvmInstruction(Getstatic {
                            class_name: "java/lang/System".to_string(),
                            field_name: "out".to_string(),
                            field_descriptor: ObjectType {
                                class_name: "java/io/PrintStream".to_string(),
                            },
                        }),
                        JvmInstruction(Swap),
                        JvmInstruction(Invokestatic {
                            class_name: "java/lang/String".to_string(),
                            method_name: "valueOf".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![BaseType(Integer)],
                                return_descriptor: FieldDescriptor(ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }),
                            },
                        }),
                        JvmInstruction(Invokevirtual {
                            class_name: "java/io/PrintStream".to_string(),
                            method_name: "println".to_string(),
                            method_descriptor: PhoronMethodDescriptor {
                                param_descriptor: vec![ObjectType {
                                    class_name: "java/lang/String".to_string(),
                                }],
                                return_descriptor: VoidDescriptor,
                            },
                        }),
                        JvmInstruction(Ret { varnum: 2 }),
                    ],
                },
            ],
        },
    };

    let actual_ast = parse("doc/grammar/AllInOne.pho")?;
    assert_eq!(expected_ast, actual_ast);

    Ok(())
}