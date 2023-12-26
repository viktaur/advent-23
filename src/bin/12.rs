use prse::{Parse, parse};
advent_of_code::solution!(12);

#[derive(Parse, Copy, Clone, PartialEq, Debug)]
enum Condition {
    #[prse = "."]
    Operational,
    #[prse = "#"]
    Damaged,
    #[prse = "?"]
    Unknown
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines().map(|line| {
            let (conditions, groups): (Vec<Condition>, Vec<usize>) = parse!(line, "{::} {:,:}");
            arrangements(conditions, &groups, 0)
        }).sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input.lines().map(|line| {
            let (conditions, groups): (Vec<Condition>, Vec<usize>) = parse!(line, "{::} {:,:}");
            arrangements(unfold_conditions(conditions), &unfold_groups(groups), 0)
        }).sum()
    )
}

fn unfold_conditions(conditions: Vec<Condition>) -> Vec<Condition> {
    (1..5).fold(conditions.clone(), |mut acc, _| {
        acc.push(Condition::Unknown);
        acc.extend(&conditions);
        acc
    })
}

fn unfold_groups(groups: Vec<usize>) -> Vec<usize> {
    (1..5).fold(groups.clone(), |mut acc, _| {
        acc.extend(&groups);
        acc
    })
}

fn arrangements(conditions: Vec<Condition>, groups: &Vec<usize>, index: usize) -> u32 {
    match possibilities(&conditions, index) {
        Some((v1, v2)) => {

            let left = {
                if is_valid_slice(&v1[..index], groups) {
                    arrangements(v1, groups, index + 1)
                } else { 0 } // pruning
            };

            let right = {
                if is_valid_slice(&v2[..index], groups) {
                    arrangements(v2, groups, index + 1)
                } else { 0 }
            };

            left + right
        },
        None => {
            if index < conditions.len()-1 {
                arrangements(conditions, groups, index + 1)
            } else if is_valid(&conditions, groups) { 1 } else { 0 }
        }
    }
}

fn is_valid(conditions: &[Condition], groups: &Vec<usize>) -> bool {
    get_layout(conditions).eq(groups)
}

fn is_valid_slice(conditions: &[Condition], groups: &Vec<usize>) -> bool {
    let layout = get_layout(conditions);
    let l = layout.len();

    if l > 0 {
        for i in 0..l-1 {
            if layout.get(i) != groups.get(i) {
                return false;
            }
        }

        match (layout.last(), groups.get(l - 1)) {
            (Some(a), Some(b)) => a <= b,
            _ => false
        }
    } else {
        true
    }
}

fn get_layout(conditions: &[Condition]) -> Vec<usize> {
    conditions
        .split(|&c| matches!(c, Condition::Operational))
        // .inspect(|v| println!("{:?}", v))
        .map(|v| v.iter().filter(|&e| matches!(e, Condition::Damaged)).count())
        .filter(|&l| l != 0)
        .collect()
}

fn possibilities(conditions: &[Condition], index: usize) -> Option<(Vec<Condition>, Vec<Condition>)> {
    let update_vec = |mut v: Vec<Condition>, c| {v[index] = c; v};

    match conditions.get(index) {
        Some(Condition::Unknown) => Some((
            update_vec(conditions.to_vec(), Condition::Damaged),
            update_vec(conditions.to_vec(), Condition::Operational)
        )),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }

    #[test]
    fn test_get_layout() {
        assert_eq!(get_layout(&parse!("#?????###??#?.", "{::}")), vec![5]);
        assert_eq!(get_layout(&parse!("..??.?.?#?", "{::}")), vec![1]);
        assert_eq!(get_layout(&parse!(".###..##..#.", "{::}")), vec![3, 2, 1]);
    }

    #[test]
    fn test_unfold_conditions() {
        assert_eq!(
            unfold_conditions(parse!(".#", "{::}")),
            parse!(".#?.#?.#?.#?.#", "{::}")
        );

        assert_eq!(
            unfold_conditions(parse!("???.###", "{::}")),
            parse!("???.###????.###????.###????.###????.###", "{::}")
        );
    }

    #[test]
    fn test_unfold_groups() {
        assert_eq!(
            unfold_groups(vec![1]),
            vec![1,1,1,1,1]
        );

        assert_eq!(
            unfold_groups(vec![1,1,3]),
            vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3]
        )
    }
}
