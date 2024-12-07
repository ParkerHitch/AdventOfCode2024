use std::collections::HashMap;

#[derive(Debug)]
struct PageDAG { 
    dag: Vec<Vec<bool>>,
    page_lookup: HashMap<usize, usize>,
}

impl PageDAG {

    fn new(rules: &Vec<(usize, usize)>, page_lookup: HashMap<usize, usize>) -> Self {

        let mut dag: PageDAG = PageDAG {
            dag: vec![vec![false; page_lookup.len()]; page_lookup.len()],
            page_lookup
        };
        
        for rule in rules {
            dag.dag
                [*dag.page_lookup.get(&rule.0).unwrap()]
                [*dag.page_lookup.get(&rule.1).unwrap()] = true;
        }

        dag
    }

    fn rule_exists(&self, pg_before: usize, pg_after: usize) -> bool {
        self.dag
            [*self.page_lookup.get(&pg_before).unwrap()]
            [*self.page_lookup.get(&pg_after).unwrap()]
    }

    fn pg_to_i(&self, pg: usize) -> usize {
        *self.page_lookup.get(&pg).unwrap()
    }

}

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D5.txt").expect("Error Reading File!");
    let strlines: Vec<&str> = fstr.lines().collect();
    let blank_ind: usize = strlines.iter().position(|x| *x == "" ).unwrap();

    let rules: Vec<(usize, usize)> = strlines[0..blank_ind].iter()
        .map(|s| {
            let div: usize = (*s).chars().position(|c| c=='|').unwrap();
            (s[0..div].parse().unwrap(), s[div+1..].parse().unwrap())
        })
        .collect();

    let sequences: Vec<Vec<usize>> = strlines[blank_ind+1..].iter()
        .map( |s| {
            s.split(',').map(|n| n.parse().unwrap()).collect()
        })
        .collect();

    // Maps page # to array index
    let mut page_lookup: HashMap<usize, usize> = HashMap::new(); 

    for rule in rules.iter() {
        if !page_lookup.contains_key(&rule.0) {
            page_lookup.insert(rule.0, page_lookup.len());
        }
        if !page_lookup.contains_key(&rule.1) {
            page_lookup.insert(rule.1, page_lookup.len());
        }
    }

    let page_dag = PageDAG::new(&rules, page_lookup);

    let failed_seqs = part1(&page_dag, sequences);
    part2(&page_dag, failed_seqs);
}

fn mark_dead(dead_pgs: &mut HashMap<usize, bool>, dag: &PageDAG, pg: usize) {

    if *dead_pgs.get(&pg).unwrap() {
        return;
    }

    dead_pgs.insert(pg, true);

    for i in dag.page_lookup.keys() {
        if dag.rule_exists(*i, pg) {
            // println!("Rule exists from {} to {}", i, pg);
            // mark_dead(dead_pgs, dag, *i);
            dead_pgs.insert(*i, true);
        }
    }
}

fn part1(dag: &PageDAG, seqs: Vec<Vec<usize>>) -> Vec<Vec<usize>>{

    let mut middle_sum: usize = 0;
    let mut failed_seqs = Vec::new();

    'outer: for seq in seqs {

        let mut dead_pgs: HashMap<usize, bool> = dag.page_lookup.keys().map(|k| (*k, false)).collect();

        for pg in &seq {

            if *dead_pgs.get(pg).unwrap() {
                failed_seqs.push(seq);
                continue 'outer;
            }

            mark_dead(&mut dead_pgs, &dag, *pg);

        }

        middle_sum += seq[seq.len()/2];
    }

    println!("Total of middle elems: {}", middle_sum);
    failed_seqs
}

fn part2(dag: &PageDAG, seqs: Vec<Vec<usize>>) {

    let mut middle_sum = 0;

    for seq in &seqs {

        let mut n_dag: Vec<Vec<bool>> = Vec::with_capacity(dag.dag.len());
        for x in dag.dag.iter() {
            n_dag.push(x.clone());
        }

        let mut preds: Vec<usize> = vec![0; seq.len()];

        for (i, num) in seq.iter().enumerate() {
            for pg in seq {
                if dag.rule_exists(*pg, *num) {
                    preds[i] += 1;
                }
            }
        }

        let mut paired: Vec<(&usize, &usize)> = seq.iter().zip(preds.iter()).collect();
        paired.sort_by_key(|x| x.1);

        middle_sum += paired[paired.len()/2].0;

    }

    println!("Total of middle for fixed ones: {}", middle_sum);

}
