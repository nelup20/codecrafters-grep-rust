use crate::regex::error::RegexParsingError;
use crate::regex::error::RegexParsingError::*;
use crate::regex::pattern::Pattern;
use crate::regex::pattern::Pattern::*;
use std::collections::VecDeque;
use std::iter::Peekable;
use std::str::Chars;

pub struct Regex {
    patterns: VecDeque<Pattern>,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Self, RegexParsingError> {
        if pattern.is_empty() {
            return Err(EmptyRegex);
        }

        Ok(Self {
            patterns: Self::parse_pattern(&mut pattern.chars().peekable())?,
        })
    }

    pub fn matches(&self, input: &str) -> bool {
        let start_indexes = self.find_start_indexes(input);

        for index in start_indexes {
            let mut matches_at_index = true;

            let input_chars = &mut input[index..].chars().enumerate().peekable();
            let mut patterns = self.patterns.iter().peekable();
            let mut backreference_values: Vec<String> = Vec::new();

            while let Some(pattern) = patterns.next() {
                let next_pattern: Option<&Pattern> = patterns.peek().map(|&val| val);

                if !pattern.matches(input_chars, next_pattern, &mut backreference_values) {
                    matches_at_index = false;
                    break;
                }
            }

            if matches_at_index {
                return true;
            }
        }

        false
    }

    fn parse_pattern(
        pattern: &mut Peekable<Chars>,
    ) -> Result<VecDeque<Pattern>, RegexParsingError> {
        let mut result = VecDeque::new();
        let mut pattern_starts_with = false;

        while let Some(current_char) = pattern.next() {
            match current_char {
                '^' => {
                    pattern_starts_with = true;
                    if pattern.peek().is_none() {
                        return Err(InvalidStart);
                    }
                }

                '$' => {
                    let previous_pattern = result.pop_back().ok_or(InvalidEnd)?;
                    result.push_back(EndOfString(Box::new(previous_pattern)));
                }

                '.' => {
                    result.push_back(Wildcard);
                }

                '\\' => match pattern.next().ok_or(InvalidCharClass)? {
                    'w' => result.push_back(AlphanumericClass),
                    'd' => result.push_back(DigitClass),
                    '\\' => result.push_back(CharLiteral('\\')),
                    number => result.push_back(Backreference(
                        number.to_digit(10).ok_or(InvalidBackreference)? as usize,
                    )),
                },

                '?' => {
                    let previous_pattern = result.pop_back().ok_or(InvalidOptionalQuantifier)?;
                    result.push_back(OptionalQuantifier(Box::new(previous_pattern)))
                }

                '+' => {
                    let previous_pattern = result.pop_back().ok_or(InvalidOneOrMoreQuantifier)?;
                    result.push_back(OneOrMoreQuantifier(Box::new(previous_pattern)))
                }

                '[' => {
                    let mut group_chars: Vec<char> = Vec::new();
                    let mut is_positive_group = true;

                    while let Some(next_char) = pattern.next() {
                        match next_char {
                            ']' => break,
                            '^' => is_positive_group = false,
                            _ => group_chars.push(next_char),
                        }
                    }

                    if is_positive_group {
                        result.push_back(PositiveCharGroup(group_chars))
                    } else {
                        result.push_back(NegativeCharGroup(group_chars))
                    }
                }

                '(' => {
                    result.push_back(Group(Self::parse_pattern(pattern)?));
                }

                ')' => return Ok(result),

                '|' => {
                    let mut alternation_candidates = Vec::new();
                    let mut current_candidate = Vec::new();

                    while let Some(char_literal) = result.pop_front() {
                        current_candidate.push(char_literal);
                    }

                    alternation_candidates.push(current_candidate);

                    current_candidate = Vec::new();
                    while let Some(next_pattern) = pattern.next() {
                        match next_pattern {
                            '|' => {
                                alternation_candidates.push(current_candidate);
                                current_candidate = Vec::new();
                            }

                            ')' => {
                                alternation_candidates.push(current_candidate);
                                result.push_back(Alternation(alternation_candidates));
                                return Ok(result);
                            }

                            next_char => current_candidate.push(CharLiteral(next_char)),
                        }
                    }
                }

                _ => {
                    result.push_back(CharLiteral(current_char));
                }
            }
        }

        if pattern_starts_with {
            let first_pattern = result.pop_front().unwrap();
            result.push_front(StartOfString(Box::new(first_pattern)))
        }

        Ok(result)
    }

