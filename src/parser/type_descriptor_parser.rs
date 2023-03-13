//! A parser for type descriptors as per the JVM specification

use crate::ast::{PhoronBaseType, PhoronFieldDescriptor, PhoronReturnDescriptor};
use std::{error::Error, fmt, iter::Peekable, str::Chars};

pub type TypeParseResult<T> = Result<T, Box<dyn Error + Send + Sync + 'static>>;

#[derive(Debug)]
pub enum TypeParseError {
    Malformed { details: &'static str },
    Missing { details: &'static str },
}

impl Error for TypeParseError {}

impl fmt::Display for TypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TypeParseError::*;

        write!(
            f,
            "{}",
            match *self {
                Malformed { ref details } => format!("{details}"),
                Missing { ref details } => format!("{details}"),
            }
        )
    }
}

pub struct TypeParser<'p> {
    src: Peekable<Chars<'p>>,
}

impl<'p> TypeParser<'p> {
    pub fn new(src: &'p str) -> Self {
        TypeParser {
            src: src.chars().peekable(),
        }
    }

    fn peek(&mut self) -> TypeParseResult<Option<&char>> {
        Ok(self.src.peek())
    }

    fn advance(&mut self) -> TypeParseResult<char> {
        Ok(self.src.next().ok_or(TypeParseError::Missing {
            details: "no more characters",
        })?)
    }

    // FieldDescriptor <- FieldType
    // FieldType <- BaseType / ObjectType / ArrayType
    // BaseType <- 'B' / 'C' / 'D' / 'F' / 'I' / 'J' / 'S' / 'Z'
    // ObjectType <- 'L' ClassName ';'
    // ArrayType <- '[' ComponentType
    // ComponentType <- FieldType
    pub fn parse_field_descriptor(&mut self) -> TypeParseResult<PhoronFieldDescriptor> {
        match self.peek()?.ok_or(TypeParseError::Missing {
            details: "out of characters",
        })? {
            'B' => {
                self.advance()?;
                Ok(PhoronFieldDescriptor::BaseType(PhoronBaseType::Byte))
            }

            'C' => {
                self.advance()?;
                Ok(PhoronFieldDescriptor::BaseType(PhoronBaseType::Character))
            }

            'D' => {
                self.advance()?;
                Ok(PhoronFieldDescriptor::BaseType(PhoronBaseType::Double))
            }

            'F' => {
                self.advance()?;
                Ok(PhoronFieldDescriptor::BaseType(PhoronBaseType::Float))
            }

            'I' => {
                self.advance()?;
                Ok(PhoronFieldDescriptor::BaseType(PhoronBaseType::Integer))
            }

            'J' => {
                self.advance()?;
                Ok(PhoronFieldDescriptor::BaseType(PhoronBaseType::Long))
            }

            'S' => {
                self.advance()?;
                Ok(PhoronFieldDescriptor::BaseType(PhoronBaseType::Short))
            }

            'Z' => {
                self.advance()?;
                Ok(PhoronFieldDescriptor::BaseType(PhoronBaseType::Boolean))
            }

            'L' => {
                self.advance()?;

                let mut classname = String::new();
                let mut semicolon_found = false;

                while self.peek()?.is_some() {
                    if let Some(';') = self.peek()? {
                        self.advance()?;
                        semicolon_found = true;
                        break;
                    } else {
                        classname.push(self.advance()?);
                    }
                }

                if !semicolon_found {
                    return Err(Box::new(TypeParseError::Malformed {
                        details: "missing ; for class type in fied descriptor",
                    }));
                }

                Ok(PhoronFieldDescriptor::ObjectType {
                    class_name: classname,
                })
            }

            '[' => {
                self.advance()?;
                let component_type = self.parse_field_descriptor().map_err(|_| {
                    Box::new(TypeParseError::Missing {
                        details: "component type for array type in field descriptor",
                    })
                })?;

                Ok(PhoronFieldDescriptor::ArrayType {
                    component_type: Box::new(component_type),
                })
            }
            _ => {
                // assume class name here
                let mut class_name = String::new();
                while let Some(c) = self.peek()? {
                    class_name.push(*c);
                    self.advance()?;
                }

                Ok(PhoronFieldDescriptor::ObjectType { class_name })
            }
        }
    }

    // ReturnDescriptor <- FieldType / VoidType
    // VoidType <- 'V'
    pub fn parse_return_descriptor(&mut self) -> TypeParseResult<PhoronReturnDescriptor> {
        match self.peek()?.ok_or(TypeParseError::Missing {
            details: "out of characters",
        })? {
            'V' => {
                self.advance()?;
                Ok(PhoronReturnDescriptor::VoidDescriptor)
            }

            _ => Ok(PhoronReturnDescriptor::FieldDescriptor(
                self.parse_field_descriptor()?,
            )),
        }
    }

    // ParameterDescriptor <- PhoronFieldDescriptor*
    pub fn parse_param_descriptor(&mut self) -> TypeParseResult<Vec<PhoronFieldDescriptor>> {
        let mut param_desc = Vec::new();

        while let Ok(field_desc) = self.parse_field_descriptor() {
            param_desc.push(field_desc);
        }

        Ok(param_desc)
    }
}
