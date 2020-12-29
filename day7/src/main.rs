use std::collections::HashMap;

use parser::{endline, fixed, many1, optional, unsigned_number, words};

type BagsMap<'a> = HashMap<&'a str, Vec<(u64, &'a str)>>;

fn parse_bag_line(input: &str) -> Option<((&str, Vec<(u64, &str)>), &str)> {
    let (color, input) = words(2, input)?;
    let (_, input) = fixed(" bags contain ", input)?;
    let (contains, input) = parse_contained_bags(input)?;
    let (_, input) = optional(endline, input);
    Some(((color, contains), input))
}

fn parse_contained_bags(mut input: &str) -> Option<(Vec<(u64, &str)>, &str)> {
    let bags = if let Some((_, rest)) = fixed("no other bags", input) {
        input = rest;
        vec![]
    } else {
        let (bags, rest) = many1(parse_contained_bag, input)?;
        input = rest;
        bags
    };
    let (_, input) = fixed(".", input)?;
    Some((bags, input))
}

fn parse_contained_bag(input: &str) -> Option<((u64, &str), &str)> {
    let (amount, input) = unsigned_number(input)?;
    let (_, input) = fixed(" ", input)?;
    let (color, input) = words(2, input)?;
    let (_, input) = fixed(" bag", input)?;
    let (_, input) = optional(|inp| fixed("s", inp), input);
    let (_, input) = optional(|inp| fixed(", ", inp), input);
    Some(((amount, color), input))
}

fn contains_bag_color<'a>(
    target_color: &str,
    contents: &[(u64, &'a str)],
    map: &'a BagsMap,
) -> bool {
    for (_, color) in contents {
        if color == &target_color
            || contains_bag_color(target_color, map.get(color).unwrap().as_ref(), map)
        {
            return true;
        }
    }
    false
}

fn count_bags_inside<'a>(contents: &[(u64, &'a str)], map: &'a BagsMap) -> u64 {
    let mut total_count = 0;
    for (count, color) in contents {
        total_count += count;
        total_count += count * count_bags_inside(map.get(color).unwrap().as_ref(), map);
    }
    total_count
}

fn main() {
    let input = include_str!("input");
    let (bags, rest) = many1(parse_bag_line, input).unwrap();
    assert_eq!(rest, "");
    let bags_map = bags.into_iter().collect::<BagsMap>();
    // Horribly inefficient, but meh
    let count_1 = bags_map
        .iter()
        .filter(|(_color, contents)| contains_bag_color("shiny gold", contents, &bags_map))
        .count();
    println!("Part 1: Number of bags {}", count_1);
    let count_2 = count_bags_inside(bags_map["shiny gold"].as_ref(), &bags_map);
    println!("Part 2: Number of bags {}", count_2);
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! bag_tests {
        ($($func:ident : ($input:expr, $expected:expr),)*) => {
            $(
                #[test]
                fn $func() {
                    let input = $input;
                    let res = parse_bag_line(input);
                    assert_eq!(res, Some(($expected, "")));
                }
            )*
        };
    }

    bag_tests! {
        empty_bag : ("faded blue bags contain no other bags.",
                     ("faded blue", vec![])),
        one_other_bag : ("bright white bags contain 1 shiny gold bag.",
                         ("bright white", vec![(1, "shiny gold")])),
        two_other_bags: ("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
                         ("vibrant plum", vec![(5, "faded blue"), (6, "dotted black")])),

    }
}
