use std::{collections::HashMap, fmt::Display, usize, hash::{Hash, Hasher}};
use Register::*;
use Gate::*;

fn main() {
    let fstr: String = std::fs::read_to_string("./input/D24.txt").expect("Error Reading File!");

    let raw_info = RawMachineInfo::from_fstr(&fstr);

    println!("Intermediate count: {}", raw_info.int_gate_lookup.iter().count());
    println!("Z count: {}", raw_info.z_gate_lookup.iter().count());

    let eval: usize = part1(&raw_info);
    println!("The gates currently evaluate to: {}", eval);

    // for (id, _) in raw_info.z_gate_lookup.iter().enumerate() {
    //     raw_info.print_reg(&OutZ(id), 0);
    // }

    let out = part2(&raw_info);
    println!("Swap found! Swap: {}", out.join(","));
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Register {
    ConstX(usize),
    ConstY(usize),
    Intermediate(String),
    OutZ(usize)
}

impl Register {

    fn from_string(raw: &str) -> Self {
        if raw.chars().all(char::is_alphabetic) {
            return Intermediate(String::from(raw));
        } else {
            let digit: usize = raw.split_at(1).1.parse().unwrap();
            if raw.starts_with("x") {
                return ConstX(digit);
            }
            if raw.starts_with("y") {
                return ConstY(digit);
            }
            if raw.starts_with("z") {
                return OutZ(digit);
            }
        }
        panic!("Bad register: {} passed", raw);
    }

}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstX(id) => write!(f, "x{id:2}"),
            ConstY(id) => write!(f, "y{id:2}"),
            Intermediate(s) => write!(f, "{s}"),
            OutZ(id) => write!(f, "z{id:2}"),
        }
    }
}

#[derive(Clone)]
enum Gate {
    And(Register, Register),
    Or(Register, Register),
    Xor(Register, Register),
}

struct RawMachineInfo {
    x_const_lookup: Vec<bool>,
    y_const_lookup: Vec<bool>,
    int_gate_lookup: HashMap<String, Gate>,
    z_gate_lookup: Vec<Gate>
}

impl RawMachineInfo {

    fn from_fstr(fstr: &str) -> Self {

        let mut out = Self {
            x_const_lookup: Vec::new(),
            y_const_lookup: Vec::new(),
            int_gate_lookup: HashMap::new(),
            z_gate_lookup: Vec::new(),
        };

        let mut z_gate_map: HashMap<usize, Gate> = HashMap::new();

        let mut lines = fstr.lines().peekable();
        while lines.peek().unwrap().starts_with("x") {
            out.x_const_lookup.push(
                lines.next().unwrap()
                .chars().last().unwrap() == '1'
            );
        }

        while lines.peek().unwrap().starts_with("y") {
            out.y_const_lookup.push(
                lines.next().unwrap()
                .chars().last().unwrap() == '1'
            );
        }

        // Skip blank line
        lines.next();

        for gate_str in lines {
            let parts = gate_str.split_once(" -> ").unwrap();
            let out_reg = Register::from_string(parts.1);
            let mut in_parts = gate_str.split_whitespace();

            let operand1 = Register::from_string(in_parts.next().unwrap());
            let gate = match in_parts.next().unwrap() {
                "AND" => And(operand1, Register::from_string(in_parts.next().unwrap())),
                "OR" => Or(operand1, Register::from_string(in_parts.next().unwrap())),
                "XOR" => Xor(operand1, Register::from_string(in_parts.next().unwrap())),
                _ => panic!("Bad gate code"),
            };

            match out_reg {
                Intermediate(id) => out.int_gate_lookup.insert(id, gate),
                OutZ(id) => z_gate_map.insert(id, gate),
                _ => panic!("Constant passed as output of gate"),
            };
        }

        let mut i = 0;
        while let Some(g) = z_gate_map.get(&i) {
            out.z_gate_lookup.push(g.clone());
            i += 1;
        }

        return out;
    }

    fn eval_gate(&self, gate: &Gate) -> bool {
        match gate {
            And(r1, r2) => self.eval_register(r1) && self.eval_register(r2),
            Or(r1, r2) => self.eval_register(r1) || self.eval_register(r2),
            Xor(r1, r2) => self.eval_register(r1) != self.eval_register(r2),
        }
    }

