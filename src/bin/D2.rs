
fn main() {

    let fstr: String = std::fs::read_to_string("./input/D2.txt").expect("Error Reading File!");

    let mut safe_cnt: u32 = 0;

    for line in fstr.lines() {

        let reports: Vec<i32> = line.split(" ").map(|s| str::parse::<i32>(s).unwrap()).collect();

        for i in 0..reports.len() {
            let inc = reports.clone().into_iter()
                .enumerate()
                .filter(|(j,_)| *j != i)
                .map(|(_,val)| val)
                .collect::<Vec<i32>>()
                .windows(2)
                .all(|p| p[1]-p[0] >= 1 && p[1]-p[0] <= 3);
            if !inc {
                let dec = reports.clone().into_iter()
                    .enumerate()
                    .filter(|(j,_)| *j != i)
                    .map(|(_,val)| val)
                    .collect::<Vec<i32>>()
                    .windows(2)
                    .all(|p| p[1]-p[0] <= -1 && p[1]-p[0] >= -3);
                if dec {
                    safe_cnt += 1;
                    break;
                }
            } else {
                safe_cnt += 1;
                break;
            }
        }

    }

    println!("Number of Safe Reports: {}", safe_cnt);

}
