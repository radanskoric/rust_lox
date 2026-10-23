#[derive(Debug)]
pub enum OpCode {
    Return,
}

pub struct Chunk {
    pub op_codes: Vec<OpCode>,
}
