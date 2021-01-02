use parser::{any, endline_terminated, fixed, make_parser, many1, unsigned_number};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Forward,
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Move {
    direction: Direction,
    amount: u64,
}

impl Move {
    fn new(direction: Direction, amount: u64) -> Move {
        Move { direction, amount }
    }

    fn parse(input: &str) -> Option<(Self, &str)> {
        let (dir, input) = any!(
            make_parser!(fixed, "L"),
            make_parser!(fixed, "R"),
            make_parser!(fixed, "F"),
            make_parser!(fixed, "N"),
            make_parser!(fixed, "S"),
            make_parser!(fixed, "W"),
            make_parser!(fixed, "E")
        )(input)
        .map(|(d, r)| {
            let dir = match d {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "F" => Direction::Forward,
                "N" => Direction::North,
                "S" => Direction::South,
                "W" => Direction::West,
                "E" => Direction::East,
                _ => unreachable!(),
            };
            (dir, r)
        })?;
        let (amount, input) = unsigned_number(input)?;
        Some((Move::new(dir, amount), input))
    }
}

struct Ship {
    position: (i64, i64),
    step: (i64, i64),
    waypoint: (i64, i64),
}

impl Ship {
    fn new() -> Ship {
        Ship {
            position: (0, 0),
            step: (1, 0),
            waypoint: (10, 1),
        }
    }

    fn move_1(&mut self, movement: Move) {
        use Direction::*;
        let (x, y) = self.position;
        let amount = movement.amount as i64;
        self.position = match movement.direction {
            East => (x + amount, y),
            West => (x - amount, y),
            South => (x, y - amount),
            North => (x, y + amount),
            Forward => {
                let (dx, dy) = self.step;
                (x + dx * amount, y + dy * amount)
            }
            Right | Left => {
                assert_eq!(movement.amount % 90, 0);
                let quarter_turns = movement.amount / 90;
                let steps = if movement.direction == Right {
                    [(1, 0), (0, -1), (-1, 0), (0, 1)]
                } else {
                    [(1, 0), (0, 1), (-1, 0), (0, -1)]
                };
                let current_step_pos = steps
                    .iter()
                    .position(|&p| p == self.step)
                    .expect("Illegal value assigned to step");
                let new_step_pos = (current_step_pos + quarter_turns as usize) % steps.len();
                self.step = steps[new_step_pos];
                self.position
            }
        };
    }

    fn move_2(&mut self, movement: Move) {
        use Direction::*;
        let (own_x, own_y) = self.position;
        let (wp_x, wp_y) = self.waypoint;
        let amount = movement.amount as i64;
        match movement.direction {
            East => self.waypoint = (wp_x + amount, wp_y),
            West => self.waypoint = (wp_x - amount, wp_y),
            South => self.waypoint = (wp_x, wp_y - amount),
            North => self.waypoint = (wp_x, wp_y + amount),
            Forward => self.position = (own_x + wp_x * amount, own_y + wp_y * amount),
            Right | Left => {
                assert_eq!(movement.amount % 90, 0);
                let quarter_turns = movement.amount / 90;
                for _ in 0..quarter_turns {
                    let (wp_x, wp_y) = self.waypoint;
                    if movement.direction == Right {
                        self.waypoint = (wp_y, -wp_x);
                    } else {
                        self.waypoint = (-wp_y, wp_x);
                    }
                }
            }
        };
    }
}

fn main() {
    let input = include_str!("input");
    let (moves, rest) =
        many1(make_parser!(endline_terminated, Move::parse), input).expect("Failed to parse input");
    assert!(rest.is_empty());

    let mut ship = Ship::new();
    for m in moves.iter() {
        ship.move_1(*m);
    }
    let (x, y) = ship.position;
    let manhattan_distance = x.abs() + y.abs();
    println!("Part 1: Manhattan distance: {}", manhattan_distance);

    let mut ship = Ship::new();
    for m in moves.iter() {
        ship.move_2(*m);
    }
    let (x, y) = ship.position;
    let manhattan_distance = x.abs() + y.abs();
    println!("Part 2: Manhattan distance: {}", manhattan_distance);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_move() {
        use Direction::*;
        for (input, expected) in [
            ("N1", Move::new(North, 1)),
            ("S10", Move::new(South, 10)),
            ("E9", Move::new(East, 9)),
            ("W2", Move::new(West, 2)),
            ("L270", Move::new(Left, 270)),
            ("R90", Move::new(Right, 90)),
            ("F12", Move::new(Forward, 12)),
        ]
        .iter()
        {
            assert_eq!(Move::parse(input), Some((*expected, "")));
        }
    }

    #[test]
    fn move_ship_1() {
        use Direction::*;
        let mut ship = Ship::new();
        ship.move_1(Move::new(Forward, 10));
        assert_eq!(ship.position, (10, 0));
        ship.move_1(Move::new(North, 3));
        assert_eq!(ship.position, (10, 3));
        ship.move_1(Move::new(Forward, 7));
        assert_eq!(ship.position, (17, 3));
        ship.move_1(Move::new(Right, 90));
        assert_eq!(ship.position, (17, 3));
        ship.move_1(Move::new(Forward, 11));
        assert_eq!(ship.position, (17, -8));
        ship.move_1(Move::new(Left, 270));
        assert_eq!(ship.position, (17, -8));
        ship.move_1(Move::new(Forward, 7));
        assert_eq!(ship.position, (10, -8));
    }

    #[test]
    fn move_ship_2() {
        use Direction::*;
        let mut ship = Ship::new();
        ship.move_2(Move::new(Forward, 10));
        assert_eq!(ship.position, (100, 10));
        ship.move_2(Move::new(North, 3));
        assert_eq!(ship.position, (100, 10));
        assert_eq!(ship.waypoint, (10, 4));
        ship.move_2(Move::new(Forward, 7));
        assert_eq!(ship.position, (170, 38));
        ship.move_2(Move::new(Right, 90));
        assert_eq!(ship.position, (170, 38));
        assert_eq!(ship.waypoint, (4, -10));
        ship.move_2(Move::new(Forward, 11));
        assert_eq!(ship.position, (214, -72));
        ship.move_2(Move::new(Left, 90));
        assert_eq!(ship.position, (214, -72));
        assert_eq!(ship.waypoint, (10, 4));
    }
}
