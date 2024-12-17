use std::collections::HashMap;
use std::usize;

use crate::TileKind::*;
use crate::RobotInstruction::*;
use aoc_util::vec2::Vec2;

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D15.txt").expect("Error Reading File!");

    let puz_in = PuzzleInput::from_fstr(fstr);

    part1(&puz_in);
    part2(&puz_in);
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum TileKind {
    EMPTY,
    WALL,
    BOX(usize),
    WBOX(usize)
}

enum RobotInstruction {
    RIGHT,
    UP,
    LEFT,
    DOWN
}

impl RobotInstruction {
    fn move_vec_in(&self, from: Vec2<usize>) -> Vec2<usize> {
        match self {
            RIGHT => Vec2 {
                x: from.x + 1,
                y: from.y
            },
            UP => Vec2 {
                x: from.x,
                y: from.y - 1,
            },
            LEFT => Vec2 {
                x: from.x - 1,
                y: from.y
            },
            DOWN => Vec2 {
                x: from.x,
                y: from.y + 1
            },
        }
    }
}

#[derive(Clone)]
struct WarehouseMap {
    width: usize,
    height: usize,
    map: Vec<TileKind>,
    box_locs: HashMap<usize, Vec2<usize>>
}

impl WarehouseMap {
    fn get(&self, r: usize, c: usize) -> TileKind {
        self.map[r*self.width + c]
    }
    fn get_v(&self, v: Vec2<usize>) -> TileKind {
        self.get(v.y, v.x)
    }
    fn set_v(&mut self, v: Vec2<usize>, t: TileKind) {
        self.map[v.y*self.width + v.x] = t;
    }

    fn make_wide(&self) -> WarehouseMap {
        let mut map: Vec<TileKind> = Vec::with_capacity(self.map.len()*2);
        let box_locs: HashMap<usize, Vec2<usize>> = self.box_locs.iter()
            .map(|(k,v)| (*k, Vec2 { x: v.x*2, y: v.y }))
            .collect();
        for tk in self.map.iter() {
            match tk {
                BOX(id) => {
                    map.push(WBOX(*id));
                    map.push(WBOX(*id));
                },
                v => {
                    map.push(*v);
                    map.push(*v);
                }
            }
        }

        WarehouseMap {
            width: self.width*2,
            height: self.height,
            map,
            box_locs,
        }
    }

    fn print_box_locs(&self) {
        for (k,v) in self.box_locs.iter() {
            println!("Box {} at: {}", k, v);
        }
    }
}

impl std::fmt::Display for WarehouseMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                let res = write!(f, "{}", match self.get(r, c) {
                    EMPTY => '.',
                    WALL => '#',
                    BOX(_) => 'O',
                    WBOX(id) => if self.box_locs.get(&id).is_some_and(|p| p.x == c) {
                        '['
                    } else {
                        ']'
                        // (id%10).to_string().chars().next().unwrap()
                    }
                });
                if res.is_err() {
                    return res;
                }
            }
            let res = write!(f, "\n");
            if res.is_err() {
                return res;
            }
        }

        return Ok(());
    }
}

struct PuzzleInput {
    map: WarehouseMap,
    start_pos: Vec2<usize>, 
    instructions: Vec<RobotInstruction>
}

impl PuzzleInput {
    fn from_fstr(fstr: String) -> Self {
        let width = fstr.lines().next().unwrap().chars().count();
        let height = fstr.lines().take_while(|l| *l!="").count();

        let mut b_id: usize = 0;
        let mut box_locs: HashMap<usize, Vec2<usize>> = HashMap::new();
        let mut map: Vec<TileKind> = Vec::with_capacity(width*height);
        let mut robo_start: Option<Vec2<usize>> = None;
        for (r, l) in fstr.lines().take_while(|l| *l!="").enumerate() {
            for (c,char) in l.chars().enumerate() {
                map.push(match char {
                    '.' => EMPTY,
                    '#' => WALL,
                    'O' => {
                        box_locs.insert(b_id, Vec2 { x: c, y: r });
                        b_id += 1;
                        BOX(b_id-1)
                    }
                    '@' => {
                        robo_start = Some(Vec2 { x: r, y: c });
                        EMPTY
                    }
                    _ => panic!("Invalid character in map"),
                });
            }
        }

        // println!("{}", WarehouseMap {
        //     width,
        //     height,
        //     map: map.clone()
        // });

        if robo_start.is_none() {
            panic!("Robot start pos could not be found");
        }


        let mut instructions: Vec<RobotInstruction> = Vec::new();
        for char in fstr.lines()
            .skip_while(|l| *l!="")
            .skip(1)
            .map(|l| l.chars())
            .flatten() 
        {
            instructions.push(match char {
                '>' => RIGHT,
                '^' => UP,
                '<' => LEFT,
                'v' => DOWN,
                _ => panic!("Invalid character in instructions"),
            });
        }

        // for ins in instructions.iter() {
        //     match ins {
        //         RIGHT => print!(">"),
        //         UP => print!("^"),
        //         LEFT => print!("<"),
        //         DOWN => print!("v")
        //     }
        // }

        PuzzleInput {
            map: WarehouseMap {
                width, height, 
                map,
                box_locs,
            },
            start_pos: robo_start.unwrap(),
            instructions,
        }

    }
}

