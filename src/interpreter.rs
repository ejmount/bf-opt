use std::io::Read;
use std::io::Write;
use std::iter::repeat;
use Instruction;

pub struct Program {
    cursor: usize,
    prog_counter: usize,
    program: Vec<Instruction>,
    step_per_inst: Vec<usize>,
    data: Vec<u8>,
}
#[allow(dead_code)]
impl Program {
    pub fn new(p: impl IntoIterator<Item = Instruction>) -> Program {
        let p: Vec<_> = p.into_iter().collect();
        Program {
            cursor: 0,
            prog_counter: 0,
            step_per_inst: vec![0; p.len() + 1],
            program: p,
            data: vec![0; 10240],
        }
    }

    pub fn print(&self) {
        let mut depth = 0;

        for (index, i) in self.program.iter().enumerate() {
            print!("{:05} ", self.step_per_inst[index]);
            if let Instruction::JumpIfNonZero(_) = i {
                depth -= 1;
            }
            for _ in 0..depth {
                print!("\t");
            }
            println!("{:?}", i);
            if let Instruction::JumpIfZero(_) = i {
                depth += 1;
            }
        }
    }

    fn allocate_location(&mut self, signed_cursor: isize) -> usize {
        if signed_cursor < 0 {
            let mut expanded = Vec::with_capacity(self.data.len() * 2);
            expanded.extend(repeat(0).take(self.data.len()));
            expanded.extend(self.data.iter());
            self.data = expanded;
            return (self.data.len() as isize + signed_cursor) as usize;
        } else if signed_cursor >= self.data.len() as isize {
            self.data.resize(2 * signed_cursor as usize, 0);
        }
        return signed_cursor as usize;
    }

    pub fn step(&mut self, r: &mut Read, w: &mut Write) {
        use Instruction::*;
        match self.program[self.prog_counter] {
            Mutate(m) => {
                assert!(self.cursor < self.data.len());
                let mut elt = &mut self.data[self.cursor];
                *elt = elt.wrapping_add(m as u8);
            }
            Move(m) => {
                let mut signed_cursor = self.cursor as isize + m;
                self.cursor = self.allocate_location(signed_cursor);
            }
            JumpIfZero(n) => {
                if self.data[self.cursor] == 0 {
                    self.prog_counter = (self.prog_counter as isize + n) as usize;
                }
            }
            JumpIfNonZero(n) => {
                if self.data[self.cursor] != 0 {
                    self.prog_counter = (self.prog_counter as isize + n) as usize;
                }
            }
            Input => match r.read(&mut self.data[self.cursor..self.cursor + 1]) {
                Ok(_) => {}
                Err(_) => {}
            },
            Output => {
                /*w.write(&mut self.data[self.cursor..self.cursor + 1])
                    .unwrap();*/
            }
            Reset => {
                self.data[self.cursor] = 0;
            }
            Transfer(index, f) => {
                let dest = self.cursor as isize + index;
                let dest_in_bounds = self.allocate_location(dest);
                let transfer_amount = self.data[self.cursor] as isize * f;
                let rem = (transfer_amount % 256) as i8;
                self.data[dest_in_bounds] =
                    (self.data[dest_in_bounds] as i8).wrapping_add(rem) as u8;
                self.data[self.cursor] = 0;
            }
        }
        self.step_per_inst[self.prog_counter] += 1;
        self.prog_counter += 1;
    }

    pub fn run(&mut self, r: &mut Read, w: &mut Write) -> usize {
        while self.prog_counter < self.program.len() {
            self.step(r, w);
        }
        return self.step_per_inst.iter().sum();
    }
}
