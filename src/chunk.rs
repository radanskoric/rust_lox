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

    pub fn read_constant(&self, index: usize) -> Value {
        self.values.0[index]
    }

    pub fn disassemble(&self, output: &mut impl std::io::Write) {
        writeln!(output, "== {} ==", self.name).expect("failed to write chunk header");
        for (ip, op_code) in self.op_codes.iter().enumerate() {
            self.disassemble_op_code(ip, op_code, output);
        }
    }

    #[cfg(any(feature = "debug-trace-execution", test))]
    pub fn disassemble_instruction(&self, ip: usize, output: &mut impl std::io::Write) {
        let op_code = self
            .op_codes
            .get(ip)
            .expect(&format!("Invalid ip provided: {}", ip));
        self.disassemble_op_code(ip, op_code, output);
    }

    fn disassemble_op_code(&self, ip: usize, op_code: &OpCode, output: &mut impl std::io::Write) {
        write!(output, "{:04} ", ip).expect("failed to write op code index");
        let line = self.line_numbers[ip];
        let last_line = if ip == 0 {
            0
        } else {
            self.line_numbers[ip - 1]
        };
        if ip == 0 || line != last_line {
            write!(output, "{:4} ", line).expect("failed to write line number");
        } else {
            write!(output, "   | ").expect("failed to write line number");
        }
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

    #[test]
    fn disassembles_a_single_instruction() {
        let mut chunk = Chunk::new("test chunk");
        chunk.write(OpCode::Return, 123);
        let mut output = Vec::new();
        chunk.disassemble_instruction(0, &mut output);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            concat!("0000  123 OP_RETURN\n")
        );
    }
}
