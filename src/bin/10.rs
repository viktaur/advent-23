use prse::{parse, Parse};
use itertools::Itertools;
advent_of_code::solution!(10);

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Tile {
    i: usize,
    j: usize,
    tile_type: TileType
}

impl Tile {
    fn adj(&self, tiles: &[Vec<Tile>], dir: Direction) -> Option<Self> {
        let i = match dir {
            Direction::Up => self.i.checked_sub(1)?,
            Direction::Down => self.i.checked_add(1)?,
            _ => self.i
        };

        let j = match dir {
            Direction::Left => self.j.checked_sub(1)?,
            Direction::Right => self.j.checked_add(1)?,
            _ => self.j
        };

        tiles.get(i)?.get(j).cloned()
    }
}

#[derive(Parse, Eq, PartialEq, Copy, Clone, Debug)]
enum TileType {
    #[prse = "|"]
    Vertical,
    #[prse = "-"]
    Horizontal,
    #[prse = "L"]
    BendNE,
    #[prse = "J"]
    BendNW,
    #[prse = "7"]
    BendSW,
    #[prse = "F"]
    BendSE,
    #[prse = "."]
    Ground,
    #[prse = "S"]
    Start
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Eq, PartialEq, Debug)]
struct Step {
    tile: Tile,
    dir: Direction
}

impl Step {
    fn new(tile: Tile, dir: Direction) -> Self {
        Self {
            tile,
            dir
        }
    }

    fn from(&self, tiles: &[Vec<Tile>], dir: Direction) -> Option<Self> {
        Some(Step::new(self.tile.adj(tiles, dir)?, dir))
    }

    // The Direction is where they're coming from
    fn next(&self, tiles: &[Vec<Tile>]) -> Option<Step> {
        let new_dir = match (self.tile.tile_type, self.dir) {

            (TileType::Vertical, Direction::Down) => Direction::Down,
            (TileType::BendNE, Direction::Down) => Direction::Right,
            (TileType::BendNW, Direction::Down) => Direction::Left,

            (TileType::Vertical, Direction::Up) => Direction::Up,
            (TileType::BendSE, Direction::Up) => Direction::Right,
            (TileType::BendSW, Direction::Up) => Direction::Left,

            (TileType::Horizontal, Direction::Left) => Direction::Left,
            (TileType::BendSE, Direction::Left) => Direction::Down,
            (TileType::BendNE, Direction::Left) => Direction::Up,

            (TileType::Horizontal, Direction::Right) => Direction::Right,
            (TileType::BendSW, Direction::Right) => Direction::Down,
            (TileType::BendNW, Direction::Right) => Direction::Up,

            _ => { return None; }
        };

        self.from(tiles, new_dir)
    }
}

fn parse_tiles(input: &str) -> Vec<Vec<Tile>> {
    input.lines().enumerate().map(|(i, l)| {
        l.chars().enumerate().map(|(j, t)| {
            Tile { i, j, tile_type: parse!(t.to_string(), "{}")}
        }).collect()
    }).collect()
}

fn find_start(tiles: &[Vec<Tile>]) -> Option<Tile> {
    for v in tiles {
        for t in v {
            if matches!(t.tile_type, TileType::Start) {
                return Some(*t);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let tiles = parse_tiles(input);
    let start_tile = find_start(&tiles)?;

    let mut cur_steps: Vec<Step> = [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
        .into_iter()
        .filter_map(|dir| Some(Step::new(start_tile.adj(&tiles, dir)?, dir)))
        .filter(|s| s.next(&tiles).is_some())
        .collect();

    let mut count = 1;

    while !cur_steps.iter().map(|s| s.tile).all_equal() {
        cur_steps = cur_steps.iter().map(|s| s.next(&tiles).unwrap()).collect();
        count += 1;
    }

    Some(count)
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
