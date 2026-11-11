use std::process::ExitCode;

#[macro_use]
mod xwrite;

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
    println!("Executing chunk:");
    chunk.disassemble(&mut output);
    println!("Executing...");
    vm.interpret(&chunk, &mut output);

    ExitCode::SUCCESS
}
