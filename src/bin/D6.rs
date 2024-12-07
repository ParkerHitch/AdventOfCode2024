use core::panic;
use std::usize;


struct LabMap {

    width: usize,
    height: usize,

    layout: Vec<bool>,

    gaurd_start_x: usize,
    gaurd_start_y: usize,

    gaurd_start_dir: usize,
}

impl LabMap {

    fn create_from_fstr(fstr: &String) -> Self {
        let lines: Vec<&str> = fstr.lines().collect();

        let mut out = LabMap {
            width: lines[0].len(),
            height: lines.len(),
            layout: Vec::with_capacity(lines.len() * lines[0].len()),
            gaurd_start_x: 0,
            gaurd_start_y: 0,
            gaurd_start_dir: 0,
        };

        for (y,line) in lines.iter().enumerate() {
            for (x,char) in line.bytes().enumerate() {
                match char.into() {
                    '.' => out.layout.push(false),
                    '#' => out.layout.push(true),
                    _ => {
                        out.layout.push(false);
                        out.gaurd_start_x = x;
                        out.gaurd_start_y = y;
                        match char.into() {
                            '^' => out.gaurd_start_dir = 0,
                            '>' => out.gaurd_start_dir = 1,
                            'v' => out.gaurd_start_dir = 2,
                            '<' => out.gaurd_start_dir = 3,
                            _ => panic!(),
                        }
                    }
                }
            }
        }

        out

    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 &&
        (x as usize) < self.width && (y as usize) < self.height
    }

    fn is_blocked(&self, x: usize, y: usize) -> bool {
        self.layout[x + y*self.width]
    }

    fn to_ind(&self, x: usize, y: usize) -> usize {
        x + y*self.width
    }

    fn dbg_print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_blocked(x, y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

impl Clone for LabMap {
    fn clone(&self) -> Self {
        Self { 
            width: self.width, 
            height: self.height, 
            layout: self.layout.clone(), 
            gaurd_start_x: self.gaurd_start_x, 
            gaurd_start_y: self.gaurd_start_y, 
            gaurd_start_dir: self.gaurd_start_dir 
        }
    }
}

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D6.txt").expect("Error Reading File!");

    let lab_map = LabMap::create_from_fstr(&fstr);

    lab_map.dbg_print();

    part1(&lab_map);
    part2(&lab_map);
}

fn part1(lm: &LabMap) {


    let mut visited = vec![false; lm.layout.len()];

    let mut g_x: i32 = lm.gaurd_start_x as i32;
    let mut g_y: i32 = lm.gaurd_start_y as i32;
    let mut g_dir = lm.gaurd_start_dir;

    while lm.in_bounds(g_x, g_y) {
        visited[lm.to_ind(g_x as usize, g_y as usize)] = true;

        let (n_x, n_y) = match g_dir {
            0 => (g_x, g_y-1),
            1 => (g_x+1, g_y),
            2 => (g_x, g_y+1),
            3 => (g_x-1, g_y),
            _ => panic!()
        };

        if lm.in_bounds(n_x, n_y) && lm.is_blocked(n_x as usize, n_y as usize) {
            g_dir = (g_dir + 1) % 4;
        } else {
            g_x = n_x;
            g_y = n_y;
        }
    }

    let visited_cnt = visited.iter().filter(|x| **x).count();

    println!("Visited {} sqares", visited_cnt);

}

fn is_cyclic(lm: &LabMap) -> bool {

    // 2d w/ dir as first dimension
    let mut visited = vec![vec![false; lm.layout.len()]; 4];

    let mut g_x: i32 = lm.gaurd_start_x as i32;
    let mut g_y: i32 = lm.gaurd_start_y as i32;
    let mut g_dir = lm.gaurd_start_dir;

    while lm.in_bounds(g_x, g_y) {
    
        if visited[g_dir][lm.to_ind(g_x as usize, g_y as usize)] {
            return true;
        }

        visited[g_dir][lm.to_ind(g_x as usize, g_y as usize)] = true;

        let (n_x, n_y) = match g_dir {
            0 => (g_x, g_y-1),
            1 => (g_x+1, g_y),
            2 => (g_x, g_y+1),
            3 => (g_x-1, g_y),
            _ => panic!()
        };

        if lm.in_bounds(n_x, n_y) && lm.is_blocked(n_x as usize, n_y as usize) {
            g_dir = (g_dir + 1) % 4;
        } else {
            g_x = n_x;
            g_y = n_y;
        }
    }

    return false;
}

fn part2(lm: &LabMap) {

    let mut block_cnt = 0;

    for i in 0..lm.layout.len() {
        if lm.layout[i] {
            continue;
        }

        let mut lm2: LabMap = lm.clone();
        lm2.layout[i] = true;

        if is_cyclic(&lm2) {
            block_cnt += 1;
        }

    }

    println!("Number of places you can add a block: {}", block_cnt);

}
