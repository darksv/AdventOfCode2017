fn main() {
    let code = read_instructions();
    println!("1: {}", execute_simple(&code));
}

fn execute_simple(code: &[Inst]) -> isize {
    let mut regs = vec![0isize; 26];
    execute(&code, &mut regs).unwrap_or(0)
}

#[derive(Debug)]
enum Inst {
    Set(Operand, Operand),
    Mul(Operand, Operand),
    Jgz(Operand, Operand),
    Add(Operand, Operand),
    Mod(Operand, Operand),
    Snd(Operand),
    Rcv(Operand),
}

#[derive(Debug, Copy, Clone)]
enum Operand {
    Imm(isize),
    Reg(u8),
}

fn read_instructions() -> Vec<Inst> {
    let mut instructions = vec![];
    use std::io::BufRead;
    let file = std::fs::File::open("input.txt").unwrap();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        if let Some(inst) = parse_opcode(&line) {
            instructions.push(inst);
        }
    }
    instructions
}

#[inline]
fn parse_reg(reg: &str) -> Option<u8> {
    reg
        .chars()
        .next()
        .map(|c| c as u8 - b'a')
}

fn parse_opcode<'a>(inst: &'a str) -> Option<Inst> {
    let mut i = inst.split(' ');
    i.next().map(|opcode| {
        let op1 = match i.next() {
            Some(s) => parse_operand(s),
            None => None,
        };
        let op2 = match i.next() {
            Some(s) => parse_operand(s),
            None => None,
        };
        match opcode {
            "set" => Inst::Set(op1.unwrap(), op2.unwrap()),
            "mul" => Inst::Mul(op1.unwrap(), op2.unwrap()),
            "jgz" => Inst::Jgz(op1.unwrap(), op2.unwrap()),
            "add" => Inst::Add(op1.unwrap(), op2.unwrap()),
            "mod" => Inst::Mod(op1.unwrap(), op2.unwrap()),
            "snd" => Inst::Snd(op1.unwrap()),
            "rcv" => Inst::Rcv(op1.unwrap()),
            other => panic!("unknown opcode: {}", other)
        }
    })
}

fn parse_operand(op: &str) -> Option<Operand> {
    match op.parse() {
        Ok(imm) => Some(Operand::Imm(imm)),
        Err(_) => parse_reg(op).map(|x| Operand::Reg(x))
    }
}

fn execute(code: &[Inst], regs: &mut [isize]) -> Option<isize> {
    let mut last_play = None;
    let mut pc = 0;
    while pc < code.len() {
        match &code[pc] {
            &Inst::Set(a, b) => {
                let val = get_value(b, regs);
                if let Some(r) = get_reg(a, regs) {
                    *r = val;
                }
            }
            &Inst::Add(a, b) => {
                let val = get_value(b, regs);
                if let Some(r) = get_reg(a, regs) {
                    *r += val;
                }
            }
            &Inst::Mul(a, b) => {
                let val = get_value(a, regs) * get_value(b, regs);
                if let Some(r) = get_reg(a, regs) {
                    *r = val;
                }
            }
            &Inst::Mod(a, b) => {
                let val = get_value(a, regs) % get_value(b, regs);
                if let Some(r) = get_reg(a, regs) {
                    *r = val;
                }
            }
            &Inst::Rcv(a) => {
                if get_value(a, regs) != 0 {
                    break;
                }
            }
            &Inst::Snd(a) => {
                last_play = Some(get_value(a, regs));
            }
            &Inst::Jgz(a, b) => {
                if get_value(a, regs) > 0 {
                    let offset = get_value(b, regs);
                    pc = (pc as isize + offset) as usize;
                    continue;
                }
            }
        }
        pc += 1;
    }
    last_play
}

#[inline]
fn get_value(o: Operand, regs: &[isize]) -> isize {
    match o {
        Operand::Reg(r) => regs[r as usize],
        Operand::Imm(i) => i,
    }
}

fn get_reg<'a>(o: Operand, regs: &'a mut [isize]) -> Option<&'a mut isize> {
    match o {
        Operand::Reg(r) => Some(&mut regs[r as usize]),
        Operand::Imm(_) => None,
    }
}