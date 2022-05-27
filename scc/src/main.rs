use sierra_parser::lexer::*;

fn main() {
    let mut lexer = Lexer::new("test 93 test34 444_44_44");

    for i in lexer.lex() {
        print!("{i:?}");
    }

    println!();
}
