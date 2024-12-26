mod interpreter;
mod sat_solve;
use crate::interpreter::*;
use crate::sat_solve::*;

use splr::*;


fn main() {

    let fstr: String = std::fs::read_to_string("./input/D17.txt").expect("Error Reading File!");

    let mut lines = fstr.lines();

    let a_init: usize = lines.next().unwrap().split(' ').last().unwrap().parse().unwrap();
    let b_init: usize = lines.next().unwrap().split(' ').last().unwrap().parse().unwrap();
    let c_init: usize = lines.next().unwrap().split(' ').last().unwrap().parse().unwrap();
    lines.next();

    let instructions: Vec<usize> = lines.next().unwrap()
        .split(' ').last().unwrap()
        .split(',').map(|n| n.parse().unwrap())
        .collect();

    let state_init = CpuState {
        a: a_init,
        b: b_init,
        c: c_init,
    };

    part1(&state_init, &instructions);

    println!("Instructions: {:?}", instructions);

    let a_special = part2(&instructions);

    let state2_init = CpuState {
        a: a_special,
        b: b_init,
        c: c_init,
    };
    part1(&state2_init, &instructions);

}
