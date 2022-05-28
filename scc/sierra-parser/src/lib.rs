#![feature(box_syntax, box_patterns)]
#![feature(let_chains)]
pub mod lexer;
pub mod common;
pub mod parser;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(test)]
mod tests {
}
