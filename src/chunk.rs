use crate::value::Value;
use crate::value::ValueArray;

#[derive(Debug, PartialEq)]
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

    pub fn get(&self, index: usize) -> Option<&OpCode> {
        self.op_codes.get(index)
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.values.0.push(value);
        self.values.0.len() - 1
    }

    pub fn read_constant(&self, index: usize) -> &Value {
        &self.values.0[index]
    }

    pub fn disassemble(&self, output: &mut impl std::io::Write) {
        writeln!(output, "== {} ==", self.name).expect("failed to write chunk header");
        let mut last_line = 0;
        for (i, op_code) in self.op_codes.iter().enumerate() {
            write!(output, "{:04} ", i).expect("failed to write op code index");
            let line = self.line_numbers[i];
            if i == 0 || line != last_line {
                write!(output, "{:4} ", line).expect("failed to write line number");
            } else {
                write!(output, "   | ").expect("failed to write line number");
            }
            last_line = line;
            match op_code {
                OpCode::Constant(constant) => writeln!(
                    output,
                    "OP_CONSTANT {} => {}",
                    constant,
                    self.read_constant(*constant)
                )
                .expect("failed to write constant op code"),
                _ => writeln!(output, "OP_{}", format!("{:?}", op_code).to_uppercase())
                    .expect("failed to write op code"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Chunk;
    use super::OpCode;
    use super::Value;

    #[test]
    fn get_returns_op_code_at_index() {
        let mut chunk = Chunk::new("test chunk");
        let const_index = chunk.add_constant(Value(1.2));
        chunk.write(OpCode::Constant(const_index), 123);
        chunk.write(OpCode::Return, 123);
        let value = chunk.get(0);
        assert_eq!(*value.unwrap(), OpCode::Constant(const_index));
        let value = chunk.get(1);
        assert_eq!(*value.unwrap(), OpCode::Return);
    }

    #[test]
    fn disassembles_chunk_with_constant() {
        let mut chunk = Chunk::new("test chunk");
        let const_index = chunk.add_constant(Value(1.2));
        chunk.write(OpCode::Constant(const_index), 123);
        chunk.write(OpCode::Return, 123);
        let mut output = Vec::new();
        chunk.disassemble(&mut output);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            concat!(
                "== test chunk ==\n",
                "0000  123 OP_CONSTANT 0 => 1.2\n",
                "0001    | OP_RETURN\n"
            )
        );
    }
}
