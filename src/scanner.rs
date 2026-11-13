pub(crate) struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

pub struct Token;

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Option<Token> {
        None
    }
}
