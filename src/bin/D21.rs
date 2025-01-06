use std::{collections::HashMap, usize};

use aoc_util::vec2::Vec2;

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D21.txt").expect("Error Reading File!");


    let comp = part1(&fstr);
    println!("Total complexity: {comp}");

    let comp2 = part2(&fstr);
    println!("Total complexity for 25 robots: {comp2}");

}

fn push_horiz(vec: &mut Vec<char>, start: Vec2<isize>, end: Vec2<isize>) {
    if end.x > start.x {
        for _ in 0..(end.x - start.x) {
            vec.push('>');
        }
    } else if end.x < start.x {
        for _ in 0..(start.x - end.x) {
            vec.push('<');
        }
    }
}

fn push_vert(vec: &mut Vec<char>, start: Vec2<isize>, end: Vec2<isize>) {
    if end.y > start.y {
        for _ in 0..(end.y - start.y) {
            vec.push('^');
        }
    } else if end.y < start.y {
        for _ in 0..(start.y - end.y) {
            vec.push('v');
        }
    }
}

fn door_code_to_arrows(code: &str) -> Vec<char> {

    let mut pos: Vec2<isize> = Vec2::zero().signed();

    let mut arrows = Vec::new();

    for char in code.chars() {
        let dest = match char {
            '0' => Vec2 { x: -1, y: 0},
            '1' => Vec2 { x: -2, y: 1},
            '2' => Vec2 { x: -1, y: 1},
            '3' => Vec2 { x:  0, y: 1},
            '4' => Vec2 { x: -2, y: 2},
            '5' => Vec2 { x: -1, y: 2},
            '6' => Vec2 { x:  0, y: 2},
            '7' => Vec2 { x: -2, y: 3},
            '8' => Vec2 { x: -1, y: 3},
            '9' => Vec2 { x:  0, y: 3},
            'A' => Vec2 { x: 0, y: 0},
            _ => panic!("invalid character encountered"),
        };


        if dest.y == 0 && pos.x == -2 {
            push_horiz(&mut arrows, pos, dest);
            push_vert(&mut arrows, pos, dest);
        } else if pos.y == 0 && dest.x == -2 {
            push_vert(&mut arrows, pos, dest);
            push_horiz(&mut arrows, pos, dest);
        } else {

            if let Some(q) = quadrent(dest - pos) {

                match q {
                    Quad::I|Quad::IV => {
                        push_vert(&mut arrows, pos, dest);
                        push_horiz(&mut arrows, pos, dest);
                    },
                    Quad::II|Quad::III => {
                        push_horiz(&mut arrows, pos, dest);
                        push_vert(&mut arrows, pos, dest);
                    },
                }

            } else {
                push_vert(&mut arrows, pos, dest);
                push_horiz(&mut arrows, pos, dest);
            }


        }
        arrows.push('A');

        pos = dest;
    }

    return arrows;
}

enum Quad {
    I,
    II,
    III,
    IV
}

fn quadrent(v: Vec2<isize>) -> Option<Quad> {
    if v.x > 0 && v.y > 0 {
        return Some(Quad::I);
    }
    if v.x < 0 && v.y > 0 {
        return Some(Quad::II);
    }
    if v.x < 0 && v.y < 0 {
        return Some(Quad::III);
    }
    if v.x > 0 && v.y < 0 {
        return Some(Quad::IV);
    }
    None
}

fn arrows_to_arrows(code: &Vec<char>) -> Vec<char> {
    let mut pos: Vec2<isize> = Vec2::zero().signed();

    let mut arrows = Vec::new();

    for char in code {
        let dest = match char {
            '>' => Vec2 { x:  0, y: -1},
            '^' => Vec2 { x: -1, y:  0},
            '<' => Vec2 { x: -2, y: -1},
            'v' => Vec2 { x: -1, y: -1},
            'A' => Vec2 { x:  0, y:  0},
            _ => panic!("invalid character encountered"),
        };

        if dest.y == 0 && pos.x == -2 {
            push_horiz(&mut arrows, pos, dest);
            push_vert(&mut arrows, pos, dest);
        } else if pos.y == 0 && dest.x == -2 {
            push_vert(&mut arrows, pos, dest);
            push_horiz(&mut arrows, pos, dest);
        } else {

            if let Some(q) = quadrent(dest - pos) {

                match q {
                    Quad::I|Quad::IV => {
                        push_vert(&mut arrows, pos, dest);
                        push_horiz(&mut arrows, pos, dest);
                    },
                    Quad::II|Quad::III => {
                        push_horiz(&mut arrows, pos, dest);
                        push_vert(&mut arrows, pos, dest);
                    },
                }

            } else {
                push_vert(&mut arrows, pos, dest);
                push_horiz(&mut arrows, pos, dest);
            }


        }
        arrows.push('A');

        pos = dest;
    }

    return arrows;
}

