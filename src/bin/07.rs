use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use prse::{parse, Parse};
advent_of_code::solution!(7);

#[derive(Parse, Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Debug)]
enum Card {
    #[prse = "A"]
    A,
    #[prse = "K"]
    K,
    #[prse = "Q"]
    Q,
    #[prse = "J"]
    J,
    #[prse = "T"]
    T,
    #[prse = "9"]
    Nine,
    #[prse = "8"]
    Eight,
    #[prse = "7"]
    Seven,
    #[prse = "6"]
    Six,
    #[prse = "5"]
    Five,
    #[prse = "4"]
    Four,
    #[prse = "3"]
    Three,
    #[prse = "2"]
    Two
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(PartialEq, Hash, Eq, Clone, Debug)]
struct Hand(Vec<Card>);

impl Hand {
    fn get_type(&self) -> HandType {
        let mut heap = self.get_frequencies();

        match (heap.pop(), heap.pop()) {
            (Some(5), None) => HandType::FiveKind,
            (Some(4), Some(1)) => HandType::FourKind,
            (Some(3), Some(2)) => HandType::FullHouse,
            (Some(3), Some(1)) => HandType::ThreeKind,
            (Some(2), Some(2)) => HandType::TwoPair,
            (Some(2), Some(1)) => HandType::OnePair,
            (Some(1), Some(1)) => HandType::HighCard,
            _ => panic!("Unknown hand type")
        }
    }

    fn get_frequencies(&self) -> BinaryHeap<u32> {
        self.clone().0.into_iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        }).into_values().collect() // sort cards by frequency
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_type().cmp(&other.get_type()) {
            Ordering::Equal => self.0.cmp(&other.0),
            o => o
        }
    }
}

#[derive(Parse, Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Debug)]
enum Card2 {
    #[prse = "A"]
    A,
    #[prse = "K"]
    K,
    #[prse = "Q"]
    Q,
    #[prse = "T"]
    T,
    #[prse = "9"]
    Nine,
    #[prse = "8"]
    Eight,
    #[prse = "7"]
    Seven,
    #[prse = "6"]
    Six,
    #[prse = "5"]
    Five,
    #[prse = "4"]
    Four,
    #[prse = "3"]
    Three,
    #[prse = "2"]
    Two,
    #[prse = "J"]
    J,
}

#[derive(PartialEq, Hash, Eq, Clone, Debug)]
struct Hand2(Vec<Card2>);

impl Hand2 {
    fn get_type(&self) -> HandType {
        let mut heap = self.get_frequencies();
        let n_js = self.0.iter().filter(|&c| matches!(c, Card2::J)).count() as u32;

        let pre_type = match (heap.pop(), heap.pop()) {
            (Some(4), _) => HandType::FourKind,
            (Some(3), Some(2)) => HandType::FullHouse,
            (Some(3), Some(1)) => HandType::ThreeKind,
            (Some(2), Some(2)) => HandType::TwoPair,
            (Some(2), Some(1)) => HandType::OnePair,
            (Some(1), Some(1)) => HandType::HighCard,
            _ => HandType::FiveKind
        };

        match pre_type {
            HandType::FourKind if n_js == 1 => HandType::FiveKind,

            HandType::ThreeKind if n_js == 2 => HandType::FiveKind,
            HandType::ThreeKind if n_js == 1 => HandType::FourKind,

            HandType::TwoPair if n_js == 1 => HandType::FullHouse,

            HandType::OnePair if n_js == 3 => HandType::FiveKind,
            HandType::OnePair if n_js == 2 => HandType::FourKind,
            HandType::OnePair if n_js == 1 => HandType::ThreeKind,

            HandType::HighCard if n_js == 4 => HandType::FiveKind,
            HandType::HighCard if n_js == 3 => HandType::FourKind,
            HandType::HighCard if n_js == 2 => HandType::ThreeKind,
            HandType::HighCard if n_js == 1 => HandType::OnePair,

            t => t
        }
    }

    fn get_frequencies(&self) -> BinaryHeap<u32> {
        self.clone().0.into_iter().fold(HashMap::new(), |mut acc, c| {
            if !matches!(c, Card2::J) {
                *acc.entry(c).or_insert(0) += 1;
            }
            acc
        }).into_values().collect() // sort cards by frequency
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_type().cmp(&other.get_type()) {
            Ordering::Equal => self.0.cmp(&other.0),
            o => o
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: BinaryHeap<(Hand, u32)> = BinaryHeap::new();

    for line in input.lines() {
        let (hand, bid): (Vec<Card>, u32) = parse!(line, "{::} {}");
        // println!("Pushing hand {:?} to the binary heap", (hand.clone(), bid));
        hands.push((Hand(hand), bid));
        // println!("Binary heap: {:?}", hands);
    }

    Some(
        (1..=hands.len() as u32).map(|i| {
            i * hands.pop().unwrap().1
        }).sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: BinaryHeap<(Hand2, u32)> = BinaryHeap::new();

    for line in input.lines() {
        let (hand, bid): (Vec<Card2>, u32) = parse!(line, "{::} {}");
        hands.push((Hand2(hand), bid));
    }

    Some(
        (1..=hands.len() as u32).map(|i| {
            i * hands.pop().unwrap().1
        }).sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
