pub fn take_while(pred: impl Fn(char) -> bool, input: &str) -> (&str, &str) {
    let mut chars = input.chars();
    let mut i = 0;
    while let Some(c) = chars.next() {
        if !pred(c) {
            break;
        }
        i += 1;
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
    Some((input.get(0..length)?, &input[length..]))
}

pub fn fixed<'a>(s: &str, input: &'a str) -> Option<(&'a str, &'a str)> {
    let (part, input) = take(s.len(), input)?;
    if part == s {
        Some((part, input))
    } else {
        None
    }
}

pub fn unsigned_number(input: &str) -> Option<(u32, &str)> {
    let (num_str, input) = take_while(|c| c.is_ascii_digit(), input);
    let num = num_str.parse::<u32>().ok()?;
    Some((num, input))
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
}
