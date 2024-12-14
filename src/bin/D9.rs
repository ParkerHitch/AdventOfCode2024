
struct Filesystem {
    fs: Vec<Option<usize>>
}

impl Filesystem {
    fn from_fstr(fstr: String) -> Self {

        let mut fid: usize = 0;
        let mut fs: Vec<Option<usize>> = Vec::new();

        for (j,byte) in fstr.chars().map_while(|c| c.to_digit(10)).enumerate() {
            // println!("{}", byte);
            if j % 2 == 0 {
                // File
                for _ in 0..byte {
                    fs.push(Some(fid));
                }
                fid += 1;
            } else {
                // Blank space
                for _ in 0..byte {
                    fs.push(None);
                }
            }
        }

        Filesystem {
            fs
        }
    }

    fn print_first(&self, n: usize) {
        for i in 0..n {
            match self.fs[i] {
                Some(id) => print!("{} ", id),
                None => print!(". "),
            }
        }
        println!("");
    }
}

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D9.txt").expect("Error Reading File!");

    let fs = Filesystem::from_fstr(fstr); 

    part1(&fs);
    part2(&fs);
}

fn part1(filesystem: &Filesystem) {

    let mut fs = filesystem.fs.clone();

    filesystem.print_first(20);

    // First blank
    let mut i: usize = 0;
    while fs[i] != None { i += 1; }
    // TODO: Edge case w/ no blanks

    let mut j: usize = fs.len()-1;
    while fs[j] == None { j -= 1; }


    while i < j { 

        fs[i] = fs[j];
        fs[j] = None;

        i += 1;
        while fs[i] != None { i += 1; }

        j -= 1;
        while fs[j] == None { j -= 1; }
    }

    let checksum: usize = fs.iter()
        .enumerate()
        .filter(|p| p.1.is_some())
        .map(|(i, fid)| i * fid.unwrap())
        .sum();

    let new_fs = Filesystem {
        fs
    };

    new_fs.print_first(20);

    println!("Checksum: {}", checksum);
}

struct Block {
    ind: usize,
    len: usize
}

impl Block {
    // Finds the block that is completely before this index i
    fn find_previous<T : std::cmp::Eq + Clone >(fs: &Vec<Option<T>>, i: usize) -> Option<Block> {
        let mut j: i32 = i as i32 - 1;

        if j < 0 {
            return None;
        }

        let expect_eql = fs[j as usize].clone();

        while j >= 0 && fs[j as usize] == expect_eql {
            j -= 1;
        }

        Some( Block {
            ind: (j + 1) as usize,
            len: i-((j + 1) as usize),
        })
    }

    // Finds the block that starts at index i
    fn find_next<T : std::cmp::Eq + Clone >(fs: &Vec<Option<T>>, i: usize) -> Option<Block> {

        if i >= fs.len() {
            return None;
        }

        let expect_eql = fs[i].clone();
        let mut j = i;

        while j < fs.len() && fs[j] == expect_eql {
            j += 1;
        }

        Some( Block {
            ind: i,
            len: j-i
        })

    }
}

fn part2(filesystem: &Filesystem) {

    let mut fs = filesystem.fs.clone();

    filesystem.print_first(20);

    let mut to_move: Option<Block> = Block::find_previous(&fs, fs.len());

    'outer: while let Some(ref block_from) = to_move {

        if fs[block_from.ind].is_none() {
            to_move = Block::find_previous(&fs, block_from.ind);
            continue;
        }

        // println!("Attempting to move: {}, {}", block_from.ind, block_from.len);

        // Try to find first open block
        let mut move_to: Option<Block> = Block::find_next(&fs, 0);
        while let Some(ref block_to) = move_to {

            if block_to.ind > block_from.ind {
                break;
            }

            if  block_to.len < block_from.len ||
                fs[block_to.ind].is_some() {
                move_to = Block::find_next(&fs, block_to.ind + block_to.len);
                continue;
            }

            // Found valid move spot!
            
            // println!("Moving {} bytes from {} to {} (available: {})", block_from.len, block_from.ind, block_to.ind, block_to.len);
            
            for i in 0..block_from.len {
                fs[block_to.ind + i] = fs[block_from.ind + i];
                fs[block_from.ind + i] = None;
            }

            continue 'outer;
        }

        // Failed move
        to_move = Block::find_previous(&fs, block_from.ind);
        

    }

    let checksum: usize = fs.iter()
        .enumerate()
        .filter(|p| p.1.is_some())
        .map(|(i, fid)| i * fid.unwrap())
        .sum();

    let new_fs = Filesystem {
        fs
    };

    new_fs.print_first(20);

    println!("Checksum: {}", checksum);
}
