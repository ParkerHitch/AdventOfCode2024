
use crate::RegexState::*;

enum RegexState {
    D,
    O,
    N,
    APOSTROPHE,
    T,
    OPEN_D,
    M,
    U,
    L,
    OPEN_M,
    D1_1,
    D1_2,
    D1_3,
    COMMA,
    D2_1,
    D2_2,
    D2_3,
    CLOSE
}

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D3.txt").expect("Error Reading File!");
    part1(fstr.clone());
    part2(fstr);

}

fn part1(fstr: String) {

    let mut total: usize = 0;
    let mut state: RegexState = CLOSE;
    let mut num1: usize = 0;
    let mut num2: usize = 0;

    for c in fstr.chars() {
        state = match (state, c) {
            (CLOSE, 'm') => M,
            (M, 'u') => U,
            (U, 'l') => L,
            (L, '(') => OPEN_M,
            (OPEN_M, '0'..='9') => {
                num1 *= 10;
                num1 += c.to_digit(10).unwrap() as usize;
                D1_1
            },
            (D1_1, '0'..='9') => {
                num1 *= 10;
                num1 += c.to_digit(10).unwrap() as usize;
                D1_2
            },
            (D1_2, '0'..='9') => {
                num1 *= 10;
                num1 += c.to_digit(10).unwrap() as usize;
                D1_3
            },
            (D1_1|D1_2|D1_3, ',') => COMMA,
            (COMMA, '0'..='9') => {
                num2 *= 10;
                num2 += c.to_digit(10).unwrap() as usize;
                D2_1
            },
            (D2_1, '0'..='9') => {
                num2 *= 10;
                num2 += c.to_digit(10).unwrap() as usize;
                D2_2
            },
            (D2_2, '0'..='9') => {
                num2 *= 10;
                num2 += c.to_digit(10).unwrap() as usize;
                D2_3
            },
            (D2_1|D2_2|D2_3, ')') => {
                total += num1 * num2;
                num1 = 0;
                num2 = 0;
                CLOSE
            }
            _ => {
                num1 = 0;
                num2 = 0;
                CLOSE
            }
        }
    }
    

    println!("Total: {}", total);
}

fn part2(fstr: String) {

    let mut total: usize = 0;
    let mut state: RegexState = CLOSE;
    let mut enabled: bool = true;
    let mut num1: usize = 0;
    let mut num2: usize = 0;

    for c in fstr.chars() {
        state = match (enabled, state, c) {
            (true, CLOSE, 'm') => M,
            (.., M, 'u') => U,
            (.., U, 'l') => L,
            (.., L, '(') => OPEN_M,
            (.., OPEN_M, '0'..='9') => {
                num1 *= 10;
                num1 += c.to_digit(10).unwrap() as usize;
                D1_1
            },
            (.., D1_1, '0'..='9') => {
                num1 *= 10;
                num1 += c.to_digit(10).unwrap() as usize;
                D1_2
            },
            (.., D1_2, '0'..='9') => {
                num1 *= 10;
                num1 += c.to_digit(10).unwrap() as usize;
                D1_3
            },
            (.., D1_1|D1_2|D1_3, ',') => COMMA,
            (.., COMMA, '0'..='9') => {
                num2 *= 10;
                num2 += c.to_digit(10).unwrap() as usize;
                D2_1
            },
            (.., D2_1, '0'..='9') => {
                num2 *= 10;
                num2 += c.to_digit(10).unwrap() as usize;
                D2_2
            },
            (.., D2_2, '0'..='9') => {
                num2 *= 10;
                num2 += c.to_digit(10).unwrap() as usize;
                D2_3
            },
            (.., D2_1|D2_2|D2_3, ')') => {
                total += num1 * num2;
                num1 = 0;
                num2 = 0;
                CLOSE
            }

            (.., CLOSE, 'd') => D,
            (.., D, 'o') => O,

            (false, O, '(') => OPEN_D,

            (true, O, 'n') => N,
            (.., N, '\'') => APOSTROPHE,
            (.., APOSTROPHE, 't') => T,
            (.., T, '(') => OPEN_D,

            (.., OPEN_D, ')') => {
                enabled = !enabled;
                CLOSE
            }
            _ => {
                num1 = 0;
                num2 = 0;
                CLOSE
            }
        }
    }
    

    println!("Total (only enabled): {}", total);
}
