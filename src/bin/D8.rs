use std::collections::{HashMap, HashSet};

struct AntennaMap {
    width: usize,
    height: usize,

    antenna_locs: HashMap<char, Vec<(i32, i32)>>,
}

impl AntennaMap {

    fn from_fstr(fstr: &String) -> Self {

        let mut antenna_locs: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

        for (y, line) in fstr.lines().enumerate() {
            for (x, c) in line.chars().enumerate().filter(|(_,c)| *c!='.') {

                let c_locs: &mut Vec<(i32, i32)> = match antenna_locs.get_mut(&c) {
                    Some(v) => v,
                    None => {
                        let v = Vec::new();
                        antenna_locs.insert(c, v);
                        antenna_locs.get_mut(&c).unwrap()
                    },
                };

                c_locs.push((x as i32, y as i32));
            }
        }

        AntennaMap {
            width: fstr.lines().next().unwrap().len(),
            height: fstr.lines().count(),

            antenna_locs
        }
    }

    fn in_bounds(&self, p: (i32, i32)) -> bool {
        p.0 >= 0 && p.1 >= 0 &&
        p.0 < self.width as i32 && p.1 < self.height as i32
    }

}

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D8.txt").expect("Error Reading File!");

    let antenna_map = AntennaMap::from_fstr(&fstr);

    part1(&antenna_map);
    part2(&antenna_map);
}

fn part1(am: &AntennaMap) {

    let mut potential_antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_c, v) in am.antenna_locs.iter() {
        for (i, a1) in v.iter().enumerate() {
            for a2 in v.iter().skip(i+1) {

                let x_diff = a2.0 - a1.0;
                let y_diff = a2.1 - a1.1;

                potential_antinodes.insert(
                    ((a2.0 + x_diff),
                    (a2.1 + y_diff))
                );

                potential_antinodes.insert(
                    ((a1.0 - x_diff),
                    (a1.1 - y_diff))
                );
            }
        }
    }

    let inbounds: Vec<_> = potential_antinodes.into_iter()
        .filter(|p| am.in_bounds(*p))
        .collect();



    let ib_cnt = inbounds.len();

    println!("Count of in-bounds antinodes: {}", ib_cnt);

}

fn part2(am: &AntennaMap) {

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_c, v) in am.antenna_locs.iter() {
        for (i, a1) in v.iter().enumerate() {
            for a2 in v.iter().skip(i+1) {

                let x_diff = a2.0 - a1.0;
                let y_diff = a2.1 - a1.1;

                let mut current_point = *a2;

                while am.in_bounds(current_point) {

                    antinodes.insert(current_point);

                    current_point =
                        ((current_point.0 + x_diff),
                         (current_point.1 + y_diff));
                }

                current_point = *a1;

                while am.in_bounds(current_point) {

                    antinodes.insert(current_point);

                    current_point = 
                        ((current_point.0 - x_diff),
                         (current_point.1 - y_diff));
                }
            }
        }
    }

    let ib_cnt = antinodes.iter().count();

    println!("Count of in-bounds antinodes (accounting for resonant harmonics): {}", ib_cnt);

}
