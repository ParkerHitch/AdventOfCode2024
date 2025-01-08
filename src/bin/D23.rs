use std::{thread::current, usize};

const NUM_CPUS: usize = 26 * 26;

fn main() {
    let fstr: String = std::fs::read_to_string("./input/D23.txt").expect("Error Reading File!");

    let connections = ComputerConnections::from_fstr(&fstr);

    let three_with_t = part1(&connections);
    println!("There are {} groups of 3 with a computer whose name starts with t!", three_with_t);

    let password = part2(&connections);
    println!("The password is: {}", password);
}

struct ComputerConnections {
    graph: [[bool; NUM_CPUS]; NUM_CPUS],
}

impl ComputerConnections {

    fn name_to_index(name: &str) -> usize {
        let mut bytes = name.bytes();
        return (26 * (bytes.next().unwrap() - ('a' as u8)) as usize +
            (bytes.next().unwrap() - ('a' as u8)) as usize).into();
    }

    fn index_to_name(index: usize) -> String {
        let mut str: Vec<u8> = Vec::new();
        str.push((index/26) as u8 + ('a' as u8));
        str.push((index%26) as u8 + ('a' as u8));
        String::from_utf8(str).unwrap()
    }

    fn from_fstr(fstr: &str) -> Self {

        let mut out = ComputerConnections {
            graph: [[false; NUM_CPUS]; NUM_CPUS]
        };

        for line in fstr.lines() {
            // println!("{line}");
            let mut ids = line.split("-").map(Self::name_to_index);
            let a = ids.next().unwrap();
            let b = ids.next().unwrap();
            out.graph[a][b] = true;
            out.graph[b][a] = true;
        }
        out
    }

    fn connections_from(&self, index: usize) -> impl Iterator<Item = usize> + use<'_> {

        return 
            self.graph[index].iter()
            .enumerate()
            .filter(|(_, exists)| **exists)
            .map(|(dest, _)| dest);

    }

    fn connection_exists(&self, a: usize, b: usize) -> bool {
        return self.graph[a][b];
    }

}

fn part1(connections: &ComputerConnections) -> usize {

    let mut count = 0;

    for base_id in 0..NUM_CPUS {
        count += connections.connections_from(base_id)
            // Iter over all possible second connections in group of three
            .filter(|second| *second > base_id)
            // Iter over all possible third connections
            .flat_map(|second| 
                connections.connections_from(second).filter(move |third| *third > second)
                    // And make a trio
                    .map(move |third| [base_id, second, third])
            )
            // But only count the ones that loop back to the start
            .filter(|trio| connections.connection_exists(trio[0], trio[2]))
            // And whose name contains a "t"
            .filter(|trio| {
                for t in trio {
                    if ComputerConnections::index_to_name(*t).starts_with("t") {
                        return true;
                    }
                }
                return false;
            })
            .count();
    }

    return count;
}

// A little slow but it gets the job done
fn dfs_find_largest(connections: &ComputerConnections, existing_group: Vec<usize>, current: usize) -> Vec<usize> {

    let to_try = connections.connections_from(current)
        .filter(|next| *next > current)
        .filter(|next| existing_group.iter().all(|existing| connections.connection_exists(*existing, *next)));

    let mut best = existing_group.clone();

    for next in to_try {
        let mut candidate_group = existing_group.clone();
        candidate_group.push(next);
        let candidate = dfs_find_largest(connections, candidate_group, next);
        if candidate.len() > best.len() {
            best = candidate;
        }
    }

    return best;
}

fn part2(connections: &ComputerConnections) -> String {

    let mut party: Vec<usize> = Vec::new();


    for base_id in 0..NUM_CPUS {
        let party_cand = dfs_find_largest(connections, vec![base_id], base_id);
        if party_cand.len() > party.len() {
            party = party_cand;
        }
    }

    let names: Vec<String> = party.iter().map(|index: &usize| ComputerConnections::index_to_name(*index)).collect();
    return names.join(",");
}
