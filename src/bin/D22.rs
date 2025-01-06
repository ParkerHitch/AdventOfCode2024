
fn main() {

    let fstr: String = std::fs::read_to_string("./input/D22.txt").expect("Error Reading File!");
    // let fstr: String = std::fs::read_to_string("./input/D22_test.txt").expect("Error Reading File!");

    let initial_secrets: Vec<u64> = fstr.lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let final_secrets = initial_secrets.iter().map(|s| {
        let mut sec = *s;
        for _ in 0..2000 {
            sec = evolve(sec);
        }
        sec
    });

    let final_sum: u64 = final_secrets.fold(0, |sum, s| {
        sum + s
    });

    println!("Sum of secrets after 2000 iterations: {}", final_sum);

    let (max_gain, seq) = part2(&initial_secrets);

    println!("Max gain of {} achived using sequence {:?}", max_gain, seq);
}

fn evolve(x: u64) -> u64 {
    let a = (x ^ (x << 6 )) % 16777216;
    let b = (a ^ (a >> 5 )) % 16777216;
    let c = (b ^ (b << 11)) % 16777216;
    c
}


// God awful implementation. Took 35 minutes to run. Should have multithreaded, but it's Rust.
// Despite it's claims, I am still very much afraid of its concurrency.
fn part2(initial_secrets: &Vec<u64>) -> (u64, (i8,i8,i8,i8)) {

    let gain_seq: Vec<Vec<(u8, (i8, i8, i8, i8))>> = 
        initial_secrets.iter().map(into_gain_sequence).collect();

    let mut max_bananas = 0;
    let mut max_seq = (0,0,0,0);

    for s1 in -9..=9 {
        for s2 in -9..=9 {
            println!("Checking: {},{},x,x", s1,s2);
            if s1 + s2 >= 10 || s1 + s2 <= -10 {
                continue;
            }
            for s3 in -9..=9 {
                if s1 + s2 + s3 >= 10 || s1 + s2 +s3 <= -10 {
                    continue;
                }
                for s4 in -9..=9 {
                    if s1 + s2 + s3 + s4 >= 10 || s1 + s2 +s3 + s4 <= -10 {
                        continue;
                    }
                    let seq = (s1,s2,s3,s4);
                    let bananas: u64 = gain_seq.iter().fold(0, |sum, gn_seq| {

                        sum + gn_seq.iter().filter(|(_, s)| *s==seq).next().unwrap_or(&(0,(0,0,0,0))).0 as u64

                    });

                    if bananas > max_bananas {
                        max_bananas = bananas;
                        max_seq = seq;
                    }

                }
            }
        }
    }




    (max_bananas, max_seq)
}

fn into_gain_sequence(seed: &u64) -> Vec<(u8, (i8, i8, i8, i8))> {

    let mut prices: Vec<u8> = Vec::with_capacity(2001);
    prices.push((*seed % 10) as u8);
    let mut s = *seed;
    for _ in 0..2000 {
        s = evolve(s);
        prices.push((s % 10) as u8);
    }

    let changes: Vec<i8> = prices.iter()
        .zip(prices.iter().skip(1))
        .map(|(a,b)| ((*b as i8-*a as i8)))
        .collect();

    let cs = changes.iter();
    let mut seq_to_gain: Vec<(u8, (i8,i8,i8,i8))> = 
        prices.iter().skip(4)
        .zip(cs.clone().zip(cs.clone().skip(1).zip(cs.clone().skip(2).zip(cs.skip(3)))))
        .map(|(p, (s1, (s2, (s3, s4))))| {
            (*p, (*s1,*s2,*s3,*s4))
        })
        .collect();

    seq_to_gain.shrink_to_fit();

    return seq_to_gain;
}
