use std::fmt;

use crate::sourcefile::{Location, SourceFile, Span};

mod emitter;

use emitter::Emitter;

pub struct DiagnosticManager;

impl DiagnosticManager {
    pub fn report_diagnostic<'d>(source_file: &SourceFile, span: Span, text: String) {
        let Location {
            src_file,
            line,
            col,
        } = span.location(&source_file);

        let src_line = span.source_line(&source_file);

        let diag = Diagnostic {
            src_file,
            line,
            col,
            src_line,
            diag_str: text,
        };

        Emitter::emit(&diag);
    }
}

#[derive(Debug)]
pub struct Diagnostic<'d> {
    src_file: &'d str,
    line: usize,
    col: usize,
    src_line: &'d str,
    diag_str: String,
}

#[derive(Debug, PartialEq)]
pub enum Level {
    Info,
    Warning,
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Level::Info => "info",
                Level::Warning => "warning",
                Level::Error => "error",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Stage {
    Lexer,
    Parser,
    ConstantPoolAnalyzer,
    CodeGenerator,
}
