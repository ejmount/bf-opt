#![feature(duration_as_u128)]

use std::time;
use std::io::Read;

mod interpreter;
mod parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Mutate(isize),
    Move(isize),
    Input,
    Output,
    JumpIfZero(isize),
    JumpIfNonZero(isize),
}

fn main() {
    let fname = std::env::args().skip(1).next();
    match fname {
        Some(s) => {
            let fh = std::fs::File::open(&s);
            match fh {
                Ok(mut f) => {
                    let mut src: String = String::new();
                    f.read_to_string(&mut src)
                        .expect("Failed to read file somehow");
                    match parser::compile(src.chars()) {
                        Ok(prog) => {
                            let mut runner = interpreter::Program::new(prog);
                            let start = time::Instant::now();
                            let c = runner.run(&mut std::io::stdin(), &mut std::io::stdout());
                            let end = time::Instant::now();
                            let dur = end.duration_since(start);
                            println!("Ran {:?} steps in {:?}, {:?} MIPS", c, dur, (c as f64/(dur.as_nanos() as f64)*1e3));
                        }
                        Err(i) => {
                            println!("Parse error at index {:?}", i);
                        }
                    }
                }
                Err(e) => {
                    println!("Could not open {:?}: {:?}", s, e);
                }
            }
        }
        None => {
            println!("No filename provided as arg 1");
        }
    }
}
