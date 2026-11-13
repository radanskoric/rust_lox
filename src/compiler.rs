use crate::chunk::Chunk;
use crate::scanner::Scanner;

pub struct CompilationError;

pub fn compile(source: &str) -> Result<Chunk<'static>, CompilationError> {
    let mut scanner = Scanner::new(source);
    let mut tokens = Vec::new();
    while let Some(token) = scanner.scan_token() {
        tokens.push(token);
    }
    Ok(Chunk::new("compiled chunk"))
}
