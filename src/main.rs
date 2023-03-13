use phoron_asm::{
    codegen::{Codegen, CodegenError},
    cp_analyzer::{ConstantPoolAnalyzer, ConstantPoolAnalyzerError},
    lexer::{Lexer, LexerError, Token, TokenKind},
    sourcefile::{Pos, SourceFile},
};
use std::{
    convert::From,
    error::Error,
    fmt, fs,
    io::{self, BufWriter},
    path::{Path, PathBuf},
};

const PHORON_VERSION: &'static str = "0.1.0";
const USAGE_STR: &'static str = r#"usage: phoron [-d <outpath>] -f <file> [<file> ...]
        or: phoron -v"#;

#[derive(Debug)]
pub enum PhoronError {
    Error { details: &'static str },
    ExtError(Box<dyn Error>),
}

impl Error for PhoronError {}

impl fmt::Display for PhoronError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PhoronError::*;
        write!(
            f,
            "{}",
            match *self {
                Error { ref details } => details.to_string(),
                ExtError(ref boxed_err) => boxed_err.to_string(),
            }
        )
    }
}

macro_rules! impl_from_err {
    ($err:ty) => {
        impl From<$err> for PhoronError {
            fn from(err: $err) -> Self {
                PhoronError::ExtError(Box::new(err))
            }
        }
    };
}

impl_from_err!(io::Error);
impl_from_err!(CodegenError);
impl_from_err!(ConstantPoolAnalyzerError);
impl_from_err!(LexerError);
//impl_from_err!(ParserError);

pub type PhoronResult<T> = Result<T, PhoronError>;

fn usage() {
    eprintln!("{USAGE_STR}");
    std::process::exit(0);
}

fn process_file(output_dir: &Path, src_file: &PathBuf) -> PhoronResult<()> {
    let out_file = output_dir.join(src_file.with_extension(".class"));

    let src = fs::read_to_string(src_file)?;
    let mut beginnings = vec![Pos(1)];
    beginnings.extend_from_slice(
        &src.match_indices("\n")
            .map(|(idx, _)| Pos(idx as u32 + 1))
            .collect::<Vec<_>>(),
    );

    let source_file = SourceFile {
        src_file: src_file.to_str().expect("no source file"),
        src,
        beginnings,
    };

    let mut lexer = Lexer::new(&source_file);
    loop {
        let tok = lexer.lex()?;
        println!("{tok:?}");

        if tok.kind == TokenKind::TEof {
            break;
        }
    }

    //let mut parser = Parser::new(Lexer::new(&source_file));
    //let ast = parser.parse()?;

    //println!("{ast:#?}");

    //let mut cp_analyzer = ConstantPoolAnalyzer::new();
    //let cp = cp_analyzer.analyze(&ast)?;

    //let mut outfile_w = BufWriter::new(fs::File::create(&outfile)?);
    //let mut codegen = Codegen::new(&mut outfile_w);
    //codegen.gen_bytecode(&ast, &cp)?;

    //println!("Generated {}", outfile.display());

    Ok(())
}

fn process_files(output_dir: &Path, srcfiles: &[PathBuf]) -> PhoronResult<()> {
    srcfiles
        .iter()
        .try_for_each(|srcfile| process_file(output_dir, srcfile))?;
    Ok(())
}

/// The entrypoint for Phoron
fn main() -> PhoronResult<()> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() == 0 {
        usage();
    } else {
        match args[0].as_str() {
            "-v" => println!("phoron {PHORON_VERSION}"),
            "-d" => {
                if args.len() < 2 {
                    eprintln!("missing output path");
                    usage();
                } else if args.len() < 4 {
                    eprintln!("missing source file(s)");
                    usage();
                } else {
                    let output_dir = &args[1];
                    if args[2] != "-f" {
                        usage();
                    } else {
                        let mut srcfiles = Vec::new();
                        for arg in &args[3..] {
                            let mut srcfile = PathBuf::new();
                            srcfile.push(arg);
                            srcfiles.push(srcfile);
                        }

                        if !Path::new(output_dir).exists() {
                            fs::create_dir(output_dir)?;
                        }
                        process_files(Path::new(output_dir), &srcfiles)?;
                    }
                }
            }

            "-f" => {
                if args.len() < 2 {
                    usage();
                } else {
                    let mut srcfiles = Vec::new();
                    for arg in &args[1..] {
                        let mut srcfile = PathBuf::new();
                        srcfile.push(arg);
                        srcfiles.push(srcfile);
                    }

                    process_files(Path::new("."), &srcfiles)?;
                }
            }

            invalid => {
                eprintln!("Invalid switch: {invalid}",);
                usage();
            }
        }
    }

    Ok(())
}
