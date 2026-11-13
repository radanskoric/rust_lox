use std::io::Write;
use std::process::ExitCode;

#[macro_use]
mod xwrite;

mod chunk;
mod compiler;
mod scanner;
mod value;
mod vm;

use vm::InterpretResult;

fn main() -> ExitCode {
    let mut vm = vm::VM::new();

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        repl(&mut vm)
    } else if args.len() == 2 {
        run_file(&mut vm, &args[1])
    } else {
        eprintln!("Usage: rust_lox [path]");
        return ExitCode::FAILURE;
    }
}

fn repl(vm: &mut vm::VM) -> ExitCode {
    loop {
        let mut line = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        match std::io::stdin().read_line(&mut line) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                match vm.interpret(&line, &mut std::io::stdout()) {
                    InterpretResult::Ok => {}
                    InterpretResult::CompileError => {
                        eprintln!("Compile error");
                    }
                    InterpretResult::RuntimeError => {
                        eprintln!("Runtime error");
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }
    }
    ExitCode::SUCCESS
}

fn run_file(vm: &mut vm::VM, path: &str) -> ExitCode {
    println!("Running file: {}", path);
    let mut output = std::io::stdout();
    match read_file(path) {
        Ok(content) => match vm.interpret(&content, &mut output) {
            InterpretResult::Ok => ExitCode::SUCCESS,
            InterpretResult::CompileError => ExitCode::FAILURE,
            InterpretResult::RuntimeError => ExitCode::FAILURE,
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
