//! A simple diagnostic module for Phoron.
//!
//! Provides formatted error reporting via the `emitter` submodule as well as a fail-fast version
//! that exits immediately.
use crate::sourcefile::{Location, SourceFile, Span};
use std::error::Error;

mod emitter;
use emitter::Emitter;

#[derive(Debug)]
pub struct Diagnostic<'d> {
    src_file: &'d str,
    line: usize,
    col: usize,
    src_line: &'d str,
    diag_str: String,
}

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

    pub fn failfast<'d>(err: impl Error) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
