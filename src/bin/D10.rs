use std::{collections::VecDeque, usize};


struct TopoMap {
    height: usize,
    width: usize,
    map: Vec<Vec<u8>>
}

struct VisitedMap {
    map: Vec<Vec<bool>>
}

struct AncestorMap {
    map: Vec<Vec<usize>>
}

impl TopoMap {

    fn from_fstr(fstr: String) -> TopoMap {
        let mut map: Vec<Vec<u8>> = Vec::new();

        for line in fstr.lines() {
            let mut row: Vec<u8> = Vec::new();
            for char in line.chars() {
                row.push(char.to_digit(10).unwrap() as u8);
            }
            map.push(row);
        }

        TopoMap {
            height: map.len(),
            width: map[0].len(),
            map
        }
    }

    fn move_from(&self, from: (usize, usize), dir: usize) -> Option<(u8, (usize, usize))> {

        let n_loc = match dir {
            0 => if from.0 + 1 >= self.width {
                return None;
            } else {
                (from.0 + 1, from.1)
            },

            1 => if from.1 <= 0 {
                return None;
            } else {
                (from.0 , from.1 - 1)
            },

            2 => if from.0 <= 0 {
                return None;
            } else {
                (from.0 - 1, from.1)
            },

            3 => if from.1 + 1 >= self.height {
                return None;
            } else {
                (from.0, from.1 + 1)
            },

            _ => panic!("Bad dir"),
        };

        Some((self.get_at_pos(n_loc), n_loc))
    }

    fn get_at_pos(&self, pos: (usize, usize)) -> u8 {
        return self.map[pos.0][pos.1];
    }

    fn new_visit_map(&self) -> VisitedMap {
        VisitedMap {
            map: vec![vec![false ; self.width] ; self.height]
        }
    }

    fn new_ancestor_map(&self) -> AncestorMap {
        AncestorMap {
            map: vec![vec![0 ; self.width] ; self.height]
        }
    }
}

impl VisitedMap {
    
    fn mark_visited(&mut self, pos: (usize, usize)) {
        self.map[pos.0][pos.1] = true;
    }

    fn is_visited(&self, pos: (usize, usize)) -> bool {
        self.map[pos.0][pos.1]
    }

}

impl AncestorMap {
    
    fn set_n_a(&mut self, pos: (usize, usize), n_a: usize) {
        self.map[pos.0][pos.1] = n_a;
    }

    fn get_n_a(&self, pos: (usize, usize)) -> usize {
        self.map[pos.0][pos.1]
    }

}

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D10.txt").expect("Error Reading File!");

    let map = TopoMap::from_fstr(fstr);

    part1(&map);
    part2(&map);
}

fn get_score(map: &TopoMap, visited: &mut VisitedMap, pos: (usize, usize), height: u8) -> usize {
    visited.mark_visited(pos);

    if height == 9 {
        return 1;
    }

    let mut sum = 0;

    for dir in 0..=3 {

        let (n_height, n_pos) = match map.move_from(pos, dir) {
            Some(x) => x,
            None => continue,
        };

        if  n_height == height + 1 && !visited.is_visited(n_pos) {
            sum += get_score(map, visited, n_pos, n_height);
        }
    }

    sum
}

fn part1(map: &TopoMap) {

    let mut total_score = 0;

    for (r,row) in map.map.iter().enumerate() {
        for (c,h) in row.iter().enumerate() {
            if *h == 0 {
                total_score += get_score(map, &mut map.new_visit_map(), (r, c), *h);
            }
        }
    }

    println!("Total trailhead score: {}", total_score);

}

fn part2(map: &TopoMap) {

    let mut to_eval: VecDeque<(usize, usize)> = VecDeque::with_capacity(map.height * map.width / 2);

    let mut total_score = 0;

    for (r,row) in map.map.iter().enumerate() {
        for (c,h) in row.iter().enumerate() {

            if *h != 0 {
                continue;
            }

            // println!("Going for: {}{}", r, c);

            to_eval.clear();
            to_eval.push_back((r,c));

            let mut a_map = map.new_ancestor_map();
            a_map.set_n_a((r,c), 1);


            let mut head_rating = 0;

            while to_eval.len() > 0 {
                let pt = to_eval.pop_front().unwrap();
                let pt_h = map.get_at_pos(pt);
                let pt_n_a = a_map.get_n_a(pt);

                if pt_h == 9 {
                    head_rating += pt_n_a;
                }

                for dir in 0..=3 {
                    let (n_height, n_pos) = match map.move_from(pt, dir) {
                        Some(x) => x,
                        None => continue,
                    };

                    if  n_height == pt_h + 1 {
                        a_map.set_n_a(n_pos, a_map.get_n_a(n_pos) + pt_n_a);

                        if !to_eval.contains(&n_pos) {
                            to_eval.push_back(n_pos);
                        }

                    }
                }

            }
            // println!("  Rating: {}", head_rating);
            total_score += head_rating;

        }
    }


    println!("Total trailhead score using updated formula: {}", total_score);


}
