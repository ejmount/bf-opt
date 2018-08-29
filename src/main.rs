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
}

static ITERS : usize = 1;

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
                            
                            println!("Got {:?} instructions", prog.len());
                            let mut data :Vec<_> = (0..ITERS).map(|_| prog.clone()).collect();
							let start = time::Instant::now();
							let mut c = 0;
							for i in 0..ITERS {
								let a = std::mem::replace(&mut data[i], Vec::new());
								let mut runner = interpreter::Program::new(a);
                            	c += runner.run(&mut std::io::stdin(), &mut std::io::stdout());
                            	//runner.print();
                        	}
                            //let c = 0;
                            let end = time::Instant::now();
                            let dur = end.duration_since(start);
                            println!(
                                "Ran {:?} steps in {:?}, {:?} MIPS",
                                c,
                                dur,
                                (c as f64 / (dur.as_nanos() as f64) * 1e3)
                            );
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