fn part1(fstr: &str) -> usize {

    let mut complexity = 0;

    for l in fstr.lines() {
        let arrows1: String = door_code_to_arrows(l).into_iter().collect();
        let arrows2: String = arrows_to_arrows(&arrows1.chars().collect()).into_iter().collect();
        let arrows3: String = arrows_to_arrows(&arrows2.chars().collect()).into_iter().collect();
        println!("{l}:");
        println!("    {arrows1}");
        println!("    {arrows2}");
        println!("    {arrows3}");

        let numeric_component_s: String = l.chars().filter(|c| c.is_digit(10)).collect();
        let num: usize = numeric_component_s.parse().unwrap();
        println!("     len: {}", arrows3.len());

        complexity += num * arrows3.len();
    }

    complexity
}

fn len_to_press(arrow: char, pos: Vec2<isize>, depth: usize, memo_table: &mut HashMap<(char, Vec2<isize>, usize),usize>) -> usize {

    if let Some(len) = memo_table.get(&(arrow, pos, depth)) {
        return *len;
    }

    let sequence: Vec<char> = press_arrow_from(arrow, pos);

    if depth == 1 {
        return sequence.len();
    } else {
        // let s: String = sequence.iter().collect();
        // print!("{}", s);
        let mut len = 0;

        let mut prev_loc = Vec2::zero().signed();
        for c in sequence {
            len += len_to_press(c, prev_loc, depth-1, memo_table);
            prev_loc = dirpad_loc(c);
        }

        memo_table.insert((arrow, pos, depth), len);

        return len;
    }
}

fn dirpad_loc(c: char) -> Vec2<isize> {
    match c {
        'A' => Vec2 { x: 0, y: 0 },
        '^' => Vec2 { x: -1, y: 0 },
        '>' => Vec2 { x: 0, y: -1 },
        'v' => Vec2 { x: -1, y: -1 },
        '<' => Vec2 { x: -2, y: -1 },
        _ => panic!("invalid dirpad char")
    }
}

fn press_arrow_from(arrow: char, pos: Vec2<isize>) -> Vec<char> {

    let dest = dirpad_loc(arrow);

    let mut arrows: Vec<char> = Vec::new();
    if dest.y == 0 && pos.x == -2 {
        push_horiz(&mut arrows, pos, dest);
        push_vert(&mut arrows, pos, dest);
    } else if pos.y == 0 && dest.x == -2 {
        push_vert(&mut arrows, pos, dest);
        push_horiz(&mut arrows, pos, dest);
    } else {

        if let Some(q) = quadrent(dest - pos) {

            match q {
                Quad::I|Quad::IV => {
                    push_vert(&mut arrows, pos, dest);
                    push_horiz(&mut arrows, pos, dest);
                },
                Quad::II|Quad::III => {
                    push_horiz(&mut arrows, pos, dest);
                    push_vert(&mut arrows, pos, dest);
                },
            }

        } else {
            push_vert(&mut arrows, pos, dest);
            push_horiz(&mut arrows, pos, dest);
        }


    }
    arrows.push('A');

    return arrows;
}

fn part2(fstr: &str) -> usize {

    let mut complexity = 0;

    let mut memo_table = HashMap::new();

    for l in fstr.lines() {

        let arrows = door_code_to_arrows(l);

        let mut len = 0;
        let mut prev_loc = Vec2::zero().signed();
        for c in arrows {
            len += len_to_press(c, prev_loc, 25, &mut memo_table);
            prev_loc = dirpad_loc(c);
        }

        let numeric_component_s: String = l.chars().filter(|c| c.is_digit(10)).collect();
        let num: usize = numeric_component_s.parse().unwrap();

        println!("len of {} is {}", l, len);
        complexity += num * len;
    }

    complexity
}
