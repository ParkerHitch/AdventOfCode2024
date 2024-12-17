use aoc_util::vec2::Vec2;
use crate::TileKind::*;
use std::{collections::{BinaryHeap, HashMap, HashSet, VecDeque}, usize};

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D16.txt").expect("Error Reading File!");

    let maze = MazeMap::from_fstr(fstr);

    both_parts(&maze);
}

#[derive(Clone)]
struct MazeMap {
    width: usize,
    height: usize,
    map: Vec<TileKind>,
    start: Vec2<usize>,
    end: Vec2<usize>
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum TileKind {
    EMPTY,
    WALL,
}

impl MazeMap {
    fn get(&self, r: usize, c: usize) -> TileKind {
        self.map[r*self.width + c]
    }
    fn get_v(&self, v: Vec2<usize>) -> TileKind {
        self.get(v.y, v.x)
    }
    fn set_v(&mut self, v: Vec2<usize>, t: TileKind) {
        self.map[v.y*self.width + v.x] = t;
    }

    fn from_fstr(fstr: String) -> Self {
        let width = fstr.lines().next().unwrap().chars().count();
        let height = fstr.lines().count();

        let mut map: Vec<TileKind> = Vec::with_capacity(width*height);
        let mut start: Vec2<usize> = Vec2::zero();
        let mut end: Vec2<usize> = Vec2::zero();

        for (r, l) in fstr.lines().enumerate() {
            for (c,char) in l.chars().enumerate() {
                map.push(match char {
                    '#' => WALL,
                    '.' => EMPTY,
                    'E' => {
                        end = Vec2{x: c, y: r};
                        EMPTY
                    },
                    'S' => {
                        start = Vec2{x: c, y: r};
                        EMPTY
                    },
                    _ => panic!("Invalid character in map"),
                });
            }
        }

        MazeMap {
            width,
            height,
            map,
            start,
            end,
        }
    }
}

#[derive(PartialEq, Eq)]
struct QueueNode {
    cost: usize,
    pos: Vec2<usize>,
    dir: usize
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn move_in_dir(v: Vec2<usize>, d: usize) -> Vec2<usize> {
    match d {
        0 => Vec2 { x: v.x+1, y:v.y},
        1 => Vec2 { x: v.x, y:v.y-1},
        2 => Vec2 { x: v.x-1, y:v.y},
        3 => Vec2 { x: v.x, y:v.y+1},
        _ => panic!("Bad dir"),
    }
}

struct PoseInfo {
    cost: usize,
    previous: Vec<(usize, Vec2<usize>)>,
}

fn both_parts(maze: &MazeMap) {

    let mut queue: BinaryHeap<QueueNode> = BinaryHeap::new();
    let mut pose_info: HashMap<(usize, Vec2<usize>), PoseInfo> = HashMap::new();
    let mut cheapest_cost: usize = std::usize::MAX;
    let mut cheapest_dirs: Vec<usize> = Vec::new();

    queue.push(QueueNode{
        cost: 0,
        pos: maze.start,
        dir: 0
    });
    pose_info.insert((0, maze.start), PoseInfo {
        cost: 0,
        previous: Vec::new()
    });


    while queue.len() > 0 {
        let checking = queue.pop().unwrap();
        let pos = checking.pos;
        let dir = checking.dir;

        // println!("Now checking {}, cost {}", checking.pos, checking.cost);
        if checking.cost >= cheapest_cost {
            continue;
        }

        for d in 0..=3 {
            let n_pos = move_in_dir(pos, d);
            if maze.get_v(n_pos) == WALL {
                continue;
            }
            let n_cost = if d==dir { checking.cost+1 } else { checking.cost + 1001 };

            // No point in continuing if we've already found a cheaper way to the end
            if n_cost > cheapest_cost {
                continue;
            }

            // If we have already been to the place we might be going
            if let Some(info) = pose_info.get_mut(&(d, n_pos)) {
                if info.cost < n_cost {
                    // if we've already found a cheaper way there, skip
                    continue;
                } else if info.cost == n_cost {
                    if !info.previous.contains(&(dir, pos)) {
                        info.previous.push((dir, pos));
                    }
                    continue;
                } else {
                    // We have truly found a shorter path to info's position
                    // It will get updated when we queue
                }
            }

            pose_info.insert((d, n_pos), PoseInfo {
                    cost: n_cost,
                    previous: vec![(dir, pos)],
            });
            if n_pos != maze.end {
                queue.push(QueueNode { 
                    cost: n_cost, 
                    pos: n_pos, 
                    dir: d 
                });
            } else {
                if n_cost < cheapest_cost {
                    cheapest_cost = n_cost;
                    cheapest_dirs = vec![dir];
                } else if n_cost == cheapest_cost {
                    if !cheapest_dirs.contains(&dir) {
                        cheapest_dirs.push(dir);
                    }
                }
            }
        }
    }

    let mut in_a_shortest_path: HashSet<Vec2<usize>>  = HashSet::new();
    let mut to_check: VecDeque<(usize,Vec2<usize>)> = VecDeque::new();

    for d in cheapest_dirs {
        to_check.push_back((d, maze.end));
    }
    in_a_shortest_path.insert(maze.end);

    while let Some(checking) = to_check.pop_front() {
        let prev = pose_info.get(&checking).unwrap();
        for p in prev.previous.iter() {
            in_a_shortest_path.insert(p.1);
            to_check.push_back(*p);
        }
    } 

    println!("Cheapest cost: {}", cheapest_cost);
    println!("Total in shortest path: {}", in_a_shortest_path.len());
}
