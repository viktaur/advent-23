use std::cmp::{max, min};
advent_of_code::solution!(3);

pub enum Char {
    Num(char),
    Dot,
    Symbol
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut scheme: Vec<Vec<Char>> = vec![];
    let mut part: Vec<u32> = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut v = vec![];
        for (j, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => v.push(Char::Num(c)),
                '.' => v.push(Char::Dot),
                _ => v.push(Char::Symbol)
            }
        }
        scheme.push(v);
    }

    let (max_i, max_j) = (scheme.len() - 1, scheme.get(0).unwrap().len() - 1);
    let (mut current, mut part_num) = (String::new(), false);

    for (i, row) in scheme.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c {
                Char::Num(n) => {
                    current.push(*n);

                    if !part_num {
                        part_num = explore_adj(i, j, max_i, max_j, &scheme);
                    }
                },
                _ => { add_part_num(&mut current, &mut part, &mut part_num); }
            }
        }
        add_part_num(&mut current, &mut part, &mut part_num);
    }

    Some(part.into_iter().sum())
}

fn add_part_num(current: &mut String, part: &mut Vec<u32>, part_num: &mut bool) {
    match current.parse() {
        Ok(n) => {
            if *part_num {
                part.push(n)
            }

            // reset variables
            current.clear();
            *part_num = false;
        },
        _ => ()
    }
}

/// Returns true if adjacent symbol found
fn explore_adj(
    i: usize, j: usize, max_i: usize, max_j: usize, scheme: &Vec<Vec<Char>>
) -> bool {
    let s_i = if i == 0 { 0 } else { i-1 };
    let s_j = if j == 0 { 0 } else { j-1 };

    let e_i = min(max_i, i+1);
    let e_j = min(max_j, j+1);

    for x in s_i..=e_i {
        for y in s_j..=e_j {
            if (x, y) == (i, j) {
                continue;
            }

            let adj = scheme.get(x).unwrap().get(y).unwrap();
            match adj {
                Char::Symbol => { return true; },
                _ => ()
            }
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
