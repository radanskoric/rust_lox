use std::process::ExitCode;

mod chunk;
mod value;

fn main() -> ExitCode {
    let mut chunk = chunk::Chunk::new("test chunk");
    chunk.write(chunk::OpCode::Return);
    let const_index = chunk.add_constant(value::Value(1.2));
    chunk.write(chunk::OpCode::Constant(const_index));
    chunk.disassemble();

    ExitCode::SUCCESS
}
