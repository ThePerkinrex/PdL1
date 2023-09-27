use std::io::Cursor;

use crate::{lexer::Lexer, error::ErrorManager};

mod lexer;
mod error;

fn main() {
    println!("Hello, world! {:0>8X}", '💯' as u32);
    let mut reader = Lexer::new(Cursor::new(include_str!("test.txt"))).unwrap();
    let mut error_mgr = ErrorManager::new();
    loop {
        match reader.next_token(&mut error_mgr) {
            Ok(Some(t)) => println!("TOKEN: {t:?}"),
            Err(_) => println!("Error"),
            Ok(None) => break
        }
    }
    error_mgr.print_errors();
}
