mod ast;
mod errors;
mod eval;
mod lexer;
mod object;
mod parser;
mod tokens;
use lexer::Lexer;
use std::env;

use crate::{eval::Interpreter, parser::Parser};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    if args.len() > 1 {
        let program = read_program(args[1].clone());
        match program {
            Ok(v) => {
                run_program(v);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("Hello, world!");
}

fn read_program(path: String) -> Result<String, std::io::Error> {
    let content_bs = std::fs::read(path)?;
    let content = String::from_utf8_lossy(&content_bs);
    Ok(content.to_string())
}

fn run_program(program: String) {
    let mut lexer = Lexer::new(program);
    let tokens = lexer.scan_tokens();
    println!("{:?}", tokens);
    match tokens {
        Ok(tks) => {
            let mut p = Parser::new(tks);
            let program_res = p.parse();
            if program_res.is_err() {
                println!("parser err: {:?}", program_res.err());
                return;
            }

            println!("{:?}", program_res);

            let mut interpreter = Interpreter::new();
            let result = interpreter.eval_program(program_res.ok().unwrap());
            println!("result: {:?}", result);
            return;
        }
        Err(e) => {
            println!("lexer err: {:?}", e);
        }
    }
}

fn run_interactive_eval() {
    let mut interpreter = Interpreter::new();

    loop {
        print!(">>>");
        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => {
                let mut lexer = Lexer::new(buf);
                let tokens = lexer.scan_tokens();
                println!("{:?}", tokens);
                match tokens {
                    Ok(tks) => {
                        let mut p = Parser::new(tks);
                        let program_res = p.parse();
                        if program_res.is_err() {
                            println!("parser err: {:?}", program_res.err());
                            return;
                        }

                        println!("{:?}", program_res);
                        let result = interpreter.eval_program(program_res.ok().unwrap());
                        println!("result: {:?}", result);
                        return;
                    }
                    Err(e) => {
                        println!("lexer err: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
