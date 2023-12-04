advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let nums = l.split(':').nth(1).unwrap();
                let winning_nums: Vec<&str> = nums.split('|').next().unwrap().split_whitespace().collect();

                let count = nums.split('|').nth(1).unwrap().split_whitespace().filter(|n| {
                    winning_nums.contains(n)
                }).count() as u32;

                if count != 0 { 2u32.pow(count-1) } else { 0 }
            })
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut copies: Vec<u32> = vec![];

    for l in input.lines() {
        let card_n: usize = l.split(':').next().unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();
        let nums = l.split(':').nth(1).unwrap();
        let winning_nums: Vec<&str> = nums.split('|').next().unwrap().split_whitespace().collect();
        let count = nums.split('|').nth(1).unwrap().split_whitespace().filter(|n| {
            winning_nums.contains(n)
        }).count();

        match copies.get(card_n-1) {
            Some(_) => copies[card_n-1] += 1,
            None => copies.push(1)
        }

        for i in 0..count {
            match copies.get(card_n+i) {
                Some(_) => copies[card_n+i] += copies[card_n-1],
                None => copies.push(copies[card_n-1])
            }
        }
    }

    Some(copies.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
