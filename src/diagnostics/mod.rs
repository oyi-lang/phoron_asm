use std::error::Error;

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

    pub fn failfast<'d>(err: Box<dyn Error>) {
        eprintln!("{}", err);
        std::process::exit(1);
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
