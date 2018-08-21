use std::io::Read;
use std::io::Write;
use std::iter::repeat;
use Instruction;

pub struct Program {
    cursor: usize,
    prog_counter: usize,
    program: Vec<Instruction>,
    data: Vec<u8>,
    count: usize,
}
#[allow(dead_code)]
impl Program {
    pub fn new(p: impl IntoIterator<Item = Instruction>) -> Program {
        Program {
            cursor: 0,
            prog_counter: 0,
            program: p.into_iter().collect(),
            data: vec![0; 1024],
            count: 0,
        }
    }

    pub fn step(&mut self, r: &mut Read, w: &mut Write) {
        use Instruction::*;
        let l = self.data.len();
        let c = self.cursor;
        match self.program[self.prog_counter] {
            Mutate(m) => {
                let elt = self
                    .data
                    .get_mut(self.cursor)
                    .unwrap_or_else(|| panic!("{:?} not allocated out of {:?}", c, l));
                *elt = elt.wrapping_add(m as u8);
            }
            Move(m) => {
                let mut signed_cursor = self.cursor as isize + m;
                //println!("SC: {:?} {:?}", signed_cursor, self.data.len() as isize);
                if signed_cursor < 0 {
                    let mut expanded = Vec::with_capacity(self.data.len() * 2);
                    expanded.extend(repeat(0).take(self.data.len()));
                    expanded.extend(self.data.iter());
                    signed_cursor += self.data.len() as isize;
                    self.data = expanded;
                } else if signed_cursor >= self.data.len() as isize {
                    self.data.resize(2*signed_cursor as usize, 0);
                }
                self.cursor = signed_cursor as usize;
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
                w.write(&mut self.data[self.cursor..self.cursor + 1])
                    .unwrap();
            }
        }
        self.prog_counter += 1;
        self.count += 1;
    }

    pub fn run(&mut self, r: &mut Read, w: &mut Write) -> usize {
        while self.prog_counter < self.program.len() {
            self.step(r, w);
        }
        return self.count;
    }
}
