use std::usize;

use aoc_util::vec2::Vec2;

const ROOM_WIDTH:  usize = 101;
const ROOM_HEIGHT: usize = 103;
const DIMENSIONS: Vec2<i64> = Vec2 {
    x: ROOM_WIDTH as i64,
    y: ROOM_HEIGHT as i64
};

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D14.txt").expect("Error Reading File!");

    let robots: Vec<Robot> = fstr.lines().map(Robot::from_line).collect();

    part1(&robots);
    part2(&robots);
}

struct Robot {
    pos: Vec2<i64>,
    vel: Vec2<i64>
}

impl Robot {

    fn from_line(l: &str) -> Self {
        let comma_1 = l.chars().position(|c| c==',').unwrap();
        let space   = l.chars().position(|c| c==' ').unwrap();
        let pos: Vec2<i64> = Vec2 {
            x: l[2..comma_1].parse().unwrap(),
            y: l[comma_1+1..space].parse().unwrap(),
        };
        let comma_2 = l[space..].chars().position(|c| c==',').unwrap();
        let vel: Vec2<i64> = Vec2 {
            x: l[space+3..space+comma_2].parse().unwrap(),
            y: l[space+comma_2+1..].parse().unwrap(),
        };
        Robot { pos, vel }
    }
}

fn pos_mod(a: i64, b: i64) -> usize {
    ((a % b + b) % b) as usize
}

fn vec_pos_mod(a: Vec2<i64>, b: Vec2<i64>) -> Vec2<usize> {
    Vec2 {
        x: pos_mod(a.x, b.x),
        y: pos_mod(a.y, b.y),
    }
}

fn part1(robots: &[Robot]) {

    let duration = 100;
    let final_positions: Vec<Vec2<usize>> = robots.iter().map(|r| {
        vec_pos_mod(r.pos + r.vel.s_mul(duration), DIMENSIONS)
    }).collect();

    let mut counts: [[usize; ROOM_WIDTH]; ROOM_HEIGHT] = [[0; ROOM_WIDTH]; ROOM_HEIGHT];

    for pos in final_positions.iter() {
        counts[pos.y][pos.x] += 1;
    }

    let c_ul = final_positions.iter()
        .filter(|p| p.x < ROOM_WIDTH/2 && p.y < ROOM_HEIGHT/2)
        .count();
    let c_ur = final_positions.iter()
        .filter(|p| p.x > ROOM_WIDTH/2 && p.y < ROOM_HEIGHT/2)
        .count();
    let c_ll = final_positions.iter()
        .filter(|p| p.x < ROOM_WIDTH/2 && p.y > ROOM_HEIGHT/2)
        .count();
    let c_lr = final_positions.iter()
        .filter(|p| p.x > ROOM_WIDTH/2 && p.y > ROOM_HEIGHT/2)
        .count();

    let total_score = c_ul * c_ur * c_ll * c_lr;

    println!("Total score: {}", total_score);
}

fn part2(robots: &[Robot]) {

    let mut current_duration = 1;
    let mut current_positions: Vec<Vec2<usize>> = robots.iter().map(|r| {
        vec_pos_mod(r.pos + r.vel.s_mul(current_duration), DIMENSIONS)
    }).collect();

    loop {
        let mut counts: [[usize; ROOM_WIDTH]; ROOM_HEIGHT] = [[0; ROOM_WIDTH]; ROOM_HEIGHT];

        for pos in current_positions.iter() {
            counts[pos.y][pos.x] += 1;
        }

        println!("Display for second: {}", current_duration);
        for row in counts {
            for col in row {
                if col > 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }

        current_duration += 1;
        current_positions = robots.iter().map(|r| {
            vec_pos_mod(r.pos + r.vel.s_mul(current_duration), DIMENSIONS)
        }).collect();
    }

    // Soloution is to then grep for "##################"
    // and then hit ^C when it shows up.

}
