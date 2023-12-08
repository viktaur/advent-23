use std::collections::HashMap;
use prse::parse;
use num::integer::lcm;
advent_of_code::solution!(8);

enum Instruction {
    Left,
    Right,
}
#[derive(Eq, PartialEq, Hash, Clone)]
struct Node(String);
type Wrapper = (Vec<Instruction>, HashMap<Node, (Node, Node)>, Node, Vec<Node>);


pub fn part_one(input: &str) -> Option<u32> {
    let (ins_vec, hash_map, start, _) = parse(input).unwrap();
    let mut current_node: Node = start;
    let mut steps = 0;

    loop {
        for i in ins_vec.iter() {
            steps += 1;

            match i {
                Instruction::Left => { current_node = hash_map.get(&current_node).unwrap().clone().0 },
                Instruction::Right => { current_node = hash_map.get(&current_node).unwrap().clone().1 },
            }

            if current_node.0.eq("ZZZ") {
                return Some(steps);
            }
        }
    }
}


pub fn part_two(input: &str) -> Option<u64> {
    let (ins_vec, hash_map, _, nodes_a) = parse(input).unwrap();
    let start_nodes: Vec<Node> = nodes_a;

    Some(
        start_nodes.iter().map(|n| {

            let mut current_node = n.clone();
            let mut steps = 0;

            loop {
                for i in ins_vec.iter() {
                    steps += 1;

                    match i {
                        Instruction::Left => { current_node = hash_map.get(&current_node).unwrap().clone().0 },
                        Instruction::Right => { current_node = hash_map.get(&current_node).unwrap().clone().1 }
                    }


                    if current_node.0.chars().nth(2).eq(&Some('Z')) {
                        return steps;
                    }
                }
            }
        }).fold(1, lcm)
    )
}

fn parse(input: &str) -> Option<Wrapper> {
    let (ins, maps) = input.split_once("\n\n").unwrap();
    let ins_vec: Vec<Instruction> = ins.chars().map(|c| {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!()
        }
    }).collect();

    let hash_map: HashMap<Node, (Node, Node)> = maps.lines().map(|l| {
        let (origin, l, r): (String, String, String) = parse!(l, "{} = ({}, {})");
        (Node(origin), (Node(l), Node(r)))
    }).collect();

    let nodes_a: Vec<Node> = maps.lines().map(|l| {
        let (origin, _, _): (String, String, String) = parse!(l, "{} = ({}, {})");
        Node(origin)
    }).filter(|o| o.0.chars().nth(2).eq(&Some('A'))).collect();

    let start = Node(String::from("AAA"));

    Some((ins_vec, hash_map, start, nodes_a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
