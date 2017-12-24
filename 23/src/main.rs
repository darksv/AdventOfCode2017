fn main() {
    let code = read_instructions();
    println!("1: {}", execute_simple(&code));
    print_c(&code);
}

fn print_c(code: &[Inst]) {
    let mut insts_with_label = std::collections::HashSet::new();

    for (idx, inst) in code.iter().enumerate() {
        if let &Inst::Jnz(_, Operand::Imm(offset)) = inst {
            let addr = offset_addr(idx, offset);
            insts_with_label.insert(addr);
        }
    }

    for (idx, inst) in code.iter().enumerate() {
        if insts_with_label.contains(&idx) {
            println!("label_{}:", idx);
        }

        match inst {
            &Inst::Set(a, b) => println!("{} = {};", a, b),
            &Inst::Sub(a, b) => {
                match b {
                    Operand::Imm(b) if b < 0 => {
                        println!("{} += {};", a, -b)
                    }
                    b => {
                        println!("{} -= {};", a, b)
                    }
                }
            },
            &Inst::Mul(a, b) => println!("{} *= {};", a, b),
            &Inst::Jnz(Operand::Reg(r), Operand::Imm(offset)) => {
                println!("if ({} != 0) {{\n goto label_{};\n}}", Operand::Reg(r), offset_addr(idx, offset));
            },
            &Inst::Jnz(Operand::Imm(x), Operand::Imm(offset)) => {
                if x != 0 {
                    println!("goto label_{};", offset_addr(idx, offset))
                } else {
                    println!(";");
                }
            },
            _ => (),
        }
    }

    if insts_with_label.contains(&code.len()) {
        println!("label_{}: ;", code.len());
    }
}

fn offset_addr(idx: usize, offset: isize) -> usize {
    if offset < 0 {
        idx - (-offset) as usize
    } else {
        idx + offset as usize
    }
}

fn execute_simple(code: &[Inst]) -> usize {
    let mut regs = vec![0isize; 26];
    execute_basic(&code, &mut regs)
}

#[derive(Debug)]
enum Inst {
    Set(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Jnz(Operand, Operand),
}

#[derive(Debug, Copy, Clone)]
enum Operand {
    Imm(isize),
    Reg(u8),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Operand::Imm(i) => f.write_fmt(format_args!("{}", i)),
            &Operand::Reg(r) => f.write_fmt(format_args!("{}", (b'a' + r) as char)),
        }
    }
}

impl std::fmt::Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Inst::Set(a, b) => f.write_fmt(format_args!("set {} {}", a, b)),
            &Inst::Sub(a, b) => f.write_fmt(format_args!("sub {} {}", a, b)),
            &Inst::Mul(a, b) => f.write_fmt(format_args!("mul {} {}", a, b)),
            &Inst::Jnz(a, b) => f.write_fmt(format_args!("jnz {} {}", a, b)),
        }
    }
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
            "sub" => Inst::Sub(op1.unwrap(), op2.unwrap()),
            "mul" => Inst::Mul(op1.unwrap(), op2.unwrap()),
            "jnz" => Inst::Jnz(op1.unwrap(), op2.unwrap()),
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

fn execute_basic(code: &[Inst], regs: &mut [isize]) -> usize {
    let mut muls = 0;
    let mut pc = 0;
    while pc < code.len() {
        match &code[pc] {
            &Inst::Set(a, b) => {
                let val = get_value(b, regs);
                if let Some(r) = get_reg(a, regs) {
                    *r = val;
                }
            }
            &Inst::Sub(a, b) => {
                let val = get_value(b, regs);
                if let Some(r) = get_reg(a, regs) {
                    *r -= val;
                }
            }
            &Inst::Mul(a, b) => {
                let val = get_value(a, regs) * get_value(b, regs);
                if let Some(r) = get_reg(a, regs) {
                    *r = val;
                    muls += 1;
                }
            }
            &Inst::Jnz(a, b) => {
                if get_value(a, regs) != 0 {
                    let offset = get_value(b, regs);
                    pc = (pc as isize + offset) as usize;
                    continue;
                }
            }
        }
        pc += 1;
    }
    muls
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