    fn find_start_indexes(&self, input: &str) -> Vec<usize> {
        let mut result = Vec::new();

        let first_pattern = self.patterns.front().unwrap();

        match first_pattern {
            StartOfString(_) => {
                result.push(0);
            }
            _ => {
                // .char_indices() for non-ASCII bytes
                for (i, _) in input.char_indices() {
                    let input_sub_range = &mut input[i..].chars().enumerate().peekable();
                    if first_pattern.matches(input_sub_range, None, &mut vec![]) {
                        result.push(i);
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod parsing_tests {
    use crate::regex::pattern::Pattern;
    use crate::regex::pattern::Pattern::*;
    use crate::regex::regex::Regex;
    use std::collections::VecDeque;

    #[test]
    fn char_literal_singular() {
        let regex = Regex::new("a").unwrap();
        assert_eq!(1, regex.patterns.len());
        assert_eq!(CharLiteral('a'), *regex.patterns.front().unwrap())
    }

    #[test]
    fn char_literal_multiple() {
        let mut regex = Regex::new("ab3").unwrap();

        assert_eq!(3, regex.patterns.len());
        assert_eq!(CharLiteral('a'), regex.patterns.pop_front().unwrap());
        assert_eq!(CharLiteral('b'), regex.patterns.pop_front().unwrap());
        assert_eq!(CharLiteral('3'), regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn digit_class_singular() {
        let mut regex = Regex::new("\\d").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(DigitClass, regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn digit_class_multiple() {
        let mut regex = Regex::new("\\d\\d").unwrap();

        assert_eq!(2, regex.patterns.len());
        assert_eq!(DigitClass, regex.patterns.pop_front().unwrap());
        assert_eq!(DigitClass, regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn alphanumeric_class_singular() {
        let mut regex = Regex::new("\\w").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(AlphanumericClass, regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn alphanumeric_class_multiple() {
        let mut regex = Regex::new("\\w\\w").unwrap();

        assert_eq!(2, regex.patterns.len());
        assert_eq!(AlphanumericClass, regex.patterns.pop_front().unwrap());
        assert_eq!(AlphanumericClass, regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn singular_positive_char_group_singular_char() {
        let mut regex = Regex::new("[a]").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(
            PositiveCharGroup(vec!['a']),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn singular_positive_char_group_multiple_chars() {
        let mut regex = Regex::new("[abc]").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(
            PositiveCharGroup(vec!['a', 'b', 'c']),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn multiple_positive_char_groups_singular_char() {
        let mut regex = Regex::new("[a][b]").unwrap();

        assert_eq!(2, regex.patterns.len());
        assert_eq!(
            PositiveCharGroup(vec!['a']),
            regex.patterns.pop_front().unwrap()
        );
        assert_eq!(
            PositiveCharGroup(vec!['b']),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn multiple_positive_char_groups_multiple_chars() {
        let mut regex = Regex::new("[abc][def]").unwrap();

        assert_eq!(2, regex.patterns.len());
        assert_eq!(
            PositiveCharGroup(vec!['a', 'b', 'c']),
            regex.patterns.pop_front().unwrap()
        );
        assert_eq!(
            PositiveCharGroup(vec!['d', 'e', 'f']),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn singular_negative_char_group_singular_char() {
        let mut regex = Regex::new("[^a]").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(
            NegativeCharGroup(vec!['a']),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn singular_negative_char_group_multiple_chars() {
        let mut regex = Regex::new("[^abc]").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(
            NegativeCharGroup(vec!['a', 'b', 'c']),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn multiple_negative_char_groups_singular_char() {
        let mut regex = Regex::new("[^a][^b]").unwrap();

        assert_eq!(2, regex.patterns.len());
        assert_eq!(
            NegativeCharGroup(vec!['a']),
            regex.patterns.pop_front().unwrap()
        );
        assert_eq!(
            NegativeCharGroup(vec!['b']),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn multiple_negative_char_groups_multiple_chars() {
        let mut regex = Regex::new("[^abc][^def]").unwrap();

        assert_eq!(2, regex.patterns.len());
        assert_eq!(
            NegativeCharGroup(vec!['a', 'b', 'c']),
            regex.patterns.pop_front().unwrap()
        );
        assert_eq!(
            NegativeCharGroup(vec!['d', 'e', 'f']),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn positive_char_groups_and_negative_char_groups_1() {
        let mut regex = Regex::new("[abc][^def]").unwrap();

        assert_eq!(2, regex.patterns.len());
        assert_eq!(
            PositiveCharGroup(vec!['a', 'b', 'c']),
            regex.patterns.pop_front().unwrap()
        );
        assert_eq!(
            NegativeCharGroup(vec!['d', 'e', 'f']),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn positive_char_groups_and_negative_char_groups_2() {
        let mut regex = Regex::new("[^abc][def][^ghi][jkl]").unwrap();

        assert_eq!(4, regex.patterns.len());
        assert_eq!(
            NegativeCharGroup(vec!['a', 'b', 'c']),
            regex.patterns.pop_front().unwrap()
        );
        assert_eq!(
            PositiveCharGroup(vec!['d', 'e', 'f']),
            regex.patterns.pop_front().unwrap()
        );
        assert_eq!(
            NegativeCharGroup(vec!['g', 'h', 'i']),
            regex.patterns.pop_front().unwrap()
        );
        assert_eq!(
            PositiveCharGroup(vec!['j', 'k', 'l']),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn start_of_string_with_single_char() {
        let mut regex = Regex::new("^a").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(
            StartOfString(Box::new(CharLiteral('a'))),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn start_of_string_with_multiple_chars() {
        let mut regex = Regex::new("^ab").unwrap();

        assert_eq!(2, regex.patterns.len());
        assert_eq!(
            StartOfString(Box::new(CharLiteral('a'))),
            regex.patterns.pop_front().unwrap()
        );
        assert_eq!(CharLiteral('b'), regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn end_of_string_with_single_char() {
        let mut regex = Regex::new("a$").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(
            EndOfString(Box::new(CharLiteral('a'))),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn end_of_string_with_multiple_chars() {
        let mut regex = Regex::new("ab$").unwrap();

        assert_eq!(2, regex.patterns.len());
        assert_eq!(CharLiteral('a'), regex.patterns.pop_front().unwrap());
        assert_eq!(
            EndOfString(Box::new(CharLiteral('b'))),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn singular_one_or_more_quantifier_with_single_char() {
        let mut regex = Regex::new("a+").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(
            OneOrMoreQuantifier(Box::new(CharLiteral('a'))),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn multiple_one_or_more_quantifiers_with_single_char() {
        let mut regex = Regex::new("a+b+").unwrap();

        assert_eq!(2, regex.patterns.len());
        assert_eq!(
            OneOrMoreQuantifier(Box::new(CharLiteral('a'))),
            regex.patterns.pop_front().unwrap()
        );
        assert_eq!(
            OneOrMoreQuantifier(Box::new(CharLiteral('b'))),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn singular_one_or_more_quantifier_with_single_positive_char_group() {
        let mut regex = Regex::new("[abc]+").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(
            OneOrMoreQuantifier(Box::new(PositiveCharGroup(vec!['a', 'b', 'c']))),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn single_optional_quantifier_with_single_char() {
        let mut regex = Regex::new("a?").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(
            OptionalQuantifier(Box::new(CharLiteral('a'))),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn multiple_optional_quantifiers_with_single_char() {
        let mut regex = Regex::new("a?b?").unwrap();

        assert_eq!(2, regex.patterns.len());
        assert_eq!(
            OptionalQuantifier(Box::new(CharLiteral('a'))),
            regex.patterns.pop_front().unwrap()
        );
        assert_eq!(
            OptionalQuantifier(Box::new(CharLiteral('b'))),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn single_optional_quantifier_with_single_positive_char_group() {
        let mut regex = Regex::new("[abc]?").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(
            OptionalQuantifier(Box::new(PositiveCharGroup(vec!['a', 'b', 'c']))),
            regex.patterns.pop_front().unwrap()
        );
    }

    #[test]
    fn single_wildcard() {
        let mut regex = Regex::new(".").unwrap();

        assert_eq!(1, regex.patterns.len());
        assert_eq!(Wildcard, regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn multiple_wildcards() {
        let mut regex = Regex::new("...").unwrap();

        assert_eq!(3, regex.patterns.len());
        assert_eq!(Wildcard, regex.patterns.pop_front().unwrap());
        assert_eq!(Wildcard, regex.patterns.pop_front().unwrap());
        assert_eq!(Wildcard, regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn single_group_with_single_literal_char() {
        let mut regex = Regex::new("(a)").unwrap();

        let mut expected = VecDeque::new();
        expected.push_back(CharLiteral('a'));

        assert_eq!(1, regex.patterns.len());
        assert_eq!(Group(expected), regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn single_group_with_multiple_literal_chars() {
        let mut regex = Regex::new("(abc)").unwrap();

        let mut expected = VecDeque::new();
        expected.push_back(CharLiteral('a'));
        expected.push_back(CharLiteral('b'));
        expected.push_back(CharLiteral('c'));

        assert_eq!(1, regex.patterns.len());
        assert_eq!(Group(expected), regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn multiple_flat_groups_with_single_literal_char() {
        let mut regex = Regex::new("(a)(b)").unwrap();

        let mut expected_1 = VecDeque::new();
        expected_1.push_back(CharLiteral('a'));

        let mut expected_2 = VecDeque::new();
        expected_2.push_back(CharLiteral('b'));

        assert_eq!(2, regex.patterns.len());
        assert_eq!(Group(expected_1), regex.patterns.pop_front().unwrap());
        assert_eq!(Group(expected_2), regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn single_alternation_with_two_options() {
        let mut regex = Regex::new("(cat|dog)").unwrap();

        let mut expected = VecDeque::new();
        expected.push_back({
            let mut word_vec = Vec::new();
            word_vec.push(vec![CharLiteral('c'), CharLiteral('a'), CharLiteral('t')]);
            word_vec.push(vec![CharLiteral('d'), CharLiteral('o'), CharLiteral('g')]);

            Alternation(word_vec)
        });

        assert_eq!(1, regex.patterns.len());
        assert_eq!(Group(expected), regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn single_alternation_with_four_options() {
        let mut regex = Regex::new("(cat|dog|bird|lion)").unwrap();

        let mut expected = VecDeque::new();
        expected.push_back({
            let mut word_vec = Vec::new();
            word_vec.push(vec![CharLiteral('c'), CharLiteral('a'), CharLiteral('t')]);
            word_vec.push(vec![CharLiteral('d'), CharLiteral('o'), CharLiteral('g')]);
            word_vec.push(vec![
                CharLiteral('b'),
                CharLiteral('i'),
                CharLiteral('r'),
                CharLiteral('d'),
            ]);
            word_vec.push(vec![
                CharLiteral('l'),
                CharLiteral('i'),
                CharLiteral('o'),
                CharLiteral('n'),
            ]);

            Alternation(word_vec)
        });

        assert_eq!(1, regex.patterns.len());
        assert_eq!(Group(expected), regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn multiple_flat_groups_with_multiple_literal_chars() {
        let mut regex = Regex::new("(abc)(def)").unwrap();

        let mut expected_1 = VecDeque::new();
        expected_1.push_back(CharLiteral('a'));
        expected_1.push_back(CharLiteral('b'));
        expected_1.push_back(CharLiteral('c'));

        let mut expected_2 = VecDeque::new();
        expected_2.push_back(CharLiteral('d'));
        expected_2.push_back(CharLiteral('e'));
        expected_2.push_back(CharLiteral('f'));

        assert_eq!(2, regex.patterns.len());
        assert_eq!(Group(expected_1), regex.patterns.pop_front().unwrap());
        assert_eq!(Group(expected_2), regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn multiple_nested_groups_with_single_literal_char() {
        let mut regex = Regex::new("(a(b))").unwrap();

        let mut expected = VecDeque::new();
        expected.push_back(CharLiteral('a'));

        let mut inner_expected = VecDeque::new();
        inner_expected.push_back(CharLiteral('b'));
        expected.push_back(Group(inner_expected));

        assert_eq!(1, regex.patterns.len());
        assert_eq!(Group(expected), regex.patterns.pop_front().unwrap());
    }

    #[test]
    fn multiple_nested_and_flat_groups_with_multiple_literal_chars() {
        let regex = Regex::new("(ab (cd (ef)) gh) (ij)").unwrap();

        let mut expected: VecDeque<Pattern> = VecDeque::new();

        expected.push_back({
            let mut vec_a_to_h = VecDeque::new();
            vec_a_to_h.push_back(CharLiteral('a'));
            vec_a_to_h.push_back(CharLiteral('b'));
            vec_a_to_h.push_back(CharLiteral(' '));

            vec_a_to_h.push_back({
                let mut vec_c_to_f = VecDeque::new();
                vec_c_to_f.push_back(CharLiteral('c'));
                vec_c_to_f.push_back(CharLiteral('d'));
                vec_c_to_f.push_back(CharLiteral(' '));

                vec_c_to_f.push_back({
                    let mut vec_e_to_f = VecDeque::new();
                    vec_e_to_f.push_back(CharLiteral('e'));
                    vec_e_to_f.push_back(CharLiteral('f'));

                    Group(vec_e_to_f)
                });

                Group(vec_c_to_f)
            });

            vec_a_to_h.push_back(CharLiteral(' '));
            vec_a_to_h.push_back(CharLiteral('g'));
            vec_a_to_h.push_back(CharLiteral('h'));

            Group(vec_a_to_h)
        });

        expected.push_back(CharLiteral(' '));

        expected.push_back({
            let mut vec_i_to_j = VecDeque::new();
            vec_i_to_j.push_back(CharLiteral('i'));
            vec_i_to_j.push_back(CharLiteral('j'));
            Group(vec_i_to_j)
        });

        assert_eq!(3, regex.patterns.len());
        assert_eq!(expected, regex.patterns);
    }
}
