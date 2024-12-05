
fn main() {

    let fstr: String = std::fs::read_to_string("./input/D1.txt").expect("Error Reading File!");

    let (mut l1, mut l2) : (Vec<u32>, Vec<u32>) = fstr.lines().map(|line| {
        let elems: Vec<u32> = line.split("   ").map(|n| str::parse::<u32>(n).unwrap()).collect();
        (elems[0], elems[1])
    }).unzip();

    l1.sort();
    l2.sort();

    let pairs = l1.clone().into_iter().zip(l2.clone().into_iter());

    let total: u32 = pairs.map(|pair| {
        if pair.0 > pair.1 {
            pair.0 - pair.1
        } else {
            pair.1 - pair.0
        }
    }).sum();

    println!("Total diff: {}", total);

    let mut i2: usize = 0;
    let mut sim_score: u32 = 0;
    for num in l1 {
        while l2[i2] <= num {
            if l2[i2] == num {
                sim_score += num;
            }
            i2 += 1;
        }
    }

    println!("Similarity Score: {}", sim_score);

}
