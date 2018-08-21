use Instruction;
use Instruction::*;

#[allow(dead_code)]
pub fn compile(tokens: impl IntoIterator<Item = char>) -> Result<Vec<Instruction>, usize> {
    let mut insts: Vec<_> = parse(tokens).collect();
    link_branches(&mut insts[..])?;
    return Ok(insts);
}

pub fn parse(tokens: impl IntoIterator<Item = char>) -> impl Iterator<Item = Instruction> {
    fn parsetok(c: char) -> Option<Instruction> {
        let i = match c {
            '+' => Mutate(1),
            '-' => Mutate(-1),
            '>' => Move(1),
            '<' => Move(-1),
            ',' => Input,
            '.' => Output,
            '[' => JumpIfZero(0),
            ']' => JumpIfNonZero(0),
            _ => return None, // Anything unrecognised is ignored
        };
        return Some(i);
    }
    tokens.into_iter().flat_map(parsetok)
}

pub fn link_branches(insts: &mut [Instruction]) -> Result<(), usize> {
    let mut current = 0;
    let mut bracket_stack = Vec::new();
    while current < insts.len() {
        match insts[current] {
            JumpIfZero(_) => bracket_stack.push(current),
            JumpIfNonZero(_) => {
                let opening = bracket_stack.pop().ok_or(current)?;
                insts[current] = JumpIfNonZero(opening as isize - current as isize);
                insts[opening] = JumpIfZero(current as isize - opening as isize);
            }
            _ => {}
        }
        current += 1;
    }
    if bracket_stack.len() == 0 {
        Ok(())
    } else {
        Err(bracket_stack.pop().unwrap())
    }
}
