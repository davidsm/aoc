#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Tree,
    Open,
}

#[derive(Debug, PartialEq)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
}

impl Map {
    fn parse(map_str: &str) -> Self {
        let mut tiles = Vec::with_capacity(map_str.len());
        let mut width: Option<usize> = None;
        for (i, ch) in map_str.chars().enumerate() {
            let tile = match ch {
                '.' => Tile::Open,
                '#' => Tile::Tree,
                c if c.is_ascii_whitespace() => {
                    let _ = width.get_or_insert(i);
                    continue;
                }
                u @ _ => panic!("Unexpected input {}", u),
            };
            tiles.push(tile);
        }
        Map {
            tiles: tiles,
            width: width.expect("No row break encountered"),
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        let wrapped_x = x % self.width;
        self.tiles.get(wrapped_x + y * self.width).copied()
    }
}

fn count_trees(x_incr: usize, y_incr: usize, map: &Map) -> u64 {
    let (mut x, mut y) = (0, 0);
    let mut trees = 0;
    while let Some(tile) = map.get_tile(x, y) {
        if tile == Tile::Tree {
            trees += 1;
        }
        x += x_incr;
        y += y_incr;
    }
    trees
}

fn main() {
    let input = include_str!("input");
    let map = Map::parse(input);

    let first_count = count_trees(3, 1, &map);
    println!("Part 1: {} trees hit", first_count);

    let mut product: u64 = [(1, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x_incr, y_incr)| count_trees(*x_incr, *y_incr, &map))
        .product();

    product *= first_count;
    println!("Part 2: {} trees hit", product);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_input() {
        let input = ".#\n#.";
        let map = Map::parse(input);
        let expected = Map {
            tiles: vec![Tile::Open, Tile::Tree, Tile::Tree, Tile::Open],
            width: 2,
        };
        assert_eq!(map, expected);
    }

    #[test]
    fn get_tile_nowrap() {
        let map = Map {
            tiles: vec![Tile::Open, Tile::Open, Tile::Tree, Tile::Open],
            width: 2,
        };
        assert_eq!(map.get_tile(0, 1), Some(Tile::Tree));
    }

    #[test]
    fn get_tile_wrap() {
        let map = Map {
            tiles: vec![Tile::Open, Tile::Open, Tile::Tree, Tile::Open],
            width: 2,
        };
        assert_eq!(map.get_tile(2, 1), Some(Tile::Tree));
    }

    #[test]
    fn get_tile_outside() {
        let map = Map {
            tiles: vec![Tile::Open, Tile::Open, Tile::Tree, Tile::Open],
            width: 2,
        };
        assert_eq!(map.get_tile(0, 2), None);
    }
}
