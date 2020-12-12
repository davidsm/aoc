use parser::{endline, optional};
use std::collections::BTreeSet;

const TOTAL_ROWS: u8 = 128;
const TOTAL_COLUMNS: u8 = 8;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Part {
    Upper,
    Lower,
}

impl Part {
    fn from_row_char(c: char) -> Option<Self> {
        match c {
            'F' => Some(Self::Lower),
            'B' => Some(Self::Upper),
            _ => None,
        }
    }

    fn from_col_char(c: char) -> Option<Self> {
        match c {
            'L' => Some(Self::Lower),
            'R' => Some(Self::Upper),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Seat {
    row: [Part; 7],
    column: [Part; 3],
}

impl Seat {
    fn new(row: [Part; 7], column: [Part; 3]) -> Self {
        Seat { row, column }
    }

    fn id(&self) -> u32 {
        let row = bisect(self.row.iter(), TOTAL_ROWS - 1);
        let column = bisect(self.column.iter(), TOTAL_COLUMNS - 1);
        row as u32 * 8 + column as u32
    }
}

fn bisect<'a>(parts: impl Iterator<Item = &'a Part>, start_high: u8) -> u32 {
    let mut high = start_high;
    let mut half = (high + 1) / 2;
    for r in parts {
        if *r == Part::Lower {
            high -= half;
        }
        half /= 2;
    }
    high as u32
}

fn parse_row(input: &str) -> Option<(Part, &str)> {
    let (i, letter) = input.char_indices().next()?;
    Some((Part::from_row_char(letter)?, &input[i + 1..]))
}

fn parse_column(input: &str) -> Option<(Part, &str)> {
    let (i, letter) = input.char_indices().next()?;
    Some((Part::from_col_char(letter)?, &input[i + 1..]))
}

fn parse_seat(input: &str) -> Option<(Seat, &str)> {
    let mut rows = [Part::Upper; 7];
    let mut columns = [Part::Upper; 3];
    let mut _input = input;
    for row_slot in rows.iter_mut() {
        let (row, rest) = parse_row(_input)?;
        _input = rest;
        *row_slot = row;
    }
    for column_slot in columns.iter_mut() {
        let (column, rest) = parse_column(_input)?;
        _input = rest;
        *column_slot = column;
    }
    let (_, _input) = optional(endline, _input);
    Some((Seat::new(rows, columns), _input))
}

fn parse_seats(input: &str) -> Vec<Seat> {
    let mut seats = Vec::new();
    let mut _input = input;
    while let Some((seat, input)) = parse_seat(_input) {
        seats.push(seat);
        _input = input;
    }
    seats
}

fn main() {
    let input = include_str!("input");
    let seats = parse_seats(input);
    let ids = seats.iter().map(|s| s.id()).collect::<BTreeSet<u32>>();
    let highest = *ids.iter().max().unwrap();
    println!("Part 1: highest ID {}", highest);
    let yours = (0..=highest)
        .filter(|i| !ids.contains(i))
        .filter(|i| *i > 0 && ids.contains(&(i - 1)) && ids.contains(&(i + 1)))
        .next()
        .unwrap();
    println!("Part 2: your seat ID is {}", yours);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn seat_id() {
        use Part::*;
        let seat = Seat::new(
            [Lower, Upper, Lower, Upper, Upper, Lower, Lower],
            [Upper, Lower, Upper],
        );
        assert_eq!(seat.id(), 357);
        let seat = Seat::new(
            [Upper, Lower, Lower, Lower, Upper, Upper, Lower],
            [Upper, Upper, Upper],
        );
        assert_eq!(seat.id(), 567);
    }

    macro_rules! param_tests {
        ($func:ident, [$(($input:expr, $expected:expr),)*]) => {
            let tests = [$(
                ($input, $expected),
            )*];

            for (input, expected) in tests.iter() {
                let res = $func(input);
                assert_eq!(res, *expected);
            }
        }
    }

    #[test]
    fn test_parse_row() {
        param_tests!(
            parse_row,
            [
                ("FBFB", Some((Part::Lower, "BFB"))),
                ("BFBF", Some((Part::Upper, "FBF"))),
                ("L", None),
            ]
        );
    }

    #[test]
    fn test_parse_column() {
        param_tests!(
            parse_column,
            [
                ("LRLR", Some((Part::Lower, "RLR"))),
                ("RLRL", Some((Part::Upper, "LRL"))),
                ("F", None),
            ]
        );
    }

    #[test]
    fn test_parse_seat() {
        use Part::*;

        param_tests!(
            parse_seat,
            [
                (
                    "FFBBFFBLRL",
                    Some((
                        Seat {
                            row: [Lower, Lower, Upper, Upper, Lower, Lower, Upper],
                            column: [Lower, Upper, Lower],
                        },
                        "",
                    ))
                ),
                (
                    "FFBBFFBLRL\nFFBBFFBLRL",
                    Some((
                        Seat {
                            row: [Lower, Lower, Upper, Upper, Lower, Lower, Upper],
                            column: [Lower, Upper, Lower],
                        },
                        "FFBBFFBLRL",
                    ))
                ),
                ("FFBB", None),
            ]
        );
    }
}
