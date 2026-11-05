use crate::chunk::Chunk;
use crate::chunk::OpCode;

pub struct VM<'a> {
    chunk: Option<&'a Chunk<'a>>,
    ip: usize,
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self { chunk: None, ip: 0 }
    }

    pub fn interpret(
        mut self,
        chunk: &'a Chunk<'a>,
        output: &mut impl std::io::Write,
    ) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run(output)
    }

    fn run(&mut self, output: &mut impl std::io::Write) -> InterpretResult {
        let chunk = self.chunk.as_ref().unwrap();
        loop {
            match chunk.get(self.ip) {
                Some(op) => {
                    #[cfg(feature = "debug-trace-execution")]
                    {
                        chunk.disassemble_instruction(self.ip, output);
                    }

                    self.ip += 1;
                    match op {
                        OpCode::Constant(const_index) => {
                            writeln!(output, "OpConstant: {}", const_index)
                                .expect("failed to write to output");
                        }
                        OpCode::Return => {
                            return InterpretResult::Ok;
                        }
                    }
                }
                None => {
                    return InterpretResult::RuntimeError;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_interpret_happy_path() {
        let mut chunk = Chunk::new("test chunk");
        let const_index = chunk.add_constant(Value(42.0));
        chunk.write(OpCode::Constant(const_index), 0);
        chunk.write(OpCode::Return, 0);
        let mut output = Vec::new();
        let result = VM::new().interpret(&chunk, &mut output);
        assert_eq!(result, InterpretResult::Ok);
        #[cfg(not(feature = "debug-trace-execution"))]
        assert_eq!(String::from_utf8(output).unwrap(), "OpConstant: 0\n");
    }

    #[test]
    fn test_interpret_missing_return() {
        let mut chunk = Chunk::new("test chunk");
        let const_index = chunk.add_constant(Value(42.0));
        chunk.write(OpCode::Constant(const_index), 0);
        let mut output = Vec::new();
        let result = VM::new().interpret(&chunk, &mut output);
        assert_eq!(result, InterpretResult::RuntimeError);
        #[cfg(not(feature = "debug-trace-execution"))]
        assert_eq!(String::from_utf8(output).unwrap(), "OpConstant: 0\n");
    }

    #[test]
    fn test_interpret_exits_on_return() {
        let mut chunk = Chunk::new("test chunk");
        chunk.write(OpCode::Return, 0);
        let const_index = chunk.add_constant(Value(42.0));
        chunk.write(OpCode::Constant(const_index), 0);
        let mut output = Vec::new();
        let result = VM::new().interpret(&chunk, &mut output);
        assert_eq!(result, InterpretResult::Ok);
        #[cfg(not(feature = "debug-trace-execution"))]
        assert_eq!(String::from_utf8(output).unwrap(), "");
        #[cfg(feature = "debug-trace-execution")]
        assert_eq!(String::from_utf8(output).unwrap(), "0000    0 OP_RETURN\n");
    }
}
