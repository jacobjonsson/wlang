use css_lexer_next::{Lexer, Token};
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let mut file = File::open("test.css")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let now = Instant::now();
    let mut lexer = Lexer::new(&contents);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next();
        if token == Token::EndOfFile {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    println!(
        "Produced {} tokens in: {} milliseconds",
        tokens.len(),
        now.elapsed().as_millis()
    );

    Ok(())
}
