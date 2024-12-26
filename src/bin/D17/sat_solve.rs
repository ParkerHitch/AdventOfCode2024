use std::usize;
use crate::interpreter::{self, part1};

use splr::*;
// use crate::interpreter::*;

enum Eqality {
    EQUAL,
    NOT_EQUAL,
}

// fn add_constraint(constraints: &mut Vec<Vec<i32>>, a: usize, b: usize, k: Eqality) {
//     match k {
//         Eqality::EQUAL => {
//             println!("{} == {}", a, b);
//             constraints.push(vec![a as i32, -(b as i32)]);
//             constraints.push(vec![b as i32, -(a as i32)]);
//         }
//         Eqality::NOT_EQUAL => {
//             println!("{} != {}", a, b);
//             constraints.push(vec![a as i32, b as i32]);
//             constraints.push(vec![-(a as i32), -(b as i32)]);
//         }
//     }
// }

fn add_constraint(constraints: &mut Vec<Vec<i32>>, b_ptr: i32, b_bit: usize, k: Eqality) {
    for b_val in 0..8 {

        let shift_amt = b_val ^ 1;

        let b_bit_ptr: i32 = b_ptr + b_bit as i32;
        let c_bit_ptr: i32 = b_ptr + (b_bit + shift_amt) as i32;

        // selected
        let s1: bool =  (b_val % 2) == 1;
        let s2: bool = ((b_val % 4) >> 1) == 1;
        let s3: bool =  (b_val >> 2) == 1;
        // println!("{}, {}, {}", s3, s2, s1);

        match k {
            Eqality::EQUAL => {
                // println!("{} == {}", a, b);
                constraints.push(vec![
                    if s1 { -b_ptr } else { b_ptr },
                    if s2 { -(b_ptr+1) } else { b_ptr+1 },
                    if s3 { -(b_ptr+2) } else { b_ptr+2 },
                    c_bit_ptr, -b_bit_ptr]);
                constraints.push(vec![
                    if s1 { -b_ptr } else { b_ptr },
                    if s2 { -(b_ptr+1) } else { b_ptr+1 },
                    if s3 { -(b_ptr+2) } else { b_ptr+2 },
                    b_bit_ptr, -c_bit_ptr]);
            }
            Eqality::NOT_EQUAL => {
                // println!("{} != {}", a, b);
                constraints.push(vec![
                    if s1 { -b_ptr } else { b_ptr },
                    if s2 { -(b_ptr+1) } else { b_ptr+1 },
                    if s3 { -(b_ptr+2) } else { b_ptr+2 },
                    b_bit_ptr, c_bit_ptr]);
                constraints.push(vec![
                    if s1 { -b_ptr } else { b_ptr },
                    if s2 { -(b_ptr+1) } else { b_ptr+1 },
                    if s3 { -(b_ptr+2) } else { b_ptr+2 },
                    -b_bit_ptr, -c_bit_ptr]);
            }
        }
    }
}
pub fn part2(instructions: &[usize]) -> usize {

    let mut constraints: Vec<Vec<i32>> = Vec::new();

    let mut A_ind: usize = 1;

    for ins in instructions {
        let xor_mask = 1 ^ 5 ^ ins;
        // print!("xor_mask for {}:", ins);
        for i in 0..3 {
            // 1 = NOT_EQUAL, 0 = Equal
            if (xor_mask & (1<<i)) > 0 {
                // print!(" {}", 1);
                add_constraint(&mut constraints, A_ind as i32, i, Eqality::NOT_EQUAL);
            } else {
                // print!(" {}", 0);
                add_constraint(&mut constraints, A_ind as i32, i, Eqality::EQUAL);
            }
        }
        // println!("\n  = {}", xor_mask);
        A_ind += 3;
    }

    for i in A_ind..=64 {
        constraints.push(vec![-(i as i32)]);
    }


    let ans_opt: Option<Vec<i32>> = 
    match Certificate::try_from(constraints.clone()) {
        Ok(Certificate::SAT(ans)) => {
            println!("SATISFIABLE: {:?}", ans);
            Some(ans)
        },
        Ok(Certificate::UNSAT) => {
            println!("UNSATISFIABLE");
            None
        },
        Err(e) => {
            panic!("UNKNOWN; {}", e);
        },
    };

    if let Some(ans) = ans_opt {

        let mut A_val: usize = 0;
        for i in ans {
            if i>0 {
                A_val += 1<<(i-1);
            }
        }

        // println!("Initial A register value: {}", A_val);

        let mut min_a = A_val;

        for i in 0..64 {
            if ((A_val >> i) % 2) == 0 {
                let mut new_vec = constraints.clone();
                new_vec.push(vec![-(i+1)]);
                if let Ok(Certificate::SAT(ans)) = Certificate::try_from(new_vec) {
                    let mut new_a: usize = 0;
                    for i in ans {
                        if i>0 {
                            new_a += 1<<(i-1);
                        }
                    }
                    // println!("New lowest: {}", new_a);
                    if new_a < min_a {
                        min_a = new_a;
                    }
                    // part1(&interpreter::CpuState{
                    //     a: new_a,
                    //     b: 0, c: 0
                    // }, instructions);
                }
            }
        }

        println!("True min: {}", min_a);
        return min_a;
    }

    return 0;

}
