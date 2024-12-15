use std::usize;
use std::ops::Add;

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D12.txt").expect("Error Reading File!");

    let map: Vec<Vec<char>> = fstr.lines().map(|l| l.chars().collect()).collect();

    part1(&map);
    part2(&map);
}

struct RegionInfo {
    area: usize,
    perimiter: usize,
}

impl Add for RegionInfo {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        RegionInfo {
            area: self.area + rhs.area,
            perimiter: self.perimiter + rhs.perimiter
        }
    }
}

fn move_from<T>(map: &Vec<Vec<T>>, pos: (usize, usize), dir: usize) -> Option<(usize, usize)> {

    match dir {
        0 => {
            if pos.1+1 >= map[0].len() {
                return None;
            }
            return Some((pos.0, pos.1+1));
        }
        1 => {
            if pos.0 <= 0 {
                return None;
            }
            return Some((pos.0-1, pos.1));
        }
        2 => {
            if pos.1 <= 0 {
                return None;
            }
            return Some((pos.0, pos.1-1));
        }
        3 => {
            if pos.0+1 >= map.len() {
                return None;
            }
            return Some((pos.0+1, pos.1));
        }
        _ => panic!("Bad dir")
    }

}

fn region_id(map: &Vec<Vec<char>>, region_map: &mut Vec<Vec<Option<usize>>>, r_id: usize, pos: (usize, usize)) -> RegionInfo {

    let this_char = map[pos.0][pos.1];

    region_map[pos.0][pos.1] = Some(r_id);

    let mut r_info = RegionInfo {
        area: 1,
        perimiter: 0
    };

    for dir in 0..=3 {

        let n_pos_opt = move_from(map, pos, dir);

        if n_pos_opt.is_none() {
            r_info.perimiter += 1;
            continue;
        }

        let n_pos = n_pos_opt.unwrap();

        if map[n_pos.0][n_pos.1] != this_char {
            r_info.perimiter += 1;
            continue;
        }

        if region_map[n_pos.0][n_pos.1].is_none() {
            r_info =  r_info + region_id(map, region_map, r_id, n_pos);
        }
    }

    r_info
}

fn part1(map: &Vec<Vec<char>>) {

    let mut region_map: Vec<Vec<Option<usize>>> = map.iter()
        .map(|l| 
            l.iter().map(|_| None).collect()
        ).collect();


    let mut region_infos: Vec<RegionInfo> = Vec::new();

    for (r, l) in map.iter().enumerate() {
        for (c, _) in l.iter().enumerate() {
            if region_map[r][c].is_some() {
                continue;
            }

            let r_id = region_infos.len();

            let r_info = region_id(map, &mut region_map, r_id, (r, c));

            region_infos.push(r_info);
        }
    }


    let mut total = 0;

    for info in region_infos {
        total += info.area * info.perimiter;
    }

    println!("Total price: {}", total);

}


// PART 2 !!!

fn region_id2(  map: &Vec<Vec<char>>, 
                region_map: &mut Vec<Vec<Option<usize>>>,
                region_infos: &mut Vec<RegionInfo>,
                start_pos: (usize, usize)) -> usize {

    let r_id = region_infos.len();
    region_infos.push(RegionInfo {
        area: 0,
        perimiter: 0
    });
    // We know start_pos must be the leftmost plant of the topmost row of the region,
    // otherwise it would have already been grouped into another region.
    
    fn get<T: Clone>(m: &Vec<Vec<T>>, p: (usize, usize)) -> T {
        m[p.0][p.1].clone()
    }

    let mut left_edges: Vec<(usize, usize)> = Vec::new();

    let this_char = get(map, start_pos);

    let mut pos = start_pos;
    let mut dir = 0;
    // Should be 1 so we count the left side of the starting position
    let mut sides = 1;

    while !(pos==start_pos && dir==1) { 
        // println!("({},{}), {}", pos.0, pos.1, dir);
        region_map[pos.0][pos.1] = Some(r_id);

        let l_dir = (dir+1)%4;
        let left_pos = move_from(map, pos, l_dir);

        if left_pos.is_some_and(|lp| get(map,lp)==this_char) {
            // Wall has ended. turn left.
            sides += 1;
            pos = left_pos.unwrap();
            dir = l_dir;
            continue;
        }

        if l_dir == 0 {
            left_edges.push(pos);
        }

        let n_pos = move_from(map, pos, dir);

        if n_pos.is_none_or(|np| get(map, np)!=this_char) {
            // Wall has ended. turn right, but dont move
            sides += 1;
            dir = match dir {
                0 => 3,
                _ => dir-1,
            };
            continue;
        }

        pos = n_pos.unwrap();
    }

    let mut area = 0;
    let mut enclosed: Vec<usize> = Vec::new();
    for (r, row) in map.iter().enumerate() {
        let mut c = 0;
        while c < row.len() {
            // println!("Starting scan from: {}", c);
            while c < row.len() && match get(region_map, (r,c)){
                    Some(id) => id!=r_id,
                    None => {
                        if c>0 && get(region_map, (r,c-1)).is_some_and(|id| enclosed.contains(&id)) {
                            false
                        } else {
                            true
                        }
                    },
                }{
                c += 1;
            }
            if c==row.len() {
                break;
            }
            
            let segment_start = c;

            while c<row.len() && get(map, (r,c))==this_char {
                area += 1;
                c += 1;
            }
            let segment_end = c;
            if c!=row.len() && get(region_map, (r,c)).is_none() {
                // if the previous one is not a left edge
                if left_edges.iter().filter(|p| p.0==r && p.1==(c-1)).next().is_none() {
                    // Found an enclosed region
                    // println!("Found enclosed region at: ({}, {})", r, c);
                    let new_id = region_id2(map, region_map, region_infos, (r,c));
                    enclosed.push(new_id);
                    sides += region_infos[new_id].perimiter;
                }
            }

            // println!("{} to {} are in region: {}", segment_start, segment_end, r_id);
            for c_ind in segment_start..segment_end {
                region_map[r][c_ind] = Some(r_id);
            }
        }
    }

    region_infos[r_id] = RegionInfo {
        area,
        perimiter: sides
    };
    r_id
}

fn part2(map: &Vec<Vec<char>>) {

    let mut region_map: Vec<Vec<Option<usize>>> = map.iter()
        .map(|l| 
            l.iter().map(|_| None).collect()
        ).collect();

    let mut region_infos: Vec<RegionInfo> = Vec::new();

    'outer: for (r, l) in map.iter().enumerate() {
        for (c, _) in l.iter().enumerate() {
            if region_map[r][c].is_some() {
                continue;
            }

            // println!("Starting region at: ({},{})", r,c);
            _ = region_id2(map, &mut region_map, &mut region_infos, (r, c));
            if region_infos.len() == 3 {
                break 'outer;
            }
        }
    }

    let mut total = 0;
    
    for (_,info) in region_infos.iter().enumerate() {
        // println!("Region: {} has sides: {} + area: {}", id, info.perimiter, info.area);
        total += info.area * info.perimiter;
    }

    println!("Total price: {}", total);
}
