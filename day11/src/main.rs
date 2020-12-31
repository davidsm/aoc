use parser::{any, either, endline, eof, fixed, make_parser};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, PartialEq)]
struct Seats {
    tiles: Vec<Tile>,
    width: usize,
}

impl Seats {
    fn parse(mut input: &str) -> Option<Seats> {
        let mut tiles = Vec::new();

        let width = input.find('\n')?;
        let parse_tile = any!(
            make_parser!(fixed, "."),
            make_parser!(fixed, "L"),
            make_parser!(fixed, "#")
        );

        let parse_tile = |inp| {
            parse_tile(inp).and_then(|(t, r)| match t {
                "." => Some((Tile::Floor, r)),
                "L" => Some((Tile::Empty, r)),
                "#" => Some((Tile::Occupied, r)),
                _ => None,
            })
        };

        while !input.is_empty() {
            for _ in 0..width {
                let (tile, rest) = parse_tile(input)?;
                tiles.push(tile);
                input = rest;
            }
            let (_, rest) = either(endline, eof, input)?;
            input = rest;
        }
        assert!(input.is_empty());
        Some(Seats { tiles, width })
    }

    fn get_adjacents(&self, x: usize, y: usize) -> Vec<Tile> {
        let mut tiles = Vec::new();
        for yi in y.saturating_sub(1)..=(y + 1) {
            for xi in x.saturating_sub(1)..=(x + 1) {
                if xi == x && yi == y {
                    continue;
                }
                if let Some(tile) = self.get_tile(xi, yi) {
                    tiles.push(tile)
                }
            }
        }
        tiles
    }

    fn simulate(&mut self) -> bool {
        let mut new_tiles = Vec::new();
        for (i, tile) in self.tiles.iter().enumerate() {
            let x = i % self.width;
            let y = i / self.width;
            let occupied_adjacents = self
                .get_adjacents(x, y)
                .iter()
                .filter(|&&t| t == Tile::Occupied)
                .count();
            let new_tile = match (tile, occupied_adjacents) {
                (Tile::Empty, 0) => Tile::Occupied,
                (Tile::Occupied, n) if n >= 4 => Tile::Empty,
                _ => *tile,
            };
            new_tiles.push(new_tile);
        }
        let ret = new_tiles == self.tiles;
        self.tiles = new_tiles;
        ret
    }

    fn simulate_2(&mut self) -> bool {
        let mut new_tiles = Vec::new();
        let dirs = [
            (-1, -1),
            (-1, 0),
            (0, -1),
            (1, 1),
            (1, 0),
            (0, 1),
            (1, -1),
            (-1, 1),
        ];

        for (i, tile) in self.tiles.iter().enumerate() {
            let mut visible_occupied = 0;
            let current_x = i % self.width;
            let current_y = i / self.width;
            for (dx, dy) in dirs.iter() {
                let mut x = current_x as isize + dx;
                let mut y = current_y as isize + dy;
                loop {
                    if x < 0 || y < 0 {
                        break;
                    }
                    match self.get_tile(x as usize, y as usize) {
                        Some(Tile::Occupied) => {
                            visible_occupied += 1;
                            break;
                        }
                        Some(Tile::Empty) | None => {
                            break;
                        }
                        Some(Tile::Floor) => {
                            x += dx;
                            y += dy;
                        }
                    };
                }
            }
            let new_tile = match (tile, visible_occupied) {
                (Tile::Empty, 0) => Tile::Occupied,
                (Tile::Occupied, n) if n >= 5 => Tile::Empty,
                _ => *tile,
            };
            new_tiles.push(new_tile);
        }
        let ret = new_tiles == self.tiles;
        self.tiles = new_tiles;
        ret
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x >= self.width {
            return None;
        }
        self.tiles.get(x + y * self.width).copied()
    }
}

fn main() {
    let input = include_str!("input");
    let mut seats = Seats::parse(input).unwrap();

    while !seats.simulate() {}
    let final_occupied_count_1 = seats.tiles.iter().filter(|&&t| t == Tile::Occupied).count();
    println!(
        "Part 1: final number of occupied seats: {}",
        final_occupied_count_1
    );

    let mut seats = Seats::parse(input).unwrap();

    while !seats.simulate_2() {}
    let final_occupied_count_2 = seats.tiles.iter().filter(|&&t| t == Tile::Occupied).count();
    println!(
        "Part 2: final number of occupied seats: {}",
        final_occupied_count_2
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_seats() {
        use Tile::*;
        let input = "#.L\nL#.\n.#L";
        let seats = Seats::parse(input);
        assert_eq!(
            seats,
            Some(Seats {
                tiles: vec![Occupied, Floor, Empty, Empty, Occupied, Floor, Floor, Occupied, Empty],
                width: 3
            })
        );
    }

    #[test]
    fn test_get_adjacents() {
        use Tile::*;
        let tiles = vec![
            Floor, Floor, Floor, Empty, Empty, Empty, Occupied, Occupied, Occupied,
        ];
        let seats = Seats { tiles, width: 3 };
        assert_eq!(seats.get_adjacents(0, 0), vec![Floor, Empty, Empty]);
        assert_eq!(
            seats.get_adjacents(1, 2),
            vec![Empty, Empty, Empty, Occupied, Occupied]
        );
        assert_eq!(
            seats.get_adjacents(2, 1),
            vec![Floor, Floor, Empty, Occupied, Occupied]
        );
        assert_eq!(
            seats.get_adjacents(1, 1),
            vec![Floor, Floor, Floor, Empty, Empty, Occupied, Occupied, Occupied]
        );
    }
}
