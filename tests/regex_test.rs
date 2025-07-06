
#[cfg(test)]
mod matches_tests {
    use codecrafters_grep::Regex;

    #[test]
    fn single_char_literal_matches() {
        let regex = Regex::new("a").unwrap();
        assert!(regex.matches("a"))
    }

    #[test]
    fn single_char_literal_doesnt_match() {
        let regex = Regex::new("a").unwrap();
        assert!(!regex.matches("b"))
    }

    #[test]
    fn multiple_char_literals_match() {
        let regex = Regex::new("a").unwrap();
        assert!(regex.matches("bac"))
    }

    #[test]
    fn multiple_char_literals_dont_match() {
        let regex = Regex::new("a").unwrap();
        assert!(!regex.matches("bcd"))
    }

    #[test]
    fn single_digit_matches() {
        let regex = Regex::new("\\d").unwrap();
        assert!(regex.matches("2"))
    }

    #[test]
    fn single_digit_doesnt_match() {
        let regex = Regex::new("\\d").unwrap();
        assert!(!regex.matches("b"))
    }

    #[test]
    fn multiple_digits_match() {
        let regex = Regex::new("\\d\\d").unwrap();
        assert!(regex.matches("24"))
    }

    #[test]
    fn multiple_digits_dont_match() {
        let regex = Regex::new("\\d\\d").unwrap();
        assert!(!regex.matches("2c"))
    }

    #[test]
    fn single_alphanumeric_char_matches() {
        let regex = Regex::new("\\w").unwrap();
        assert!(regex.matches("a"))
    }

    #[test]
    fn single_alphanumeric_digit_matches() {
        let regex = Regex::new("\\w").unwrap();
        assert!(regex.matches("2"))
    }

    #[test]
    fn single_alphanumeric_doesnt_match() {
        let regex = Regex::new("\\w").unwrap();
        assert!(!regex.matches("@"))
    }

    #[test]
    fn multiple_alphanumeric_match() {
        let regex = Regex::new("\\w\\w").unwrap();
        assert!(regex.matches("2C"))
    }

    #[test]
    fn multiple_alphanumeric_dont_match() {
        let regex = Regex::new("\\w\\w").unwrap();
        assert!(!regex.matches("5@"))
    }

    #[test]
    fn single_positive_char_group_with_single_char_matches() {
        let regex = Regex::new("[a]").unwrap();
        assert!(regex.matches("a"))
    }

    #[test]
    fn single_positive_char_group_with_single_char_doesnt_match() {
        let regex = Regex::new("[a]").unwrap();
        assert!(!regex.matches("b"))
    }

    #[test]
    fn single_positive_char_group_with_multiple_chars_matches() {
        let regex = Regex::new("[def]").unwrap();
        assert!(regex.matches("e"))
    }

    #[test]
    fn single_positive_char_group_with_multiple_chars_doesnt_match() {
        let regex = Regex::new("[def]").unwrap();
        assert!(!regex.matches("z"))
    }

    #[test]
    fn single_negative_char_group_with_single_char_matches() {
        let regex = Regex::new("[^a]").unwrap();
        assert!(regex.matches("b"))
    }

    #[test]
    fn single_negative_char_group_with_single_char_doesnt_match() {
        let regex = Regex::new("[^a]").unwrap();
        assert!(!regex.matches("a"))
    }

    #[test]
    fn single_negative_char_group_with_multiple_chars_matches() {
        let regex = Regex::new("[^def]").unwrap();
        assert!(regex.matches("z"))
    }

    #[test]
    fn single_negative_char_group_with_multiple_chars_doesnt_match() {
        let regex = Regex::new("[^def]").unwrap();
        assert!(!regex.matches("e"))
    }

    #[test]
    fn multiple_positive_char_groups_with_multiple_chars_match() {
        let regex = Regex::new("[abc][def]").unwrap();
        assert!(regex.matches("be"))
    }

    #[test]
    fn multiple_positive_char_groups_with_multiple_chars_dont_match() {
        let regex = Regex::new("[abc][def]").unwrap();
        assert!(!regex.matches("bz"))
    }

    #[test]
    fn multiple_negative_char_groups_with_multiple_chars_match() {
        let regex = Regex::new("[^abc][^def]").unwrap();
        assert!(regex.matches("yz"))
    }

    #[test]
    fn multiple_negative_char_groups_with_multiple_chars_dont_match() {
        let regex = Regex::new("[^abc][^def]").unwrap();
        assert!(!regex.matches("ze"))
    }

    #[test]
    fn combined_literals_and_character_classes_match() {
        let regex = Regex::new("\\d \\w something").unwrap();
        assert!(regex.matches("1 a something"))
    }

    #[test]
    fn combined_literals_and_character_classes_dont_match() {
        let regex = Regex::new("\\d \\w something").unwrap();
        assert!(!regex.matches("1 a somEthing"))
    }

    #[test]
    fn start_of_string_matches() {
        let regex = Regex::new("^log").unwrap();
        assert!(regex.matches("logging "))
    }

    #[test]
    fn start_of_string_doesnt_match_1() {
        let regex = Regex::new("^log").unwrap();
        assert!(!regex.matches("lAgging "))
    }

    #[test]
    fn start_of_string_doesnt_match_2() {
        let regex = Regex::new("^log").unwrap();
        assert!(!regex.matches("slog"))
    }

    #[test]
    fn end_of_string_matches() {
        let regex = Regex::new("log$").unwrap();
        assert!(regex.matches("blog"))
    }

