use std::{collections::HashMap, hash::Hash, usize};

use regex::bytes::Regex;

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D19.txt").expect("Error Reading File!");

    let good_cnt = part1(&fstr);
    println!("Number of possible patterns: {}", good_cnt);

    let total_cnt = part2(&fstr);
    println!("Total count of all combinations: {}", total_cnt);
}

fn part1(fstr: &str) -> usize {

    let mut lines = fstr.lines();

    let l1 = lines.next().unwrap();

    let mut r_str: String = String::new();
    r_str.push_str("^(");
    r_str.push_str(&l1.replace(", ", "|"));
    r_str.push_str(")+$");

    let rgx = Regex::new(&r_str).unwrap();

    lines.next();

    let mut cnt = 0;
    for l in lines {
        if rgx.is_match(l.as_bytes()) {
            cnt += 1;
        }
    }

    cnt
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Color {
    WHITE,
    BLUE,
    BLACK,
    RED,
    GREEN
}

impl Hash for Color {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) -> () {
        match self {
            Color::WHITE => 0.hash(state),
            Color::BLUE => 1.hash(state),
            Color::BLACK => 2.hash(state),
            Color::RED => 3.hash(state),
            Color::GREEN => 4.hash(state),
        }
    }
}

impl Color {
    fn from_char(c: char) -> Self {
        match c {
            'w' => Color::WHITE,
            'u' => Color::BLUE,
            'b' => Color::BLACK,
            'r' => Color::RED,
            'g' => Color::GREEN,
            _ => panic!("Invalid color code"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Color::WHITE => 'w',
            Color::BLUE => 'u',
            Color::BLACK => 'b',
            Color::RED => 'r',
            Color::GREEN => 'g',
        }
    }
}

fn print_fa(fa: &Vec<HashMap<Color, Vec<usize>>>) {
    let t_from0: Vec<(Color, usize)> = fa[0].iter()
        .flat_map(|(c,s_v)| {
            s_v.iter().map(|s| {
                (*c, *s)
            })
        }).collect();

    for (c, s) in t_from0 {
        let mut n_state = s;
        let mut tran_c = c;
        print!("0");
        loop {
            print!("-{}->{}", tran_c.to_char(), n_state);
            if n_state == 0 {
                break;
            }

            let tran = fa[n_state].iter().next().unwrap();
            tran_c = *tran.0;
            n_state = tran.1[0];
        }
        println!("");
    }
}

fn part2(fstr: &str) -> usize {
    let mut lines = fstr.lines();


    // Make towel finite automata
    let towels: Vec<Vec<Color>> = lines.next().unwrap()
        .split(", ")
        .map(|ts| {
            ts.chars().map(Color::from_char).collect()
        })
        .collect();

    let mut fa_states: Vec<HashMap<Color, Vec<usize>>> = Vec::new();
    fa_states.push(HashMap::new());

    for towel in towels {
        let mut state = 0;
        for (i,c) in towel.iter().enumerate() {
            let mut n_state = 0;

            if i != towel.len() - 1 { 
                n_state = fa_states.len();
                fa_states.push(HashMap::new());
            };

            if let Some(v) = fa_states[state].get_mut(c) {
                v.push(n_state);
            } else {
                fa_states[state].insert(*c, vec![n_state]);
            }

            state = n_state;
        }
    }


    // print_fa(&fa_states);

    // println!("Number of fa states: {}", fa_states.len());


    // Actually parse the patterns

    lines.next(); // skip blank separator
    
    return lines.fold(0, |cnt, l| {

        // println!("Checkign line: {}", l);

        let patt = l.chars().map(Color::from_char);

        let mut state_counts: HashMap<usize, usize> = HashMap::new();
        state_counts.insert(0, 1);

        for c in patt {
            let mut n_state: HashMap<usize, usize> = HashMap::new();

            for (s,s_cnt) in state_counts.iter() {

                if let Some(n_v) = fa_states[*s].get(&c) {
                    // Take those transitions, adding their dests to the next state
                    for n in n_v {

                        // If we already have a next state, just add to its count
                        if let Some(n_cnt) = n_state.get_mut(n) {
                            *n_cnt += *s_cnt;
                        } else {
                            // Otherwise, create that next state
                            n_state.insert(*n, *s_cnt);
                        }
                    }
                }

            }

            state_counts = n_state;
        };

        cnt + *state_counts.get(&0).unwrap_or(&0)
    });
}
