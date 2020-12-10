#![feature(iterator_fold_self)]

use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
struct Map {
    pub height: usize,
    pub width: usize,

    pub data: Vec<Vec<Tile>>,
}

impl Map {
    fn check_position(&self, pos: &Position) -> Tile {
        assert!(pos.y < self.height);
        let wrapped_x_width = pos.x % self.width;
        self.data[pos.y][wrapped_x_width]
    }

    fn parse(raw_map: &str) -> Self {
        let mut width = 0;
        let mut data = Vec::new();

        for line in raw_map.lines() {
            let mut row = Vec::new();

            for tile_char in line.trim().chars() {
                row.push(Tile::from(tile_char));
            }

            data.push(row);
        }

        let height = data.len();
        if height > 0 {
            width = data[0].len();
        }

        Self {
            height,
            width,
            data,
        }
    }

    fn collisions_along_slope(&self, tgt: Tile, pos: Position, slope: &Slope) -> usize {
        let mut current_position = pos;
        let mut collisions = 0;

        assert!(slope.y > 0);

        loop {
            if self.check_position(&current_position) == tgt {
                collisions += 1;
            }

            current_position.add_slope(&slope);
            if current_position.y >= self.height {
                return collisions;
            }
        }
    }
}

struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn add_slope(&mut self, slope: &Slope) {
        self.x += slope.x;
        self.y += slope.y;
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

struct Slope {
    pub x: usize,
    pub y: usize,
}

impl Slope {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Tree,
    Empty,
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Tile::Tree,
            _ => Tile::Empty,
        }
    }
}

fn main() {
    let mut in_dat_fh = File::open("./data/input1.txt").unwrap();
    let mut in_dat = String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();

    let map = Map::parse(&in_dat);
    let collision_count =
        map.collisions_along_slope(Tile::Tree, Position::default(), &Slope::new(3, 1));

    println!("Map had {} collisions with trees", collision_count);

    let testable_slopes = vec![
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ];

    let slope_product = testable_slopes
        .iter()
        .map(|s| map.collisions_along_slope(Tile::Tree, Position::default(), s))
        .fold_first(|acc, x| acc * x)
        .unwrap();

    println!("Product of the test slopes was: {}", slope_product);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_MAP: &str = "..##.......\n\
                              #...#...#..\n\
                              .#....#..#.\n\
                              ..#.#...#.#\n\
                              .#...##..#.\n\
                              ..#.##.....\n\
                              .#.#.#....#\n\
                              .#........#\n\
                              #.##...#...\n\
                              #...##....#\n\
                              .#..#...#.#";

    const SMALL_MAP: &str = "..##\n\
                             #..#";

    const TEST_SLOPES: [Slope; 5] = [
        Slope { x: 1, y: 1 },
        Slope { x: 3, y: 1 },
        Slope { x: 5, y: 1 },
        Slope { x: 7, y: 1 },
        Slope { x: 1, y: 2 },
    ];

    #[test]
    fn test_small_map_parsing() {
        let map = Map::parse(SMALL_MAP);

        assert_eq!(map.height, 2);
        assert_eq!(map.width, 4);

        let expected_map_data = vec![
            vec![Tile::Empty, Tile::Empty, Tile::Tree, Tile::Tree],
            vec![Tile::Tree, Tile::Empty, Tile::Empty, Tile::Tree],
        ];

        assert_eq!(map.data, expected_map_data);
    }

    #[test]
    fn test_sample_map_partial_parsing() {
        let map = Map::parse(SAMPLE_MAP);

        assert_eq!(map.height, 11);
        assert_eq!(map.width, 11);
    }

    #[test]
    fn test_collision_along_slope() {
        let map = Map::parse(SAMPLE_MAP);
        let collisions =
            map.collisions_along_slope(Tile::Tree, Position::default(), &Slope::new(3, 1));
        assert_eq!(collisions, 7);
    }

    #[test]
    fn test_sample_tile_parsing() {
        assert_eq!(Tile::from('#'), Tile::Tree);
        assert_eq!(Tile::from('.'), Tile::Empty);
    }

    #[test]
    fn test_summed_slope() {
        let map = Map::parse(SAMPLE_MAP);
        let result: usize = TEST_SLOPES
            .iter()
            .map(|s| map.collisions_along_slope(Tile::Tree, Position::default(), s))
            .fold_first(|acc, x| acc * x)
            .unwrap();

        assert_eq!(result, 336);
    }
}
