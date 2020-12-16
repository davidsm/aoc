// Stolen from Nom, more or less
// TODO: Figure out how to make this with &str instead of generic I...
pub trait Parser<O, I> {
    fn parse(&self, input: I) -> Option<(O, I)>;
}

impl<'a, I, O, F> Parser<O, I> for F
where
    F: Fn(I) -> Option<(O, I)> + 'a,
{
    fn parse(&self, i: I) -> Option<(O, I)> {
        self(i)
    }
}

pub fn take_while(pred: impl Fn(char) -> bool, input: &str) -> (&str, &str) {
    let mut i = 0;
    for (ci, c) in input.char_indices() {
        i = ci;
        if !pred(c) {
            break;
        }
        i += 1
    }
    (&input[..i], &input[i..])
}

pub fn take_while1(pred: impl Fn(char) -> bool, input: &str) -> Option<(&str, &str)> {
    let (matching, input) = take_while(pred, input);
    if !matching.is_empty() {
        Some((matching, input))
    } else {
        None
    }
}

pub fn take(length: usize, input: &str) -> Option<(&str, &str)> {
    let mut char_ind_iter = input.char_indices();
    let (ci, _) = char_ind_iter.nth(length - 1)?;
    let ci = char_ind_iter.next().map(|(ci, _)| ci).unwrap_or(ci + 1);
    Some((&input[..ci], &input[ci..]))
}

pub fn fixed<'a>(s: &str, input: &'a str) -> Option<(&'a str, &'a str)> {
    if let Some(rest) = input.strip_prefix(s) {
        Some((&input[..s.len()], rest))
    } else {
        None
    }
}

pub fn unsigned_number(input: &str) -> Option<(u32, &str)> {
    let (num_str, input) = take_while(|c| c.is_ascii_digit(), input);
    let num = num_str.parse::<u32>().ok()?;
    Some((num, input))
}

pub fn match_n(pred: impl Fn(char) -> bool, length: usize, input: &str) -> Option<(&str, &str)> {
    let (part, input) = take(length, input)?;
    if part.chars().all(pred) {
        Some((part, input))
    } else {
        None
    }
}

pub fn optional<'a, O>(parser: impl Parser<O, &'a str>, input: &'a str) -> (Option<O>, &'a str) {
    if let Some((res, rest)) = parser.parse(input) {
        (Some(res), rest)
    } else {
        (None, input)
    }
}

pub fn many1<'a, O>(parser: impl Parser<O, &'a str>, mut input: &'a str) -> Option<(Vec<O>, &str)> {
    let mut collected = Vec::new();
    while let Some((res, rest)) = parser.parse(input) {
        collected.push(res);
        input = rest;
    }

    if !collected.is_empty() {
        Some((collected, input))
    } else {
        None
    }
}

pub fn endline(input: &str) -> Option<(&str, &str)> {
    match_n(|c| c == '\n', 1, input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn take_while_with_matches() {
        let input = "1234abc";
        let res = take_while(|c| c.is_ascii_digit(), input);
        assert_eq!(res, ("1234", "abc"));
    }

    #[test]
    fn take_while_full_match() {
        let input = "1234";
        let res = take_while(|c| c.is_ascii_digit(), input);
        assert_eq!(res, ("1234", ""));
    }

    #[test]
    fn take_while_no_matches() {
        let input = "abc";
        let res = take_while(|c| c.is_ascii_digit(), input);
        assert_eq!(res, ("", "abc"));
    }

    #[test]
    fn take_while1_with_match() {
        let input = "1234abc";
        let res = take_while1(|c| c.is_ascii_digit(), input);
        assert_eq!(res, Some(("1234", "abc")));
    }

    #[test]
    fn take_while1_no_matches() {
        let input = "abc";
        let res = take_while1(|c| c.is_ascii_digit(), input);
        assert!(res.is_none());
    }

    #[test]
    fn take_within_range() {
        let input = "1234";
        let res = take(2, input);
        assert_eq!(res, Some(("12", "34")));
    }

    #[test]
    fn take_beyond_range() {
        let input = "1234";
        let res = take(5, input);
        assert!(res.is_none());
    }

    #[test]
    fn take_complete() {
        let input = "1234";
        let res = take(4, input);
        assert_eq!(res, Some(("1234", "")));
    }

    #[test]
    fn fixed_matches() {
        let input = "1234abc";
        let res = fixed("1234", input);
        assert_eq!(res, Some(("1234", "abc")));
    }

    #[test]
    fn fixed_no_match() {
        let input = "1234abc";
        let res = fixed("12345", input);
        assert!(res.is_none());
    }

    #[test]
    fn unsigned_number_matches() {
        let input = "1234abc";
        let res = unsigned_number(input);
        assert_eq!(res, Some((1234, "abc")));
    }

    #[test]
    fn unsigned_number_no_match() {
        let input = "abc1234";
        let res = unsigned_number(input);
        assert!(res.is_none());
    }

    #[test]
    fn match_n_matches() {
        let input = "1234abc";
        let res = match_n(|c| c.is_ascii_digit(), 2, input);
        assert_eq!(res, Some(("12", "34abc")));
    }

    #[test]
    fn match_n_no_match() {
        let input = "abc1234";
        let res = match_n(|c| c.is_ascii_digit(), 2, input);
        assert!(res.is_none());
    }

    #[test]
    fn match_n_beyond_range() {
        let input = "12";
        let res = match_n(|c| c.is_ascii_digit(), 3, input);
        assert!(res.is_none());
    }

    fn two_space(input: &str) -> Option<(&str, &str)> {
        if input.starts_with("  ") {
            Some((&input[..2], &input[2..]))
        } else {
            None
        }
    }

    #[test]
    fn optional_matches() {
        let input = "  abc";
        let res = optional(two_space, input);
        assert_eq!(res, (Some("  "), "abc"));
    }

    #[test]
    fn optional_no_match() {
        let input = " abc";
        let res = optional(two_space, input);
        assert_eq!(res, (None, " abc"));
    }

    #[test]
    fn match1_matches() {
        let input = "    abc";
        let res = many1(two_space, input);
        let expected = vec!["  ", "  "];
        assert_eq!(res, Some((expected, "abc")));
    }

    #[test]
    fn match1_no_match() {
        let input = "abc";
        let res = many1(two_space, input);
        assert!(res.is_none());
    }

    #[test]
    fn endline_match() {
        let input = "\nabc";
        let res = endline(input);
        assert_eq!(res, Some(("\n", "abc")));
    }
}
