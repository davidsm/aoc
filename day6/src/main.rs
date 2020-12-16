use std::collections::{HashMap, HashSet};

use parser::{endline, fixed, many1, optional, take_while1};

fn blank_line(input: &str) -> Option<(&str, &str)> {
    fixed("\n\n", input)
}

fn parse_answers(input: &str) -> Option<(&str, &str)> {
    let (_, input) = optional(endline, input);
    take_while1(|c| c.is_ascii_lowercase(), input)
}

fn parse_group(input: &str) -> Option<(Vec<&str>, &str)> {
    let (group, input) = many1(parse_answers, input)?;
    let (_, input) = optional(blank_line, input);
    Some((group, input))
}

fn sum_everyone_answered(group: &[&str]) -> usize {
    let mut counts_map = HashMap::new();
    for c in group.iter().flat_map(|a| a.chars()) {
        let char_count = counts_map.entry(c).or_insert(0);
        *char_count += 1;
    }
    counts_map.values().filter(|&v| *v == group.len()).count()
}

fn main() {
    let input = include_str!("input");
    let (groups, input) = many1(parse_group, input).unwrap();
    assert_eq!(input, "");
    let sum_answers_1: usize = groups
        .iter()
        .map(|g| {
            g.iter()
                .flat_map(|a| a.chars())
                .fold(HashSet::new(), |mut s, a| {
                    s.insert(a);
                    s
                })
        })
        .map(|s| s.len())
        .sum();
    println!("Part 1: Sum of answers: {}", sum_answers_1);
    let sum_answers_2: usize = groups.iter().map(|g| sum_everyone_answered(g)).sum();
    println!("Part 2: Sum of answers: {}", sum_answers_2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_answer() {
        let res = parse_answers("ab\nc");
        assert_eq!(res, Some(("ab", "\nc")));

        let res = parse_answers("\nc");
        assert_eq!(res, Some(("c", "")));

        let res = parse_answers("");
        assert!(res.is_none());
    }

    #[test]
    fn test_parse_group() {
        let res = parse_group("ab\na\n\nc");
        assert_eq!(res, Some((vec!["ab", "a"], "c")));

        let res = parse_group("c");
        assert_eq!(res, Some((vec!["c"], "")));

        let res = parse_group("");
        assert!(res.is_none());
    }
}
