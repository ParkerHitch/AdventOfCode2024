use std::usize;


fn main() {
    let fstr: String = std::fs::read_to_string("./input/D25.txt").expect("Error Reading File!");

    let info = ProblemInfo::from_fstr(&fstr);

    let good_pairs = part1(&info);
    println!("Part 1. {good_pairs} possibel pairs!");
}

struct Lock {
    heights: [u8; 5]
}


struct Key {
    heights: [u8; 5]
}

struct ProblemInfo {
    locks: Vec<Lock>,
    keys: Vec<Key>
}

impl ProblemInfo {
    fn from_fstr(fstr: &str) -> Self {
        let mut out = ProblemInfo {
            locks: Vec::new(),
            keys: Vec::new()
        };

        let mut lines = fstr.lines().peekable();

        while let Some(_) = lines.peek() {


            let top_char = lines.next().unwrap().chars().next().unwrap();
            let mut stalagtite_height: [u8; 5] = [0; 5];
            // println!("top char: {top_char}");

            for _ in 0..5 {
                for (i,c) in lines.next().unwrap().chars().enumerate() {
                    if c == top_char {
                        stalagtite_height[i] += 1;
                    }
                }
            }

            match top_char {
                '#' => {
                    println!("Lock: {:?}", stalagtite_height);
                    out.locks.push(Lock {
                        heights: stalagtite_height,
                    });
                },
                '.' => {
                    out.keys.push(Key {
                        heights: [5 - stalagtite_height[0], 5 - stalagtite_height[1], 5 - stalagtite_height[2], 5 - stalagtite_height[3],5 - stalagtite_height[4]]
                    });
                    println!("Key: {:?}", out.keys.last().unwrap().heights);
                },
                _ => panic!("Invalid char encountered!"),
            }


            // Bottom line
            lines.next();
            // blank line
            lines.next();
        }

        return out;
    }
}

fn part1(info: &ProblemInfo) -> usize {

    let mut good_pairs = 0;

    for lock in info.locks.iter() {
        for key in info.keys.iter() {

            if lock.heights.iter().zip(key.heights.iter())
                .all(|(l,k)| l+k <= 5) {
                good_pairs += 1;
            }
        }
    }

    return good_pairs;
}

