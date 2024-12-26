use aoc_util::vec2::Vec2;
use std::{collections::{BinaryHeap, HashMap}, usize};

const WIDTH: usize = 71;
const HEIGHT: usize = 71;

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D18.txt").expect("Error Reading File!");

    let falls: Vec<Vec2<usize>> = fstr.lines()
        .map(|l| {
            let mut s = l.split(',');
            Vec2 {
                x: s.next().unwrap().parse().unwrap(),
                y: s.next().unwrap().parse().unwrap()
            }
        })
        .collect();

    let p1_map = MapState::at_t(1024, &falls);

    let p1_d = part1(&p1_map);
    println!("Dist for part 1: {}", p1_d);

    let last_block = part2(&falls);
    println!("Last block to fall: {}", last_block);
}

struct MapState {
    map: [[bool; WIDTH]; HEIGHT]
}

impl MapState {
    fn at_t(t: usize, falls: &[Vec2<usize>]) -> Self {
        let mut out = MapState {
            map: [[false; WIDTH]; HEIGHT] 
        };
        for i in 0..t {
            out.map[falls[i].y][falls[i].x] = true;
        }
        out
    }
}

struct DjikstraNode {
    dist: usize,
    loc: Vec2<usize>
}

impl PartialOrd for DjikstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DjikstraNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl Eq for DjikstraNode {}

impl PartialEq for DjikstraNode {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

fn part1(map: &MapState) -> usize {

    const FINISH: Vec2<usize> = Vec2{x: WIDTH-1, y: HEIGHT-1};

    let mut queue: BinaryHeap<DjikstraNode> = BinaryHeap::new();
    queue.push(DjikstraNode {
        dist: 0,
        loc: Vec2 {
            x:0, y: 0
        }
    });

    let mut loc_min_dists: HashMap<Vec2<usize>, usize> = HashMap::new();
    loc_min_dists.insert(Vec2{x:0,y:0}, 0);

    let mut total_dist = usize::MAX;

    'outer: while let Some(node) = queue.pop() {

        let n_dist = node.dist + 1;

        if n_dist >= total_dist {
            continue;
        }

        for d in 0..=3 {

            let next: Vec2<usize> = match d {
                0 => {
                    if node.loc.x + 1 == WIDTH { continue; }
                    Vec2{x:node.loc.x + 1, y:node.loc.y}
                },
                1 => {
                    if node.loc.y == 0 { continue; }
                    Vec2{x:node.loc.x, y:node.loc.y-1}
                },
                2 => {
                    if node.loc.x == 0 { continue; }
                    Vec2{x:node.loc.x - 1, y:node.loc.y}
                },
                3 => {
                    if node.loc.y + 1 == HEIGHT { continue; }
                    Vec2{x:node.loc.x, y:node.loc.y+1}
                },
                _ => unreachable!()
            };

            if map.map[next.y][next.x] {
                continue;
            }

            if let Some(existing_dist) = loc_min_dists.get(&next) {
                if *existing_dist <= n_dist {
                    continue;
                }
            }

            loc_min_dists.insert(next, n_dist);
            if next == FINISH {
                if n_dist < total_dist {
                    total_dist = n_dist;
                }
                continue 'outer;
            } else {
                queue.push(DjikstraNode {
                    dist: n_dist,
                    loc: next
                });
            }
        }
    }

    total_dist
}

fn is_valid(d: usize) -> bool {
    return d <= WIDTH * HEIGHT;
}

fn part2(falls: &[Vec2<usize>]) -> Vec2<usize> {

    let mut r = falls.len();
    let mut l = 0;

    let mut cutoff = 0;
    while r-l>0 {

        let m = ((r-l)/2) + l;
        // println!("l: {}, m: {}, r: {}", l,m,r);

        let m_map = MapState::at_t(m, falls);
        let d = part1(&m_map);
        if is_valid(d) {
            if r-m == 1 {
                cutoff = r;
                break;
            }
            l = m;
        } else {
            if m-l == 1 {
                cutoff = m;
                break;
            }
            r = m;
        }

    };

    falls[cutoff-1]
}
