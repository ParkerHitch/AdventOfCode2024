use std::{fmt::{Debug, Display}, isize, usize};

use crate::RaceTile::*;
use aoc_util::{map2d::Map2D, vec2::Vec2};

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D20.txt").expect("Error Reading File!");

    let map = RaceMap::from_fstr(&fstr);

    let count_100 = part1(&map);
    println!("Count of cheats that save at least 100 picoseconds: {}", count_100);

    let count_100_new = part2(&map);
    println!("Count of cheats that save at least 100 picoseconds (up to 20 moves): {}", count_100_new);
}

#[derive(PartialEq, Eq)]
enum RaceTile {
    OPEN(usize),
    BLOCKED
}

impl Display for RaceTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OPEN(_) => ".",
            BLOCKED => "#",
        })
    }
}

impl Debug for RaceTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OPEN(x) => write!(f, "{:4}", x),
            BLOCKED => write!(f, "#   "),
        }
    }
}

struct RaceMap {
    start: Vec2<usize>,
    end: Vec2<usize>,
    map: Map2D<RaceTile>
}

impl RaceMap {
    fn from_fstr(fstr: &str) -> Self {
        let mut start = Vec2::zero();
        let mut end = Vec2::zero();

        let mut opencnt = 0;

        let mut map = Map2D::from_chars(fstr.lines(), |ch, (r,c)| {
            match ch {
                '.' => {
                    opencnt += 1;
                    OPEN(0)},
                '#' => BLOCKED,
                'S' => {
                    start = Vec2{ x: c, y: r};
                    OPEN(0)
                },
                'E' => {
                    end = Vec2{ x: c, y: r};
                    OPEN(0)
                },
                _ => panic!("Invalid char encountered!"),
            }
        }).expect("Mapgen unexpectedly failed :(");


        let mut pos = start;
        let mut dist = 0;
        let mut p_dir = 4;
        'outer: while pos != end {
            for dir in 0..=3 {
                if (dir+2)%4 == p_dir {
                    continue;
                }
                if let Some(n_pos) = map.bounded_move_in_dir(pos, dir) {
                    if map.get_v(n_pos) == OPEN(0) {
                        dist += 1;
                        p_dir = dir;
                        pos = n_pos;
                        map.set_v(n_pos, OPEN(dist));
                        continue 'outer;
                    }
                }
            }
            panic!("Dead end found at: {pos}");
        }

        return Self {
            start, end, map
        };
    }
}

const MIN_SAVE: usize = 100;

fn part1(map: &RaceMap) -> usize {

    let mut pos = map.start;

    let mut good_count = 0;

    while pos != map.end {

        let mut next_dir = 0;
        let current_val: usize = match map.map.get_v(pos) {
            OPEN(x) => x,
            BLOCKED => unreachable!("Setup got screwed up"),
        };

        for d in 0..=3 {
            if let Some(w_pos) = map.map.bounded_move_in_dir(pos, d) {
                match map.map.get_v(w_pos) {
                    BLOCKED => {
                        if let Some(dest_pos) = map.map.bounded_move_in_dir(w_pos, d) {
                            if let OPEN(dest_val) = map.map.get_v(dest_pos) {
                                let skip_val: i32 = (dest_val as i32) - (current_val as i32);
                                if skip_val - 2 >= (MIN_SAVE as i32) {
                                    good_count += 1;
                                    // println!("Found! {} to {}", pos, dest_pos);
                                }
                            }
                        }
                    } 
                    OPEN(n_val) => {
                        if (n_val as i32) - (current_val as i32) == 1 {
                            next_dir = d;
                        }
                    }
                }
            }
        }

        pos = map.map.bounded_move_in_dir(pos, next_dir).unwrap();
    }

    return good_count;
}

struct DiamondIter {
    curr: Option<Vec2<isize>>,
    dir: Vec2<isize>,
    radius: isize
}

impl Iterator for DiamondIter {
    type Item = Vec2<isize>;

    fn next(&mut self) -> Option<Self::Item> {

        let curr = self.curr;

        if self.curr == None {
            return None;
        }

        let next = self.curr.unwrap() + self.dir;

        if next.x == self.radius {
            self.curr = None;
        } else {
            if next.y == self.radius {
                self.dir = Vec2 { x: -1, y: -1 };
            } else if next.x == -self.radius {
                self.dir = Vec2 { x: 1, y: -1 };
            } else if next.y == -self.radius {
                self.dir = Vec2 { x: 1, y: 1 };
            }
            self.curr = Some(next);
        }

        return curr;
    }
}

impl DiamondIter {
    fn new(r: usize) -> Self {
        return Self {
            curr: Some(Vec2 { x: r as isize, y: 0}),
            dir: Vec2 { x: -1, y: 1},
            radius: r as isize
        }
    }
}

fn part2(map: &RaceMap) -> usize {

    let mut pos = map.start;
    let mut good_cnt = 0;

    while pos != map.end {

        // println!("Checking: {pos}");

        let start_val = match map.map.get_v(pos) {
            OPEN(x) => x,
            BLOCKED => panic!(),
        };
    

        let ipos: Vec2<isize> = Vec2{ x: pos.x as isize, y: pos.y as isize};

        let mut n_pos = Vec2::zero();

        for n_cand in DiamondIter::new(1)
            .map(|v| ipos + v) {
            // println!("cand: {n_cand}");
            if let OPEN(c_val) = map.map.get(n_cand.y as usize, n_cand.x as usize) {
                if (c_val as isize) - (start_val as isize) == 1 {
                    n_pos = n_cand.unsigned();
                }
            }
        }

        for r in 2..=20 {
            // println!("  r = {r}");
            good_cnt += DiamondIter::new(r)
                    .map(|v| ipos + v)
                    .filter(|p| map.map.in_bounds_iv(*p))
                    .map(|p: Vec2<isize>| Vec2::unsigned(&p))
                    .filter_map(|p| match map.map.get_v(p) {
                        OPEN(x) => Some(x as isize - r as isize - start_val as isize),
                        BLOCKED => None,
                    })
                    .filter(|s| *s >= MIN_SAVE as isize)
                    .count();
        }

        pos = n_pos;
    }


    good_cnt

}
