use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::value::Value;

pub struct VM<'a> {
    chunk: Option<&'a Chunk<'a>>,
    ip: usize,
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: vec![],
        }
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
                        writeln!(output, "STACK: {:?}", self.stack)
                            .expect("failed to write to output");
                        chunk.disassemble_instruction(self.ip, output);
                    }

                    self.ip += 1;
                    match op {
                        OpCode::Constant(const_index) => {
                            self.stack.push(chunk.read_constant(*const_index))
                        }
                        OpCode::Return => {
                            if let Some(val) = self.stack.pop() {
                                writeln!(output, "{}", val).expect("failed to write to output")
                            }
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
        assert_eq!(String::from_utf8(output).unwrap(), "42\n");
    }

    #[test]
    fn test_interpret_missing_return() {
        let mut chunk = Chunk::new("test chunk");
        let const_index = chunk.add_constant(Value(42.0));
        chunk.write(OpCode::Constant(const_index), 0);
        let mut output = Vec::new();
        let result = VM::new().interpret(&chunk, &mut output);
        assert_eq!(result, InterpretResult::RuntimeError);
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
        assert_eq!(
            String::from_utf8(output).unwrap(),
            concat!("STACK: []\n", "0000    0 OP_RETURN\n")
        );
    }

    #[test]
    fn lone_return_does_nothing() {
        let mut chunk = Chunk::new("test chunk");
        chunk.write(OpCode::Return, 0);
        let mut output = Vec::new();
        let result = VM::new().interpret(&chunk, &mut output);
        assert_eq!(result, InterpretResult::Ok);
        #[cfg(not(feature = "debug-trace-execution"))]
        assert_eq!(String::from_utf8(output).unwrap(), "");
    }
}
