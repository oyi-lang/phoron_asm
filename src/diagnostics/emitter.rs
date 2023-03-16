use super::Diagnostic;

pub struct Emitter;

const RED: &'static str = "\u{0001b}[31m";
const BLUE: &'static str = "\u{001b}[34m";
const RESET: &'static str = "\u{001b}[0m";

//  ```
//    <File:Line:Col>: <Error Text>
//              |
//   <LINE NUM> | <SOURCE CODE LINE> generated from the span
//              |          ^^^^
//
//  ```
//
impl Emitter {
    pub fn emit(diagnostic: &Diagnostic) {
        let Diagnostic {
            src_file,
            line,
            col,
            src_line,
            diag_str,
        } = diagnostic;

        println!("{RED}error{RESET}: {diag_str}");
        println!("{BLUE}--->{RESET} {src_file}:{line}:{col}");
        println!("{BLUE}{:>10}{RESET}", "|");
        println!("{BLUE}{:>5}{:>5}{RESET}{:>10}", line, "|", src_line.trim());
        print!("{BLUE}{:>10}{RESET}", "|");
        for _ in 0..*col {
            print!("{}", " ");
        }
        println!("{RED}^{RESET}");
        println!("{BLUE}{:>10}{RESET}", "|");
    }
}