fn part1(puz_in: &PuzzleInput) {

    let mut map = puz_in.map.clone();
    let mut pos = puz_in.start_pos.clone();

    for instr in puz_in.instructions.iter() {

        let mut checking = instr.move_vec_in(pos);

        if map.get_v(checking) == EMPTY {
            pos = checking;
            continue;
        }

        while let BOX(_) = map.get_v(checking) {
            checking = instr.move_vec_in(checking);
        }

        if map.get_v(checking) == WALL {
            continue;
        }

        // We are now pushing a box
        let new_pos = instr.move_vec_in(pos);
        map.set_v(checking, BOX(0));
        map.set_v(new_pos, EMPTY);
        pos = new_pos;
    }

    // println!("{}", map);

    let mut score = 0;
    for r in 0..map.height {
        for c in 0..map.width {
            if let BOX(_) = map.get(r,c) {
                score += 100 * r + c;
            }
        }
    }

    println!("Total GPS score: {}", score);

}

fn part2(puz_in: &PuzzleInput) {
    let mut map = puz_in.map.make_wide();
    let mut pos = puz_in.start_pos.clone();
    pos.x = pos.x*2;

    // map.print_box_locs();

    for instr in puz_in.instructions.iter() {

        let checking = instr.move_vec_in(pos);

        if map.get_v(checking) == EMPTY {
            pos = checking;
            continue;
        }

        if map.get_v(checking) == WALL {
            continue;
        }

        if let WBOX(id) = map.get_v(checking) {
            if box_can_move(&map, id, instr) {
                move_box(&mut map, id, instr);
                pos = checking;
            }
        }
    }

    println!("{}", map);

    let score = map.box_locs.iter()
        .fold(0, |total, (_, v)| {
            total + (v.x + 100 * v.y)
        });
    println!("Total GPS score: {}", score);
}



fn box_can_move(map: &WarehouseMap, id: usize, instr: &RobotInstruction) -> bool {

    let mut to_check: [Vec2<usize>; 2] = [Vec2{x:0,y:0}; 2];
    let mut check_cnt: usize = 1;
    
    let b_loc = *map.box_locs.get(&id).unwrap();

    match instr {
        RIGHT => {
            to_check[0] = Vec2 {
                x: b_loc.x+2,
                y: b_loc.y
            };
        },
        LEFT => {
            to_check[0] = Vec2 {
                x: b_loc.x-1,
                y: b_loc.y
            };
        }
        UP => {
            to_check[0] = Vec2 {
                x: b_loc.x, y: b_loc.y - 1
            };
            to_check[1] = Vec2 {
                x: b_loc.x + 1, y: b_loc.y - 1
            };
            check_cnt = 2;
        },
        DOWN => {
            to_check[0] = Vec2 {
                x: b_loc.x, y: b_loc.y + 1
            };
            to_check[1] = Vec2 {
                x: b_loc.x + 1, y: b_loc.y + 1
            };
            check_cnt = 2;
        },
    }


    for i in 0..check_cnt {
        match map.get_v(to_check[i]) {
            EMPTY => continue,
            WALL => return false,
            BOX(_) => panic!("WTF did you do"),
            WBOX(n_id) => {
                if !box_can_move(map, n_id, instr) {
                    return false;
                }
            }
        }
    }

    true
}

fn move_box(map: &mut WarehouseMap, id: usize, instr: &RobotInstruction) {
    let mut to_check: [Vec2<usize>; 2] = [Vec2{x:0,y:0}; 2];
    let mut check_cnt: usize = 1;
    
    let b_loc = *map.box_locs.get(&id).unwrap();

    match instr {
        RIGHT => {
            to_check[0] = Vec2 {
                x: b_loc.x+2,
                y: b_loc.y
            };
        },
        LEFT => {
            to_check[0] = Vec2 {
                x: b_loc.x-1,
                y: b_loc.y
            };
        }
        UP => {
            to_check[0] = Vec2 {
                x: b_loc.x, y: b_loc.y - 1
            };
            to_check[1] = Vec2 {
                x: b_loc.x + 1, y: b_loc.y - 1
            };
            check_cnt = 2;
        },
        DOWN => {
            to_check[0] = Vec2 {
                x: b_loc.x, y: b_loc.y + 1
            };
            to_check[1] = Vec2 {
                x: b_loc.x + 1, y: b_loc.y + 1
            };
            check_cnt = 2;
        },
    }

    for i in 0..check_cnt {
        if let WBOX(n_id) = map.get_v(to_check[i]) {
            move_box(map, n_id, instr);
        }
    }

    let n_pos = match instr {
        RIGHT => Vec2 {
            x: b_loc.x + 1, y: b_loc.y
        },
        UP => Vec2 {
            x: b_loc.x, y: b_loc.y - 1
        },
        LEFT => Vec2 {
            x: b_loc.x - 1, y: b_loc.y
        },
        DOWN => Vec2 {
            x: b_loc.x,  y: b_loc.y + 1
        },
    };

    map.set_v(b_loc, EMPTY);
    map.set_v(b_loc + Vec2{x:1,y:0}, EMPTY);

    map.set_v(n_pos, WBOX(id));
    map.set_v(n_pos + Vec2{x:1,y:0}, WBOX(id));

    map.box_locs.insert(id, n_pos);
}

