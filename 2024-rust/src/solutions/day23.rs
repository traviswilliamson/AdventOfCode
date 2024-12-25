use std::{collections::{HashMap, HashSet}, error::Error};
use itertools::Itertools;
use rayon::prelude::*;

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let adjacency_list = _input.split_terminator("\n").fold(HashMap::new(), |mut map, l| {
        let mut halves = l.split("-");
        let n1 = halves.next().unwrap();
        let n2 = halves.next().unwrap();
        let n1_entry = map.entry(n1).or_insert(HashSet::new());
        n1_entry.insert(n2);
        let n2_entry = map.entry(n2).or_insert(HashSet::new());
        n2_entry.insert(n1);
        map
    });
    // println!("{:?}", adjacency_list.len());
    Ok(adjacency_list.par_iter().map(|(el1, adjacencies)| {
        adjacencies.iter().tuple_combinations::<(_, _)>().filter(|(el2, el3)| {
            (*el2 > el1 && *el3 > el1) && (el1.starts_with('t') || el2.starts_with('t') || el3.starts_with('t')) && adjacency_list.get(*el2).unwrap().contains(*el3)
        }).count()
    }).sum::<usize>().to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let adjacency_list = _input.split_terminator("\n").fold(HashMap::new(), |mut map, l| {
        let mut halves = l.split("-");
        let n1 = halves.next().unwrap();
        let n2 = halves.next().unwrap();
        let n1_entry = map.entry(n1).or_insert(HashSet::new());
        n1_entry.insert(n2);
        let n2_entry = map.entry(n2).or_insert(HashSet::new());
        n2_entry.insert(n1);
        map
    });

    // Yes, yes, it's actually one more than that. Hush.
    let max_subgraph_size = adjacency_list.par_iter().map(|(_, adj)| adj.len()).max().unwrap();
    let mut max_subgraph = Vec::new();
    'outer: for subgraph_size in (0..(max_subgraph_size + 1)).rev() {
        for (node, adjacencies) in adjacency_list.iter() {
            for combos in adjacencies.iter().combinations(subgraph_size) {
                if combos.iter().all(|combo_node| {
                    adjacency_list.get(*combo_node).unwrap().intersection(&HashSet::from_iter(combos.iter().filter(|n| ***n != **combo_node).map(|n| **n))).count() == combos.len() - 1
                }) {
                    max_subgraph = Vec::from(combos);
                    max_subgraph.push(node);
                    break 'outer;
                }
            }
        }
    }
    max_subgraph.sort();

    Ok(max_subgraph.into_iter().join(","))
}