    fn eval_register(&self, reg: &Register) -> bool {
        match reg {
            ConstX(id) => self.x_const_lookup[*id],
            ConstY(id) => self.y_const_lookup[*id],
            Intermediate(name) => self.eval_gate(self.int_gate_lookup.get(name).unwrap()),
            OutZ(id) => self.eval_gate(&self.z_gate_lookup[*id]),
        }
    }

    fn print_reg(&self, reg: &Register, indent: usize) {
        for _ in 0..indent {
            print!("    ");
        }
        let gate_o: Option<&Gate> = match reg {
            ConstX(_) | ConstY(_) => {
                print!("{reg}");
                None
            },
            Intermediate(s) => {
                print!("{reg}-");
                self.int_gate_lookup.get(s)
            },
            OutZ(id) => {
                print!("{reg}-");
                self.z_gate_lookup.get(*id)
            },
        };

        if let Some(gate) = gate_o {
            let (r1, r2) = match gate {
                And(r1, r2) => {
                    println!("AND");
                    (r1,r2)
                },
                Or(r1, r2) => {
                    println!("OR");
                    (r1,r2)
                },
                Xor(r1, r2) => {
                    println!("XOR");
                    (r1,r2)
                },
            };
            self.print_reg(r1, indent+1);
            self.print_reg(r2, indent+1);
        } else {
            println!();
        }
    }

    fn get_str(&self, str: &str) -> Gate {
        if str.starts_with("z") {
            let id: usize = str[1..].parse().unwrap();
            return self.z_gate_lookup[id].clone();
        } else {
            return self.int_gate_lookup.get(str).unwrap().clone();
        }
    }
}



fn part1(info: &RawMachineInfo) -> usize {

    let mut out = 0;

    for (i, gate) in info.z_gate_lookup.iter().enumerate() {
        out += if info.eval_gate(gate) {1} else {0} << i;
    }

    return out;
}

// Bro I'm sorry but I just have to restart for part 2 completely
//
// Absolutely just stole the shit out of: https://elixirforum.com/t/advent-of-code-2024-day-24/68344/5?u=bjorng
// Tried to use the SAT solver again but couldn't figure it out :(

#[derive(Hash)]
enum LogicTree {
    Xor(Box<LogicTree>, Box<LogicTree>),
    And(Box<LogicTree>, Box<LogicTree>),
    Or(Box<LogicTree>, Box<LogicTree>),
    Register(Register),
}

impl PartialEq for LogicTree {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LogicTree::Xor(t1, t2), LogicTree::Xor(t3, t4)) => (t1==t3 && t2==t4) || (t1==t4 && t2==t3),
            (LogicTree::And(t1, t2), LogicTree::And(t3, t4)) => (t1==t3 && t2==t4) || (t1==t4 && t2==t3),
            (LogicTree::Or(t1, t2), LogicTree::Or(t3, t4)) => (t1==t3 && t2==t4) || (t1==t4 && t2==t3),
            (LogicTree::Register(r1), LogicTree::Register(r2)) => r1==r2,
            _ => false,
        }
    }
}

impl Eq for LogicTree {}

fn adder(bit: usize) -> LogicTree {
    if bit==0 {
        return LogicTree::Xor(
            Box::new(LogicTree::Register(ConstX(0))),
            Box::new(LogicTree::Register(ConstY(0)))
        );
    } else {
        return LogicTree::Xor(
            Box::new(LogicTree::Xor(
                Box::new(LogicTree::Register(ConstX(bit))), 
                Box::new(LogicTree::Register(ConstY(bit)))
            )),
            Box::new(carry(bit-1))
        );
    }
}

fn carry(bit: usize) -> LogicTree {
    if bit==0 {
        return LogicTree::And(
            Box::new(LogicTree::Register(ConstX(0))),
            Box::new(LogicTree::Register(ConstY(0)))
        );
    } else {
        return LogicTree::Or(
            Box::new(LogicTree::And(
                Box::new(LogicTree::Register(ConstX(bit))), 
                Box::new(LogicTree::Register(ConstY(bit)))
            )),
            Box::new(LogicTree::And(
                Box::new(LogicTree::Xor(
                        Box::new(LogicTree::Register(ConstX(bit))), 
                    Box::new(LogicTree::Register(ConstY(bit)))
                )),
                Box::new(carry(bit-1))
            ))
        );
    }
}

