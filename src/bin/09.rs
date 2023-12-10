advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input.lines().map(|l| {
            find_history_value(
            l.split_whitespace().map(|n| { n.parse::<i32>().unwrap() }).collect()
            )
        }).sum()
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input.lines().map(|l| {
            find_history_value_2(
                l.split_whitespace().map(|n| { n.parse::<i32>().unwrap() }).collect()
            )
        }).sum()
    )}

fn diff_row(row: &Vec<i32>) -> Vec<i32> {
    let mut new_row: Vec<i32> = vec![];

    for i in 0..row.len()-1 {
        new_row.push(row[i+1].saturating_sub(row[i]));
    }

    new_row
}

fn find_history_value(history: Vec<i32>) -> i32 {
    let new_history = diff_row(&history);

    history.last().unwrap() + {
        match new_history.last() {
            Some(0) => 0,
            Some(_) => find_history_value(new_history),
            None => panic!()
        }
    }
}

fn find_history_value_2(history: Vec<i32>) -> i32 {
    let new_history = diff_row(&history);

    match new_history.last() {
        Some(0) => *history.first().unwrap(),
        Some(_) => history.first().unwrap() - find_history_value_2(new_history),
        None => panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
