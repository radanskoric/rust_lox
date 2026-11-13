use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::compiler::compile;
use crate::value::Value;

pub struct VM<'a> {
    chunk: Option<Chunk<'a>>,
    ip: usize,
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

macro_rules! binary_op {
    ($context:expr, $op:tt) => {
        if let Some(b) = $context.stack.pop() {
            if let Some(a) = $context.stack.pop() {
                $context.stack.push(Value(a.0 $op b.0))
            }
        }
    };
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: vec![],
        }
    }

    pub fn interpret(&mut self, source: &str, output: &mut impl std::io::Write) -> InterpretResult {
        match compile(source) {
            Ok(chunk) => {
                self.chunk = Some(chunk);
                self.run(output)
            }
            Err(_) => InterpretResult::CompileError,
        }
    }

    fn run(&mut self, output: &mut impl std::io::Write) -> InterpretResult {
        let chunk = self.chunk.as_ref().unwrap();
        self.ip = 0;
        loop {
            match chunk.get(self.ip) {
                Some(op) => {
                    #[cfg(feature = "debug-trace-execution")]
                    {
                        xwriteln!(output, "STACK: {:?}", self.stack);
                        chunk.disassemble_instruction(self.ip, output);
                    }

                    self.ip += 1;
                    match op {
                        OpCode::Constant(const_index) => {
                            self.stack.push(chunk.read_constant(*const_index))
                        }
                        OpCode::Negate => {
                            if let Some(val) = self.stack.pop() {
                                self.stack.push(Value(-val.0))
                            }
                        }
                        OpCode::Add => binary_op!(self, +),
                        OpCode::Subtract => binary_op!(self, -),
                        OpCode::Multiply => binary_op!(self, *),
                        OpCode::Divide => binary_op!(self, /),
                        OpCode::Return => {
                            if let Some(val) = self.stack.pop() {
                                xwriteln!(output, "{}", val)
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
    use crate::chunk;

    use super::*;

    #[test]
    fn test_interpret_happy_path() {
        let mut chunk = Chunk::new("test chunk");
        let const_index = chunk.add_constant(Value(42.0));
        chunk.write(OpCode::Constant(const_index), 0);
        chunk.write(OpCode::Return, 0);
        let mut output = Vec::new();
        let mut vm = VM::new();
        vm.chunk = Some(chunk);
        let result = vm.run(&mut output);
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
        let mut vm = VM::new();
        vm.chunk = Some(chunk);
        let result = vm.run(&mut output);
        assert_eq!(result, InterpretResult::RuntimeError);
    }

    #[test]
    fn test_interpret_exits_on_return() {
        let mut chunk = Chunk::new("test chunk");
        chunk.write(OpCode::Return, 0);
        let const_index = chunk.add_constant(Value(42.0));
        chunk.write(OpCode::Constant(const_index), 0);
        let mut output = Vec::new();
        let mut vm = VM::new();
        vm.chunk = Some(chunk);
        let result = vm.run(&mut output);
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
        let mut vm = VM::new();
        vm.chunk = Some(chunk);
        let result = vm.run(&mut output);
        assert_eq!(result, InterpretResult::Ok);
        #[cfg(not(feature = "debug-trace-execution"))]
        assert_eq!(String::from_utf8(output).unwrap(), "");
    }

    #[test]
    fn test_interpret_negate() {
        let mut chunk = Chunk::new("test chunk");
        let const_index = chunk.add_constant(Value(42.0));
        chunk.write(OpCode::Constant(const_index), 0);
        chunk.write(OpCode::Negate, 1);
        chunk.write(OpCode::Return, 2);
        let mut output = Vec::new();
        let mut vm = VM::new();
        vm.chunk = Some(chunk);
        let result = vm.run(&mut output);
        assert_eq!(result, InterpretResult::Ok);
        #[cfg(not(feature = "debug-trace-execution"))]
        assert_eq!(String::from_utf8(output).unwrap(), "-42\n");
    }

    #[test]
    fn test_interpret_runs_binary_arithmetic_operations() {
        let mut chunk = Chunk::new("test chunk");
        // The opcodes to calculate: ((1+2)*3-1)/2
        let const_index1 = chunk.add_constant(Value(1.0));
        let const_index2 = chunk.add_constant(Value(2.0));
        chunk.write(OpCode::Constant(const_index1), 0);
        chunk.write(OpCode::Constant(const_index2), 0);
        chunk.write(OpCode::Add, 0);
        let const_index3 = chunk.add_constant(Value(3.0));
        chunk.write(OpCode::Constant(const_index3), 0);
        chunk.write(OpCode::Multiply, 0);
        let const_index4 = chunk.add_constant(Value(1.0));
        chunk.write(OpCode::Constant(const_index4), 0);
        chunk.write(OpCode::Subtract, 0);
        let const_index5 = chunk.add_constant(Value(2.0));
        chunk.write(OpCode::Constant(const_index5), 0);
        chunk.write(OpCode::Divide, 0);
        chunk.write(OpCode::Return, 0);
        let mut output = Vec::new();
        let mut vm = VM::new();
        vm.chunk = Some(chunk);
        let result = vm.run(&mut output);
        assert_eq!(result, InterpretResult::Ok);
        #[cfg(not(feature = "debug-trace-execution"))]
        assert_eq!(String::from_utf8(output).unwrap(), "4\n");
        #[cfg(feature = "debug-trace-execution")]
        println!("{}", String::from_utf8(output).unwrap());
    }

    #[test]
    fn test_repeatable_runs_with_chunk_changes_work_correctly() {
        let mut chunk1 = Chunk::new("test chunk");
        let const_index1 = chunk1.add_constant(Value(1.0));
        chunk1.write(OpCode::Constant(const_index1), 0);
        chunk1.write(OpCode::Return, 0);
        let mut output = Vec::new();
        let mut vm = VM::new();
        vm.chunk = Some(chunk1);
        let result = vm.run(&mut output);
        assert_eq!(result, InterpretResult::Ok);

        let mut chunk2 = Chunk::new("test chunk 2");
        let const_index2 = chunk2.add_constant(Value(2.0));
        chunk2.write(OpCode::Constant(const_index2), 0);
        chunk2.write(OpCode::Return, 0);
        vm.chunk = Some(chunk2);
        let result = vm.run(&mut output);
        assert_eq!(result, InterpretResult::Ok);
        #[cfg(not(feature = "debug-trace-execution"))]
        assert_eq!(String::from_utf8(output).unwrap(), "1\n2\n");
    }
}
