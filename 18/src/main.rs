fn main() {
    let code = read_instructions();
    println!("1: {}", execute_simple(&code));
    println!("2: {}", execute_advanced(&code));
}

fn execute_simple(code: &[Inst]) -> isize {
    let mut regs = init_regs(0);
    let mut machine = Machine::new(&mut regs, code);

    let mut last_snd = None;
    while machine.has_next() {
        match machine.step() {
            StepResult::Ok => (),
            StepResult::Send(val) => {
                last_snd = Some(val);
            }
            StepResult::Receive(r) => {
                let last_snd = last_snd.unwrap_or(0);
                if last_snd != 0 {
                    return last_snd;
                }
                machine.regs[r as usize] = last_snd;
            }
        }
    }
    0
}

fn execute_advanced(code: &[Inst]) -> isize {
    use std::collections::vec_deque::VecDeque;

    let mut regs1 = init_regs(0);
    let mut regs2 = init_regs(1);

    let mut machine_1 = Machine::new(&mut regs1, code);
    let mut machine_2 = Machine::new(&mut regs2, code);

    let mut messages_for_1 = VecDeque::new();
    let mut messages_for_2 = VecDeque::new();

    let mut message_dst_for_1 = None;
    let mut message_dst_for_2 = None;

    let mut count = 0;
    loop {
        if let Some(reg) = message_dst_for_1 {
            if let Some(msg) = messages_for_1.pop_back() {
                machine_1.regs[reg as usize] = msg;
                message_dst_for_1 = None;
            }
        } else if machine_1.has_next() {
            match machine_1.step() {
                StepResult::Ok => (),
                StepResult::Send(val) => {
                    messages_for_2.push_front(val);
                }
                StepResult::Receive(r) => {
                    message_dst_for_1 = Some(r);
                }
            }
        }

        if let Some(reg) = message_dst_for_2 {
            if let Some(msg) = messages_for_2.pop_back() {
                machine_2.regs[reg as usize] = msg;
                message_dst_for_2 = None;
            }
        } else if machine_2.has_next() {
            match machine_2.step() {
                StepResult::Ok => (),
                StepResult::Send(val) => {
                    messages_for_1.push_front(val);
                    count += 1;
                }
                StepResult::Receive(r) => {
                    message_dst_for_2 = Some(r);
                }
            }
        }

        // Deadlock detection
        if message_dst_for_1.is_some() && messages_for_1.is_empty() &&
            message_dst_for_2.is_some() && messages_for_2.is_empty() {
            return count;
        }
    }
}

fn init_regs(p: isize) -> [isize; 26] {
    let mut regs = [0isize; 26];
    regs[parse_reg("p").unwrap() as usize] = p;
    regs
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

struct Machine<'a> {
    code: &'a [Inst],
    regs: &'a mut [isize],
    pc: usize,
}

enum StepResult {
    Ok,
    Send(isize),
    Receive(u8),
}

impl<'a> Machine<'a> {
    pub fn new(regs: &'a mut [isize], code: &'a [Inst]) -> Self {
        Self {
            pc: 0,
            code,
            regs,
        }
    }

    pub fn step(&mut self) -> StepResult {
        let mut result = StepResult::Ok;
        match &self.code[self.pc] {
            &Inst::Set(x, y) => {
                let val = self.get_value(y);
                if let Some(r) = self.get_reg(x) {
                    *r = val;
                }
            }
            &Inst::Add(x, y) => {
                let val = self.get_value(y);
                if let Some(r) = self.get_reg(x) {
                    *r += val;
                }
            }
            &Inst::Mul(x, y) => {
                let val = self.get_value(x) * self.get_value(y);
                if let Some(r) = self.get_reg(x) {
                    *r = val;
                }
            }
            &Inst::Mod(x, y) => {
                let val = self.get_value(x) % self.get_value(y);
                if let Some(r) = self.get_reg(x) {
                    *r = val;
                }
            }
            &Inst::Rcv(Operand::Reg(r)) => {
                result = StepResult::Receive(r);
            }
            &Inst::Snd(x) => {
                result = StepResult::Send(self.get_value(x));
            }
            &Inst::Jgz(x, y) => {
                if self.get_value(x) > 0 {
                    let offset = self.get_value(y);
                    self.pc = (self.pc as isize + offset) as usize;
                    return result;
                }
            }
            _ => panic!("invalid instruction")
        }
        self.pc += 1;
        result
    }

    fn has_next(&self) -> bool {
        self.pc < self.code.len()
    }

    #[inline]
    fn get_value(&mut self, o: Operand) -> isize {
        match o {
            Operand::Reg(r) => self.regs[r as usize],
            Operand::Imm(i) => i,
        }
    }

    #[inline]
    fn get_reg(&mut self, o: Operand) -> Option<&mut isize> {
        match o {
            Operand::Reg(r) => Some(&mut self.regs[r as usize]),
            Operand::Imm(_) => None,
        }
    }
}