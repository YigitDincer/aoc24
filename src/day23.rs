use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
struct Connections(HashMap<String, Vec<String>>);

fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let first_number = parts.next().unwrap();
            let second_number = parts.next().unwrap();
            (first_number, second_number)
        })
        .collect()
}

fn get_all_connections(single_connections: &Vec<(&str, &str)>) -> Connections {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    for (key, value) in single_connections {
        connections
            .entry(key.to_string())
            .and_modify(|vec| vec.push(value.to_string()))
            .or_insert_with(|| vec![value.to_string()]);
    }

    for (value, key) in single_connections {
        connections
            .entry(key.to_string())
            .and_modify(|vec| vec.push(value.to_string()))
            .or_insert_with(|| vec![value.to_string()]);
    }

    return Connections(connections);
}

fn get_all_trios_with_t(connections: &Connections) -> usize {
    get_all_trios(connections)
        .iter()
        .filter(|&trio| trio.iter().any(|pc| pc.starts_with('t')))
        .count()
}

fn get_all_trios(connections: &Connections) -> HashSet<Vec<String>> {
    let Connections(connections) = connections;
    let mut all_trios = HashSet::new();

    for (source, targets) in connections {
        let combs: Vec<_> = targets.iter().combinations(2).collect();
        for comb in combs {
            if connections.get(comb[0]).unwrap().contains(comb[1]) {
                let mut trio: Vec<String> =
                    vec![source.clone(), comb[0].clone(), comb[1].clone()].to_vec();
                trio.sort();
                all_trios.insert(trio);
            }
        }
    }

    all_trios
}

fn get_party_of_size_n(connections: &Connections, size: usize) -> String {
    let Connections(connections) = connections;

    for (source, targets) in connections {
        let combination_to_check = targets.iter().combinations(size - 1);

        for pcs_in_combination in combination_to_check {
            let mut flag = true;
            for &pc in &pcs_in_combination {
                let mut all_but_pc = pcs_in_combination.clone();
                all_but_pc.retain(|&a| a != pc);

                for other_pc in all_but_pc {
                    if !connections.get(pc).unwrap().contains(other_pc) {
                        flag = false;
                    }
                    if !flag {
                        break;
                    }
                }
            }
            if flag {
                let mut pcs_in_combination_dup = pcs_in_combination.clone();
                pcs_in_combination_dup.push(source);
                pcs_in_combination_dup.sort();
                return pcs_in_combination_dup.into_iter().join(",");
            }
        }
    }

    return "".to_string();
}

fn get_biggest_lan_party(connections: &Connections) -> String {
    let mut comb = 3;
    let mut largest_party = String::new();
    loop {
        let last_party = get_party_of_size_n(connections, comb);
        if last_party.is_empty() {
            break;
        }
        comb += 1;
        largest_party = last_party.clone();
    }

    return largest_party.to_string();
}

pub fn solve(input: &str) {
    let single_connections = parse(input);
    let connections = get_all_connections(&single_connections);

    println!("{}", get_all_trios_with_t(&connections));
    println!("{}", get_biggest_lan_party(&connections));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    const SHORT_EXAMPLE: &str = "ka-co
ta-co
de-co
ta-ka
de-ta
ka-de";

    #[test]
    fn test_trios() {
        let single_connections = parse(EXAMPLE);
        let connections = get_all_connections(&single_connections);
        assert_eq!(7, get_all_trios_with_t(&connections));
    }

    #[test]
    fn test_biggest_lan_party_large_input() {
        let single_connections = parse(EXAMPLE);
        let connections = get_all_connections(&single_connections);
        let conn = get_biggest_lan_party(&connections);
        assert_eq!(conn, "co,de,ka,ta");
    }

    #[test]
    fn test_biggest_lan_party() {
        let single_connections = parse(SHORT_EXAMPLE);
        let connections = get_all_connections(&single_connections);
        let conn = get_biggest_lan_party(&connections);
        assert_eq!(conn, "co,de,ka,ta");
    }
}
