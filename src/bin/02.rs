use std::cmp::max;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines()
            .map(|l| {
                let game_id = l.split(':').next().unwrap()
                    .split(' ').nth(1).unwrap()
                    .parse::<u32>().unwrap();

                let mut games = l.split(":").nth(1).unwrap().split(";");
                if games.all(|gs| {
                    gs.split(",").all(|g| valid_n_cubes(
                        g.trim().split(" ").nth(0).unwrap(),
                        g.trim().split(" ").nth(1).unwrap()
                    ))
                }) { game_id } else { 0 }
            })
            .sum()
    )
}

fn valid_n_cubes(n: &str, colour: &str) -> bool {
    let n: u32 = n.parse().unwrap();
    match colour {
        "red" => n <= 12,
        "green" => n <= 13,
        "blue" => n <= 14,
        _ => false
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input.lines()
            .map(|l| {
                let set = l.split(":").nth(1).unwrap().split(";");
                let (mut r, mut g, mut b) = (0, 0, 0);

                for s in set {
                    for c in s.split(",") {
                        let n: u32 = c.trim().split(" ").nth(0).unwrap().parse().unwrap();
                        let colour = c.trim().split(" ").nth(1).unwrap();
                        match colour {
                            "red" => r = max(r, n),
                            "green" => g = max(g, n),
                            "blue" => b = max(b, n),
                            _ => ()
                        }
                    }
                }

                r * g * b
            })
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
