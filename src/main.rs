use std::process::ExitCode;

mod chunk;

fn main() -> ExitCode {
    let chunk = chunk::Chunk {
        op_codes: vec![chunk::OpCode::Return],
    };
    disassemble_chunk(&chunk, "test chunk");

    ExitCode::SUCCESS
}

fn disassemble_chunk(chunk: &chunk::Chunk, name: &str) {
    println!("== {} ==", name);
    for (i, op_code) in chunk.op_codes.iter().enumerate() {
        println!("{:04} OP_{}", i, format!("{:?}", op_code).to_uppercase());
    }
}
