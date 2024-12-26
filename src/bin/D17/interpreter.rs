use crate::interpreter::InstructionKind::*;


#[derive(Clone, Copy, Debug)]
pub struct CpuState {
    pub a: usize,
    pub b: usize,
    pub c: usize
}

#[derive(Debug)]
enum InstructionKind {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV
}

impl InstructionKind {
    fn from_int(i: usize) -> Self {
        match i {
            0 => ADV,
            1 => BXL,
            2 => BST,
            3 => JNZ,
            4 => BXC,
            5 => OUT,
            6 => BDV,
            7 => CDV,
            _ => panic!("Bad opcode"),
        }
    }
}

fn combo_op_to_num(op: usize, state: &CpuState) -> usize {
    match op {
        0..=3 => op,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => panic!("Invalid combo op"),
    }
}

// Returns new location of instruction pointer
fn execute_instr(instr_ptr: usize, instructions: &[usize], state: &mut CpuState) -> usize {
    let instr = InstructionKind::from_int(instructions[instr_ptr]);
    let op = instructions[instr_ptr + 1];

    // println!("Executing: {:?}", instr);

    match instr {
        ADV => {
            state.a >>= combo_op_to_num(op, state);
        },
        BXL => {
            state.b ^= op;
        },
        BST => {
            state.b = combo_op_to_num(op, state) % 8;
        },
        JNZ => {
            if state.a != 0 {
                return op;
            }
        }
        BXC => {
            state.b ^= state.c;
        },
        OUT => {
            print!("{},", combo_op_to_num(op, state) % 8);
        },
        BDV => {
            state.b = state.a >> combo_op_to_num(op, state);
        },
        CDV => {
            state.c = state.a >> combo_op_to_num(op, state);
        }
    }


    return instr_ptr + 2;
}

pub fn part1(state_init: &CpuState, instructions: &[usize]) {
    let mut instr_ptr = 0;
    let mut state = state_init.clone();

    while instr_ptr < instructions.len() {
        instr_ptr = execute_instr(instr_ptr, instructions, &mut state);
        // dbg!(state);
    }

    println!("\nProgram Halted!");
}

