use crate::value::Value;
use crate::value::ValueArray;

#[derive(Debug)]
pub enum OpCode {
    Return,
    Constant(usize),
}

pub struct Chunk<'a> {
    name: &'a str,
    op_codes: Vec<OpCode>,
    values: ValueArray,
}

impl<'a> Chunk<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            op_codes: Vec::new(),
            values: ValueArray(Vec::new()),
        }
    }

    pub fn write(&mut self, op_code: OpCode) {
        self.op_codes.push(op_code);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.values.0.push(value);
        self.values.0.len() - 1
    }

    pub fn read_constant(&self, index: usize) -> &Value {
        &self.values.0[index]
    }

    pub fn disassemble(&self) {
        println!("== {} ==", self.name);
        for (i, op_code) in self.op_codes.iter().enumerate() {
            match op_code {
                OpCode::Constant(constant) => println!(
                    "{:04} OP_CONSTANT {} => {}",
                    i,
                    constant,
                    self.read_constant(*constant)
                ),
                _ => println!("{:04} OP_{}", i, format!("{:?}", op_code).to_uppercase()),
            }
        }
    }
}
