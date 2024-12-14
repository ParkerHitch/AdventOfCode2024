use std::{collections::{HashMap, VecDeque}, usize};


fn main() {

    let fstr: String = std::fs::read_to_string("./input/D11.txt").expect("Error Reading File!");

    let stones: Vec<usize> = fstr.split(" ").map(|s| {
        s.trim().parse().unwrap()
    }).collect();

    part1(&stones, 25);
    part2(&stones, 25);
    part2(&stones, 75);
}

fn part1(s_in: &Vec<usize>, n_blinks: usize) {
    use std::time::Instant;

    let mut stones = s_in.clone();

    for b in 0..n_blinks {

        let start = Instant::now();

        let mut i = 0;

        while i < stones.len() {

            let s = stones[i];

            if s == 0 {
                stones[i] = 1;
                i += 1;
            } else if (s.ilog10()+1)%2 == 0 {

                let n_dig = s.ilog10()+1;

                stones[i] /= 10_usize.pow(n_dig/2);
                stones.insert(i+1, s % 10_usize.pow(n_dig/2));

                i += 2;
            } else {
                stones[i] *= 2024;
                i += 1;
            }

        }

        let elapsed = start.elapsed();
        println!("Finished blink {} in {:2?}", b, elapsed);
    }

    println!("Final stone cnt after {} blinks: {}", n_blinks, stones.len());

}

enum NStoneKind {
    TODO,
    Single(usize),
    Double((usize, usize)),
}

fn next_val(s: usize) -> NStoneKind {
    use NStoneKind::*;
    if s == 0 {
        Single(1)
    } else if (s.ilog10()+1)%2 == 0 {

        let n_dig = s.ilog10()+1;

        Double((s / 10_usize.pow(n_dig/2),
                s % 10_usize.pow(n_dig/2))
        )

    } else {
        Single(s * 2024)
    }
}

struct Node {
    stone_val: usize,
    next_ind: NStoneKind
}

fn add_stone (seen_stones: &mut HashMap<usize, usize>, stone_tree: &mut Vec<Node>, node: Node) -> usize {
    let i = stone_tree.len();
    stone_tree.push(node);
    seen_stones.insert(stone_tree[i].stone_val, i);
    i
}

fn link_or_add(worklist: &mut VecDeque<usize>, seen_stones: &mut HashMap<usize, usize>, stone_tree: &mut Vec<Node>, val: usize) -> usize {
    use NStoneKind::TODO;
    let n_ind_op = seen_stones.get(&val);

    if n_ind_op.is_some() {
        *n_ind_op.unwrap()
    } else {
        let i = add_stone(seen_stones, stone_tree, Node{
            stone_val: val,
            next_ind: TODO
        });
        worklist.push_back(i);
        i
    }
}

fn count_stones_recursive(mem_tbl: &mut HashMap<(usize, usize), usize>, stone_tree: &Vec<Node>, s_ind: usize, depth: usize) -> usize {
    let n: &Node = &stone_tree[s_ind];
    if depth == 0 {
        // print!("{} ", n.stone_val);
        return 1;
    }

    if depth > 30 {

        let opt = mem_tbl.get(&(s_ind, depth));
        if opt.is_some() {
            return *opt.unwrap();
        }

        let ret = match n.next_ind {
            NStoneKind::TODO => panic!(),
            NStoneKind::Single(n) => count_stones_recursive(mem_tbl, stone_tree, n, depth-1),
            NStoneKind::Double((n1,n2)) => count_stones_recursive(mem_tbl, stone_tree, n1, depth-1)
                                         + count_stones_recursive(mem_tbl, stone_tree, n2, depth-1),
        };
        mem_tbl.insert((s_ind, depth), ret);

        return ret;

    } else {


        let ret = match n.next_ind {
            NStoneKind::TODO => panic!(),
            NStoneKind::Single(n) => count_stones_recursive(mem_tbl, stone_tree, n, depth-1),
            NStoneKind::Double((n1,n2)) => count_stones_recursive(mem_tbl, stone_tree, n1, depth-1)
                                         + count_stones_recursive(mem_tbl, stone_tree, n2, depth-1),
        };
        return ret;
    }


}

fn part2(s_in: &Vec<usize>, n_blinks: usize) {
    use std::time::Instant;
    use NStoneKind::*;

    // Maps node val to ind
    let mut seen_stones: HashMap<usize, usize> = HashMap::new();
    let mut stone_tree: Vec<Node> = Vec::new();

    let mut worklist: VecDeque<usize> = VecDeque::new();
    let mut n_worklist: VecDeque<usize>;

    for s in s_in {
        let i = add_stone(&mut seen_stones, &mut stone_tree, Node {
            stone_val: *s,
            next_ind: TODO,
        });
        worklist.push_back(i);
    }

    for b in 0..n_blinks {
        n_worklist = VecDeque::with_capacity(worklist.capacity());

        let start = Instant::now();
        println!("Starting blink {} at {:?}", b, start);
        // println!("Worklist: ");

        for node in worklist {
            // println!("  {}, {}", node, stone_tree[node].stone_val);
            let next_val = next_val(stone_tree[node].stone_val);
            match next_val {
                TODO => panic!(),
                Single(n_val) => {
                    stone_tree[node].next_ind = Single(
                        link_or_add(&mut n_worklist, &mut seen_stones, &mut stone_tree, n_val)
                    );
                },
                Double((n_val1, n_val2)) => {
                    stone_tree[node].next_ind = Double((
                        link_or_add(&mut n_worklist, &mut seen_stones, &mut stone_tree, n_val1),
                        link_or_add(&mut n_worklist, &mut seen_stones, &mut stone_tree, n_val2)
                    ));
                },
            }
        }

        println!("Finished blink {} in {:2?}", b, start.elapsed());

        worklist = n_worklist;
    }

    let mut count = 0;
    let mut memoized_table: HashMap<(usize, usize), usize> = HashMap::new();
    for s in s_in.iter().map(|s_val| seen_stones.get(s_val).unwrap()) {
        count += count_stones_recursive(&mut memoized_table, &stone_tree, *s, n_blinks);
    }

    println!("Total stones: {}", count)
}