    #[test]
    fn end_of_string_doesnt_match() {
        let regex = Regex::new("log$").unwrap();
        assert!(!regex.matches("bl0g"))
    }

    #[test]
    fn one_or_more_matches_1() {
        let regex = Regex::new("a+").unwrap();
        assert!(regex.matches("aaaaaaaaaabb"))
    }

    #[test]
    fn one_or_more_matches_2() {
        let regex = Regex::new("ca+t").unwrap();
        assert!(regex.matches("cat"))
    }

    #[test]
    fn one_or_more_matches_3() {
        let regex = Regex::new("ca+at").unwrap();
        assert!(regex.matches("caaats"))
    }

    #[test]
    fn one_or_more_doesnt_match() {
        let regex = Regex::new("a+").unwrap();
        assert!(!regex.matches("bbbbbbb"))
    }

    #[test]
    fn optional_within_literal_string_with_present_char_matches() {
        let regex = Regex::new("foos?_and_bars").unwrap();
        assert!(regex.matches("foos_and_bars"))
    }

    #[test]
    fn optional_within_literal_string_with_missing_char_matches() {
        let regex = Regex::new("foos?_and_bars").unwrap();
        assert!(regex.matches("foo_and_bars"))
    }

    #[test]
    fn optional_within_literal_string_with_present_char_doesnt_match() {
        let regex = Regex::new("foos?_and_bars").unwrap();
        assert!(!regex.matches("foos_AND_bars"))
    }

    #[test]
    fn optional_within_literal_string_with_missing_char_doesnt_match() {
        let regex = Regex::new("foos?_and_bars").unwrap();
        assert!(!regex.matches("foo_AND_bars"))
    }

    #[test]
    fn single_wildcard_matches() {
        let regex = Regex::new("d.g").unwrap();
        assert!(regex.matches("dog"))
    }

    #[test]
    fn single_wildcard_doesnt_match() {
        let regex = Regex::new("d.g").unwrap();
        assert!(regex.matches("dAg"))
    }

    #[test]
    fn multiple_wildcards_match() {
        let regex = Regex::new("f.o b.r").unwrap();
        assert!(regex.matches("foo bar"))
    }

    #[test]
    fn multiple_wildcards_dont_match() {
        let regex = Regex::new("f.o b.r").unwrap();
        assert!(!regex.matches("fo  bar"))
    }

    #[test]
    fn single_alternation_with_two_options_matches() {
        let regex = Regex::new("(cat|dog)").unwrap();
        assert!(regex.matches("cat"))
    }

    #[test]
    fn single_alternation_with_two_options_doesnt_match() {
        let regex = Regex::new("(cat|dog)").unwrap();
        assert!(!regex.matches("bir"))
    }

    #[test]
    fn single_alternation_with_four_options_matches() {
        let regex = Regex::new("(cat|dog|bird|lion)").unwrap();
        assert!(regex.matches("bird"))
    }

    #[test]
    fn single_alternation_with_four_options_doesnt_match() {
        let regex = Regex::new("(cat|dog|bird|lion)").unwrap();
        assert!(!regex.matches("horse"))
    }

    #[test]
    fn single_alternation_with_four_options_and_literal_match_1() {
        let regex = Regex::new("(cat|dog|bird|lion) hello").unwrap();
        assert!(regex.matches("bird hello"))
    }

    #[test]
    fn single_alternation_with_four_options_and_literal_match_2() {
        let regex = Regex::new("hello (cat|dog|bird|lion) bye").unwrap();
        assert!(regex.matches("hello dog bye"))
    }

    #[test]
    fn alternation_stage_1() {
        let regex = Regex::new("^I see (\\d (cat|dog|cow)s?(, | and )?)+$").unwrap();
        assert!(regex.matches("I see 1 cat, 2 dogs and 3 cows"))
    }

    #[test]
    fn unicode_matches_1() {
        let regex = Regex::new("g.+gol").unwrap();
        assert!(regex.matches("goøö0Ogol"))
    }

    #[test]
    fn backreference_matches_1() {
        let regex = Regex::new("([abcd]+) is \\1, not [^xyz]+").unwrap();
        assert!(regex.matches("abcd is abcd, not efg"))
    }

    #[test]
    fn backreference_matches_2() {
        let regex = Regex::new("^(\\w+) starts and ends with \\1$").unwrap();
        assert!(regex.matches("this starts and ends with this"))
    }

    #[test]
    fn backreference_matches_3() {
        let regex = Regex::new("once a (drea+mer), alwaysz? a \\1").unwrap();
        assert!(regex.matches("once a dreaaamer, always a dreaaamer"))
    }

    #[test]
    fn multiple_backreferences_dont_match_1() {
        let regex = Regex::new("(\\d+) (\\w+) squares and \\1 \\2 circles").unwrap();
        assert!(!regex.matches("3 red squares and 4 red circles"))
    }

    #[test]
    fn nested_backreferences_match_1() {
        let regex = Regex::new("('(cat) and \\2') is the same as \\1").unwrap();
        assert!(regex.matches("'cat and cat' is the same as 'cat and cat'"))
    }

    #[test]
    fn nested_backreferences_match_2() {
        let regex = Regex::new("(([abc]+)-([def]+)) is \\1, not ([^xyz]+), \\2, or \\3").unwrap();
        assert!(regex.matches("abc-def is abc-def, not efg, abc, or def"))
    }
}
