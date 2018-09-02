use Instruction;
use Instruction::*;

#[allow(dead_code)]
pub fn compile(tokens: impl IntoIterator<Item = char>) -> Result<Vec<Instruction>, usize> {
    let mut insts: Vec<_> = parse(tokens).collect();
    optimize(&mut insts);
    link_branches(&mut insts)?;
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

pub fn optimize(insts: &mut Vec<Instruction>) {
    let mut old_len;
    while {
        old_len = insts.len();
        merge_general(
            insts,
            |t| Move(t),
            |m| if let Move(t) = m { Some(t) } else { None },
        );
        merge_general(
            insts,
            |t| Mutate(t),
            |m| if let Mutate(t) = m { Some(t) } else { None },
        );
        find_resets(insts);
        find_transfers(insts);
        old_len != insts.len()
    } {} // Horrible hack for do-while
}

fn find_resets(insts: &mut Vec<Instruction>) {
	let mut i = 0; 
	while i < insts.len()-2 {
		if let (JumpIfZero(0), Mutate(-1), JumpIfNonZero(0)) = (insts[i], insts[i+1], insts[i+2]) {
			insts[i] = Reset;
			insts.drain(i+1..=i+2);
		}
		i += 1;
	}
}

fn find_transfers(insts: &mut Vec<Instruction>) {
	let mut i = 0; 
	while i < insts.len()-6 {
		match insts[i..i+6] {
			[JumpIfZero(_), Move(d), Mutate(s), Move(f), Mutate(-1), JumpIfNonZero(_)]
			if d == -f => {
			insts[i] = Transfer(d, s);
			insts.drain(i+1..i+6);
		}
		_ => {}
	}
		i += 1;
	}
}




fn merge_general<C, D>(insts: &mut Vec<Instruction>, create: C, unwrap: D)
where
    C: Fn(isize) -> Instruction,
    D: Fn(Instruction) -> Option<isize>,
{
    let mut c = insts.len() - 1;
    while c < insts.len() {
        if unwrap(insts[c]).is_some() {
            let mut k = c;
            let mut t = 0;
            while k <= c {
                if let Some(m) = unwrap(insts[k]) {
                    t += m;
                } else {
                    break;
                }
                k = k.wrapping_sub(1);
            }
            k = k.wrapping_add(1);
            insts[k] = create(t);
            let start = if t == 0 { k } else { k + 1 };
            insts.drain(start..=c);
            c = k;
        }
        c = c.wrapping_sub(1);
    }
}
