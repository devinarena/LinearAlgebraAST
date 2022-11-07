mod lexer;
mod tokens;

fn main() {
    let lexer = lexer::Lexer::new("file.m");
    let tokens = lexer.scan_tokens();
    for token in tokens {
        println!("{}", token);
    }
}
