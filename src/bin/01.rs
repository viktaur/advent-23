
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines()
            .map(|l| {
                let num_chars = l.chars().filter(|c| c.is_numeric());

                num_chars.clone().nth(0).unwrap().to_digit(10).unwrap() * 10 +
                    num_chars.clone().last().unwrap().to_digit(10).unwrap()
            }).sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input.lines()
            .map(|l| {
                let nums = parse_line(l);
                nums.get(0).unwrap() * 10 + nums.last().unwrap()
            }).sum()
    )
}

fn parse_line(line: &str) -> Vec<u32> {
    let mut digits = vec![];
    let mut s = String::new();
    for c in line.chars() {
        s.push(c);
        ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
            .map(|w| if s.contains(w) {
                digits.push(letters_to_digit(&w).unwrap());
                s.clear();
            });

        if c.is_numeric() {
            digits.push(c.to_digit(10).unwrap());
            s.clear();
        }
    }
    digits
}

fn letters_to_digit(word: &str) -> Option<u32> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
