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

pub fn unsigned_number(input: &str) -> Option<(u64, &str)> {
    let (num_str, input) = take_while(|c| c.is_ascii_digit(), input);
    let num = num_str.parse::<u64>().ok()?;
    Some((num, input))
}

pub fn signed_number(input: &str) -> Option<(i64, &str)> {
    let parser = |inp| {
        let (_, inp) = optional(
            |inp_| either(|i| fixed("+", i), |i| fixed("-", i), inp_),
            inp,
        );
        let (_, inp) = unsigned_number(inp)?;
        Some(((), inp))
    };
    let (num_str, input) = recognize(parser, input)?;
    let num = num_str.parse::<i64>().ok()?;
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

pub fn endline(input: &str) -> Option<(&str, &str)> {
    match_n(|c| c == '\n', 1, input)
}

pub fn words(number: usize, input: &str) -> Option<(&str, &str)> {
    assert!(number > 0);
    let mut rest = input;
    let mut pos_words_end = 0;
    for _ in 0..(number - 1) {
        let pos_word_end = rest.find(' ')?;
        if pos_word_end == input.len() - 1 {
            return None;
        }
        let start_next_word = pos_word_end + 1;
        rest = &rest[start_next_word..];
        pos_words_end += start_next_word;
    }
    let pos_word_end = rest.find(' ').unwrap_or_else(|| rest.len());
    pos_words_end += pos_word_end;
    Some((&input[..pos_words_end], &input[pos_words_end..]))
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

pub fn either<'a, O>(
    parser1: impl Parser<O, &'a str>,
    parser2: impl Parser<O, &'a str>,
    input: &'a str,
) -> Option<(O, &str)> {
    parser1.parse(input).or_else(|| parser2.parse(input))
}

pub fn recognize<'a, O>(parser: impl Parser<O, &'a str>, input: &'a str) -> Option<(&str, &str)> {
    let (_, rest) = parser.parse(input)?;
    // Feels a little weird, but stolen from Nom, so probably fine, maybe
    let input_ptr = input.as_ptr();
    let rest_ptr = rest.as_ptr();
    let offset = rest_ptr as usize - input_ptr as usize;
    Some((&input[..offset], rest))
}

pub fn eof(input: &str) -> Option<(&str, &str)> {
    if input.is_empty() {
        Some(("", input))
    } else {
        None
    }
}

pub fn endline_terminated<'a, O>(
    parser: impl Parser<O, &'a str>,
    input: &'a str,
) -> Option<(O, &str)> {
    let (res, input) = parser.parse(input)?;
    let (_, input) = either(endline, eof, input)?;
    Some((res, input))
}

#[macro_export]
macro_rules! make_parser {
    ($parser:ident, $($arg:expr),*) => {
        move |inp| $parser($($arg,)* inp)
    }
}

#[macro_export]
macro_rules! any {
    ($parser_1:expr, $parser_2:expr, $($parser_n:expr),*) => {
        {
            let parser = make_parser!(either, $parser_1, $parser_2);
            $(let parser = make_parser!(either, parser, $parser_n);)*
            parser
        }
    }
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
    fn signed_number_matches_positive() {
        let input = "+10";
        let res = signed_number(input);
        assert_eq!(res, Some((10, "")));
    }

    #[test]
    fn signed_number_matches_negative() {
        let input = "-9";
        let res = signed_number(input);
        assert_eq!(res, Some((-9, "")));
    }

    #[test]
    fn signed_number_matches_no_sign() {
        let input = "10";
        let res = signed_number(input);
        assert_eq!(res, Some((10, "")));
    }

    #[test]
    fn signed_number_no_match() {
        let input = "a10";
        let res = signed_number(input);
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

    #[test]
    fn words_one() {
        let input = "dark green sky";
        let res = words(1, input);
        assert_eq!(res, Some(("dark", " green sky")));
    }

    #[test]
    fn words_two() {
        let input = "dark green sky";
        let res = words(2, input);
        assert_eq!(res, Some(("dark green", " sky")));
    }

    #[test]
    fn words_incomplete() {
        let input = "dark";
        let res = words(2, input);
        assert!(res.is_none());
    }

    #[test]
    fn words_complete() {
        let input = "dark";
        let res = words(1, input);
        assert_eq!(res, Some(("dark", "")));
        let input = "dark green";
        let res = words(2, input);
        assert_eq!(res, Some(("dark green", "")));
    }

    #[test]
    fn recognize_match() {
        let parser = |inp| {
            let (_, inp) = fixed("#", inp)?;
            let (_, inp) = take_while1(|c| c.is_ascii_digit(), inp)?;
            Some(((), inp))
        };
        let input = "#1234abc";

        let res = recognize(parser, input);
        assert_eq!(res, Some(("#1234", "abc")));
    }

    #[test]
    fn either_one_matches() {
        let p1 = |inp| fixed("A", inp);
        let p2 = |inp| fixed("a", inp);
        let input = "abcd";
        let res = either(p1, p2, input);
        assert_eq!(res, Some(("a", "bcd")));
        let res = either(p2, p1, input);
        assert_eq!(res, Some(("a", "bcd")));

        let input = "Abcd";
        let res = either(p1, p2, input);
        assert_eq!(res, Some(("A", "bcd")));
    }

    #[test]
    fn either_no_match() {
        let p1 = |inp| fixed("A", inp);
        let p2 = |inp| fixed("a", inp);
        let input = "bcd";
        let res = either(p1, p2, input);
        assert!(res.is_none());
    }

    #[test]
    fn endline_terminated_endline() {
        let input = "abc\ndef";
        let res = endline_terminated(|inp| fixed("abc", inp), input);
        assert_eq!(res, Some(("abc", "def")));
    }

    #[test]
    fn endline_terminated_eof() {
        let input = "abc";
        let res = endline_terminated(|inp| fixed("abc", inp), input);
        assert_eq!(res, Some(("abc", "")));
    }

    #[test]
    fn make_parser_fixed() {
        let parser = make_parser!(fixed, "abc");
        let res = parser("abcd");
        assert_eq!(res, Some(("abc", "d")));
    }

    #[test]
    fn make_parser_match_n() {
        let parser = make_parser!(match_n, |c| c == 'a', 3);
        let res = parser("aaaab");
        assert_eq!(res, Some(("aaa", "ab")));
    }

    #[test]
    fn any_fixed() {
        let parser1 = make_parser!(fixed, "a");
        let parser2 = make_parser!(fixed, "b");
        let parser3 = make_parser!(fixed, "c");
        let any_parser = any!(parser1, parser2, parser3);
        assert_eq!(any_parser("a "), Some(("a", " ")));
        assert_eq!(any_parser("b "), Some(("b", " ")));
        assert_eq!(any_parser("c "), Some(("c", " ")));
        assert!(any_parser("d ").is_none());
    }
}
