pub mod ast;
pub mod lexer;
pub mod tokens;

fn main() {
    println!("Hello, world!");

    let rs = lexer::lex("1 - 8 + 5 * 32 / 4 - 3");
    print!("{:?}", rs);
}
