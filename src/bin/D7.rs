
use crate::Operator::*;

struct Equation {
    expected: usize,
    vals: Vec<usize>,
}

impl Equation {

    fn from_line(fstr: &str) -> Self {
        
        let mut split = fstr.split(": ");

        Equation {
            expected: 
                split.next().unwrap().parse().unwrap(),
            vals: 
                split.next().unwrap()
                    .split(" ")
                    .map(|s| s.parse().unwrap())
                    .collect(),
        }

    }

}

impl std::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let eqstr: String = self.vals.iter().fold(String::new(), |st, x| st + &x.to_string() + " " );
        write!(f, "{}: {}", self.expected, &eqstr[0..eqstr.len()-1])
    }
}

enum Operator {
    ADD,
    MUL,
    CAT,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ADD => write!(f, "+"),
            MUL => write!(f, "*"),
            CAT => write!(f, "||"),
        }
    }
}

impl Operator {

    fn execute(&self, a: usize, b: usize) -> usize {
        match self {
            ADD => a + b,
            MUL => a * b,
            CAT => {
                (a.to_string() + &b.to_string()).parse().unwrap()
                // let digits = b.ilog10() + 1;
                // a * 10_usize.pow(digits) + b
            }
        }
    }

}

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D7.txt").expect("Error Reading File!");

    let equations: Vec<Equation> = fstr.lines()
        .map(Equation::from_line)
        .collect();

    // for eq in equations.iter() {
    //     println!("{}", eq);
    // }

    // dbg!(CAT.execute(12390,999799));

    part1(&equations);
    part2(&equations);
}

fn potentially_valid(eq: &Equation, total: usize, op_ind: usize, ops_to_check: &[Operator]) -> bool {

    if op_ind == eq.vals.len()-1 {
        return false;
    }

    for op in ops_to_check {

        let new_total = op.execute(total, eq.vals[op_ind+1]);

        if new_total == eq.expected && op_ind+1 == eq.vals.len()-1 || 
                potentially_valid(eq, new_total, op_ind+1, ops_to_check) { 
            return true;
        }

    }

    return false;
}

fn part1(eqs: &Vec<Equation>) {

    let mut total = 0;

    let ops_to_check = &[ADD, MUL];

    for eq in eqs {
        if potentially_valid(eq, eq.vals[0], 0, ops_to_check) {
            total += eq.expected;
        }
    }

    println!("Total of potentially valid eqs.: {}", total);

}
fn part2(eqs: &Vec<Equation>) {

    let mut total = 0;

    let ops_to_check = &[ADD, MUL, CAT];

    for eq in eqs {
        if potentially_valid(eq, eq.vals[0], 0, ops_to_check) {
            total += eq.expected;
        }
    }

    println!("Total of potentially valid eqs. with cat: {}", total);

}
