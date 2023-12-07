advent_of_code::solution!(5);

#[derive(Clone, Debug)]
struct Seeds(Vec<u64>);

#[derive(Clone, Debug)]
struct Mappings(Vec<Vec<u64>>);

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, mappings_vec) = parse_almanac(input).unwrap();

    seeds.0.iter().map(|&s| {
        mappings_vec.iter().fold(s, |n, m| convert_number(n, m).unwrap())
    }).min()
}

// Takes a while
pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, mappings_vec) = parse_almanac(input).unwrap();
    let mut mappings_vec_rev = mappings_vec.clone();
    mappings_vec_rev.reverse();

    (0..).find(|&loc| {
        let gen_seed = mappings_vec_rev.iter()
            .fold(loc, |n, m| get_source(n, m).unwrap());

        seeds.0.chunks(2).any(|pair| {
            let s = pair[0];
            let r = pair[1];

            (s..s+r).contains(&gen_seed)
        })
    })
}

fn get_source(n: u64, mappings: &Mappings) -> Option<u64> {
    for l in mappings.0.iter() {
        let (dest, source, length) = (l[0], l[1], l[2]);

        if dest <= n && n < dest + length {
            return Some(source + (n-dest));
        }
    }
    Some(n) // numbers that aren't mapped stay the same
}

fn convert_number(n: u64, mappings: &Mappings) -> Option<u64> {
    for l in mappings.0.iter() {
        let (dest, source, length) = (l[0], l[1], l[2]);

        if source <= n && n < source + length {
            return Some(dest + (n-source));
        }
    }

    Some(n) // numbers that aren't mapped stay the same
}

fn parse_almanac(input: &str) -> Option<(Seeds, Vec<Mappings>)> {
    let seeds = Seeds(
        input.lines().next().unwrap().split(':').nth(1).unwrap()
        .split_whitespace().map(|n| n.parse().unwrap()).collect()
    );

    let mappings_vec: Vec<Mappings> = input.split("\n\n").skip(1)
        .map(|m| {
            Mappings(
                m.lines().skip(1).map(|l|
                    l.split_whitespace().map(|n| n.parse().unwrap()).collect()
                ).collect()
            )
        }).collect();

    Some((seeds, mappings_vec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