fn find_swap(current: HashMap<LogicTree, String>, expected: &LogicTree, actual: &LogicTree) -> (String, String) {

    if let Some((_existing, name)) = current.iter().filter(|(k,_v)| **k==*expected).next() {
        return (name.clone(), current.get(&actual).unwrap().clone());
    }

    let (e1, e2) = match expected {
        LogicTree::Xor(t1, t2) 
        | LogicTree::And(t1, t2)
        | LogicTree::Or(t1, t2) => (t1, t2),
        LogicTree::Register(_) => panic!("Couldn't find a swap!"),
    };
    let (a1, a2) = match actual {
        LogicTree::Xor(t1, t2) 
        | LogicTree::And(t1, t2)
        | LogicTree::Or(t1, t2) => (t1, t2),
        LogicTree::Register(_) => panic!("Couldn't find a swap!"),
    };

    if **e1 == **a1 {
        return find_swap(current, e2, a2);
    } else if e1 == a2 {
        return find_swap(current, e2, a1);
    } else if e2 == a1 {
        return find_swap(current, e1, a2);
    } else if e2 == a2 {
        return find_swap(current, e1, a1);
    }

    panic!("Couldn't find swap. No conditions satisfied.");
}

struct SwappedMachineInfo<'a> {
    raw: &'a RawMachineInfo,
    swaps: HashMap<String, String>,
}

impl SwappedMachineInfo<'_> {

    fn tree_for_reg(&self, reg: &Register) -> LogicTree {
        match reg {
            ConstX(_) | ConstY(_) => LogicTree::Register(reg.clone()),
            Intermediate(name) => 
                self.tree_for_gate(&self.raw.get_str(self.swaps.get(name).unwrap_or(name))),
            OutZ(id) => 
                self.tree_for_gate(&self.raw.get_str(self.swaps.get(&zstr(*id)).unwrap_or(&zstr(*id)))),
        }
    }

    fn tree_for_gate(&self, gate: &Gate) -> LogicTree {
        match gate {
            And(r1, r2) => LogicTree::And(
                Box::new(self.tree_for_reg(r1)), 
                Box::new(self.tree_for_reg(r2))
            ),
            Or(r1, r2) => LogicTree::Or(
                Box::new(self.tree_for_reg(r1)), 
                Box::new(self.tree_for_reg(r2))
            ),
            Xor(r1, r2) => LogicTree::Xor(
                Box::new(self.tree_for_reg(r1)), 
                Box::new(self.tree_for_reg(r2))
            ),
        }
    }

    fn new<'a>(raw: &'a RawMachineInfo) -> SwappedMachineInfo<'a> {
        return SwappedMachineInfo {
            raw,
            swaps: HashMap::new(),
        }
    }
}

fn part2(info: &RawMachineInfo) -> Vec<String> {

    let mut sinfo = SwappedMachineInfo::new(info);

    for z_ind in 0..info.z_gate_lookup.len() {
        let expected = adder(z_ind);
        let actual = sinfo.tree_for_reg(&OutZ(z_ind));

        if expected != actual && z_ind != 45 {
            println!("Broken at: {z_ind}");


            let bigmap: HashMap<LogicTree, String> = 
                sinfo.raw.int_gate_lookup.keys().map(|s| (sinfo.tree_for_reg(&Intermediate(s.clone())), s.clone()))
                .chain(sinfo.raw.z_gate_lookup.iter().enumerate().map(|(id, _g)| (sinfo.tree_for_reg(&OutZ(id)), zstr(id))))
                .collect();

            let swap = find_swap(bigmap, &expected, &actual);

            println!("Swap: {swap:?}");

            sinfo.swaps.insert(swap.0.clone(), swap.1.clone());
            sinfo.swaps.insert(swap.1, swap.0);

        }

    }

    let mut out: Vec<String> = sinfo.swaps.keys().cloned()
        .collect();

    out.sort();

    return out;
}

fn zstr(id: usize) -> String {
    format!("z{:0>2}", id)
}

