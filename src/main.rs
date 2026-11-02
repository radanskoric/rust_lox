use std::process::ExitCode;

mod chunk;
mod value;
mod vm;

fn main() -> ExitCode {
    let vm = vm::VM::new();
    let mut chunk = chunk::Chunk::new("test chunk");
    let const_index = chunk.add_constant(value::Value(1.2));
    chunk.write(chunk::OpCode::Constant(const_index), 123);
    chunk.write(chunk::OpCode::Return, 123);
    let mut output = std::io::stdout();
    chunk.disassemble(&mut output);
    vm.interpret(&chunk, &mut output);

    ExitCode::SUCCESS
}
