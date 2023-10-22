use std::env;

fn match_here(regexp: &[char], text: &[char]) -> bool {
    if regexp.is_empty() {
        return true;
    }

    if regexp[0] == '$' && regexp.len() == 1 {
        println!("text: {:?}", text);
        return text.is_empty();
    }

    if text.is_empty() {
        return false;
    }

    let first_match = regexp[0] == '.' || regexp[0] == text[0];

    if regexp.len() >= 2 && regexp[1] == '*' {
        return match_star(regexp[0], &regexp[2..], text)
            || (first_match && match_here(regexp, &text[1..]));
    }

    first_match && match_here(&regexp[1..], &text[1..])
}

fn match_star(c: char, regexp: &[char], text: &[char]) -> bool {
    let mut i = 0;
    while match_here(&[c], &text[i..]) {
        i += 1;
    }
    match_here(regexp, &text[i..])
}

fn match_expression(regexp: &[char], text: &[char]) -> bool {
    if !regexp.is_empty() && regexp[0] == '^' {
        return match_here(&regexp[1..], text);
    }

    for i in 0..=text.len() {
        if match_here(regexp, &text[i..]) {
            return true;
        }
    }

    false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <regex> <text>", args[0]);
        std::process::exit(1);
    }

    let regexp = &args[1];
    let text = &args[2];

    let regexp_chars: Vec<char> = regexp.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();

    let result = match_expression(&regexp_chars, &text_chars);

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_regular_character() {
        assert!(match_expression(&['a'], &['a']));
    }

    #[test]
    fn test_non_matching_regular_character() {
        assert!(!match_expression(&['a'], &['b']));
    }

    #[test]
    fn test_matching_start_anchor() {
        assert!(match_expression(&['^', 'a', 'b', 'c'], &['a', 'b', 'c']));
    }

    #[test]
    fn test_non_matching_start_anchor() {
        assert!(!match_expression(
            &['^', 'a', 'b', 'c'],
            &['x', 'a', 'b', 'c']
        ));
    }

    #[test]
    fn test_matching_end_anchor() {
        assert!(match_expression(&['a', 'b', 'c', '$'], &['a', 'b', 'c']));
    }

    #[test]
    fn test_non_matching_end_anchor() {
        assert!(!match_expression(
            &['a', 'b', 'c', '$'],
            &['a', 'b', 'c', 'x']
        ));
    }

    #[test]
    fn test_matching_zero_or_more_instances() {
        assert!(match_expression(
            &['a', '*', 'b', '*', 'c'],
            &['b', 'b', 'c']
        ));
    }

    #[test]
    fn test_non_matching_zero_or_more_instances() {
        assert!(!match_expression(
            &['a', '*', 'b', '*', 'c'],
            &['a', 'b', 'b']
        ));
    }

    #[test]
    fn test_matching_wildcard_character() {
        assert!(match_expression(&['a', '.', 'c'], &['a', 'b', 'c']));
    }

    #[test]
    fn test_non_matching_wildcard_character() {
        assert!(!match_expression(&['a', '.', 'c'], &['a', 'c']));
    }

    #[test]
    fn test_empty_regex_matches_anything() {
        assert!(match_expression(&[], &['a', 'b', 'c']));
    }

    #[test]
    fn test_empty_text_matches_only_empty_regex() {
        assert!(match_expression(&[], &[]));
        assert!(!match_expression(&['a'], &[]));
    }
}
