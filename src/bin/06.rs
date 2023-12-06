advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let times: Vec<u32> = input.lines().next()?.split(':').nth(1)?
        .split_whitespace().map(|t| t.parse::<u32>().unwrap()).collect();

    let distances: Vec<u32> = input.lines().nth(1)?.split(':').nth(1)?
        .split_whitespace().map(|d| d.parse::<u32>().unwrap()).collect();

    Some(
        (0..times.len()).map(|r| {
            (0..=times[r]).filter(|x| x * (times[r]-x) > distances[r]).count() as u32
        }).product()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let time: u64 = input.lines().next().unwrap().split(':').nth(1).unwrap()
        .chars().filter(|c| !c.is_whitespace()).collect::<String>().parse().unwrap();

    let distance: u64 = input.lines().nth(1).unwrap().split(':').nth(1).unwrap()
        .chars().filter(|c| !c.is_whitespace()).collect::<String>().parse().unwrap();

    Some(
        (0..=time).filter(|x| x * (time - x) > distance).count() as u32
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
