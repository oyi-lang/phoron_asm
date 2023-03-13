pub struct Diagnostic {
    src_file: String,
    line: u32,
    col: u32,
    stage: Stage,
    level: Level,
    src: String,
}

pub enum Level {
    Info,
    Warning,
    Error,
}

pub enum Stage {
    Lexer,
    Parser,
    ConstantPoolAnalyzer,
    CodeGenerator,
}
