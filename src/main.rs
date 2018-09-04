#![feature(duration_as_u128)]

use std::io::Read;
use std::time;

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
    Reset,
    Transfer(isize, isize),
}

fn benchmark(src: String) {
    match parser::compile(src.chars(), false) {
        Ok(prog) => {
            
        	let insts = prog.len();
            let mut unopt_runner = interpreter::Program::new(prog);
            let start = time::Instant::now();
            let unopt_steps = unopt_runner.run(&mut std::io::stdin(), &mut std::io::stdout());
            let end = time::Instant::now();

            let opt_prog = parser::compile(src.chars(), true).unwrap();
            let opt_insts = opt_prog.len();
            let mut opt_runner = interpreter::Program::new(opt_prog);
            let opt_start = time::Instant::now();
            let opt_steps = opt_runner.run(&mut std::io::stdin(), &mut std::io::stdout());
            let opt_end = time::Instant::now();

            println!("{:?} unoptimized instructions, {:?} optimized", insts, opt_insts);

            let unopt_time = end.duration_since(start);
            let opt_time = opt_end.duration_since(opt_start);
            println!(
                "Ran {:?} unoptimized steps in {:?}, {:?} MIPS",
                unopt_steps,
                unopt_time,
                (unopt_steps as f64 / (unopt_time.as_nanos() as f64) * 1e3)
            );
            println!(
                "Ran {:?} optimized steps in {:?}, {:?} equivalent MIPS ({:?} real)",
                opt_steps,
                opt_time,
                (unopt_steps as f64 / (opt_time.as_nanos() as f64) * 1e3),
                (opt_steps as f64 / (opt_time.as_nanos() as f64) * 1e3)
            );
            println!("{:?}x speed improvement", unopt_time.as_nanos() as f64/(opt_time.as_nanos() as f64));
        }
        Err(i) => {
            println!("Parse error at index {:?}", i);
        }
    }
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
                    benchmark(src);
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
