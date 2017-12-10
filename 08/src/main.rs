#![feature(entry_and_modify)]
#![feature(map_entry_replace)]
#![feature(entry_or_default)]

#[derive(Debug)]
enum Operator {
    Dec,
    Inc,
}

#[derive(Debug)]
enum Comparison {
    Equal,
    NotEqual,
    LessOrEqual,
    GreaterOrEqual,
    Less,
    Greater,
}

#[derive(Debug)]
struct Instruction {
    dst: String,
    op: Operator,
    arg: isize,
    src: String,
    comp: Comparison,
    val: isize,
}

fn main() {
    use std::collections::HashMap;
    use std::io::BufRead;

    let mut regs = HashMap::new();
    let mut max = 0;
    let mut current_max = 0;

    let file = std::fs::File::open("input.txt").unwrap();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let inst = parse_inst(&line);

        let value = regs
            .entry(inst.src)
            .or_insert(0)
            .clone();

        let predicate = match inst.comp {
            Comparison::GreaterOrEqual => value >= inst.val,
            Comparison::Greater => value > inst.val,
            Comparison::LessOrEqual => value <= inst.val,
            Comparison::Less => value < inst.val,
            Comparison::Equal => value == inst.val,
            Comparison::NotEqual => value != inst.val,
        };

        if predicate {
            use std::collections::hash_map::Entry;

            let entry = regs
                .entry(inst.dst);
            let current = match &entry {
                &Entry::Vacant(_) => 0,
                &Entry::Occupied(ref e) => *e.get()
            };
            let new = match inst.op {
                Operator::Inc => current + inst.arg,
                Operator::Dec => current - inst.arg,
            };
            entry
                .and_modify(|e| *e = new)
                .or_insert(new);
        }

        current_max = *regs.values().max().unwrap();
        if current_max > max {
            max = current_max;
        }
    }

    println!("1: {:?}", current_max);
    println!("2: {:?}", max);
}


fn parse_inst(s: &str) -> Instruction {
    let mut mod_reg = None;
    let mut op = None;
    let mut diff = None;
    let mut reg = None;
    let mut comp = None;
    let mut val = None;

    for (index, part) in s.split(' ').enumerate() {
        match index {
            0 => {
                mod_reg = Some(part.to_owned());
            }
            1 => {
                op = Some(match part {
                    "inc" => Operator::Inc,
                    "dec" => Operator::Dec,
                    _ => panic!("wtf?")
                });
            }
            2 => {
                diff = Some(part.parse::<isize>().unwrap());
            }
            3 => (),
            4 => {
                reg = Some(part.to_owned());
            }
            5 => {
                comp = Some(match part {
                    "==" => Comparison::Equal,
                    "!=" => Comparison::NotEqual,
                    "<=" => Comparison::LessOrEqual,
                    ">=" => Comparison::GreaterOrEqual,
                    "<" => Comparison::Less,
                    ">" => Comparison::Greater,
                    _ => panic!("wtf?")
                });
            }
            6 => {
                val = Some(part.parse::<isize>().unwrap());
            }
            _ => panic!("unexpected part #{}: {}", index, part)
        }
    }

    Instruction {
        dst: mod_reg.unwrap(),
        op: op.unwrap(),
        arg: diff.unwrap(),
        src: reg.unwrap(),
        comp: comp.unwrap(),
        val: val.unwrap()
    }
}