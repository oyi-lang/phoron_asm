//! `Phoron` is a [Jasmin](https://jasmin.sourceforge.net/) compatible assembler for the JVM.
//!
//! `phoron_asm` provides the actual assembler for the assembly language whereas `phoron_core`
//! handles the low-level `class` file serialisation and deserialisation.
//!

pub mod ast;
pub mod codegen;
pub mod cp_analyzer;
pub mod diagnostics;
pub mod lexer;
pub mod parser;
pub mod sourcefile;
