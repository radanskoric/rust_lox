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
    line_numbers: Vec<usize>,
    values: ValueArray,
}

impl<'a> Chunk<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            op_codes: Vec::new(),
            line_numbers: Vec::new(),
            values: ValueArray(Vec::new()),
        }
    }

    pub fn write(&mut self, op_code: OpCode, line_number: usize) {
        self.op_codes.push(op_code);
        self.line_numbers.push(line_number);
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
        let mut last_line = 0;
        for (i, op_code) in self.op_codes.iter().enumerate() {
            print!("{:04} ", i);
            let line = self.line_numbers[i];
            if i == 0 || line != last_line {
                print!("{:4} ", line);
            } else {
                print!("   | ");
            }
            last_line = line;
            match op_code {
                OpCode::Constant(constant) => println!(
                    "OP_CONSTANT {} => {}",
                    constant,
                    self.read_constant(*constant)
                ),
                _ => println!("OP_{}", format!("{:?}", op_code).to_uppercase()),
            }
        }
    }
}
