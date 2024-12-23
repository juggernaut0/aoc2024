use aoc::parse_lines_with;
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (connections, nodes_to_connections) = parse_input(&input);
        let triples: HashSet<_> = connections
            .into_iter()
            .flat_map(|(from, to)| {
                let from_connections = &nodes_to_connections[from];
                let to_connections = &nodes_to_connections[to];
                from_connections
                    .iter()
                    .copied()
                    .filter(|c| to_connections.contains(c))
                    .map(move |c| {
                        let mut res = vec![from, to, c];
                        res.sort_unstable();
                        res
                    })
            })
            .filter(|t| t.iter().any(|&n| n.starts_with('t')))
            .collect();
        triples.len().to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let (connections, nodes_to_connections) = parse_input(&input);
        let mut cliques: HashSet<Vec<&str>> = connections
            .into_iter()
            .map(|(from, to)| {
                let mut res = vec![from, to];
                res.sort_unstable();
                res
            })
            .collect();

        while cliques.len() > 1 {
            log::info!("{} cliques left", cliques.len());
            if cliques.len() == 78 {
                for c in &cliques {
                    log::debug!("{c:?}");
                }
            }
            let mut new_cliques = HashSet::new();
            for clique in cliques {
                let mut new_clique = clique.clone();
                for &node in nodes_to_connections.keys() {
                    if !clique.contains(&node)
                        && clique
                            .iter()
                            .all(|&n| nodes_to_connections[n].contains(&node))
                    {
                        new_clique.push(node);
                        break;
                    }
                }
                if new_clique.len() == clique.len() {
                    continue;
                }
                new_clique.sort_unstable();
                log::trace!("{clique:?} -> {new_clique:?}");
                new_cliques.insert(new_clique);
            }
            cliques = new_cliques;
        }

        cliques.into_iter().next().unwrap().join(",")
    }
}

type Connections<'a> = Vec<(&'a str, &'a str)>;
type Nodes<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_input(input: &str) -> (Connections, Nodes) {
    let connections: Vec<_> =
        parse_lines_with(input, |line| line.split_once('-').unwrap()).collect();
    let mut nodes_to_connections = HashMap::new();
    for &(from, to) in &connections {
        nodes_to_connections
            .entry(from)
            .or_insert_with(Vec::new)
            .push(to);
        nodes_to_connections
            .entry(to)
            .or_insert_with(Vec::new)
            .push(from);
    }
    (connections, nodes_to_connections)
}
