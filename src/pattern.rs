use std::str::Chars;

const ASCII_0: u8 = 48;
const ASCII_9: u8 = 57;
const ASCII_UPPERCASE_A: u8 = 65;
const ASCII_UPPERCASE_Z: u8 = 90;
const ASCII_LOWERCASE_A: u8 = 97;
const ASCII_LOWERCASE_Z: u8 = 122;

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let pattern_indexes = get_pattern_start_indexes(input_line, pattern);
    
    for index in pattern_indexes {
        if pattern_matches_for_substring(&input_line[index..], pattern) {
            return true;
        }
    }
    
    false
}

// TODO: refactor this monstrosity
fn pattern_matches_for_substring(sub_input: &str, pattern: &str) -> bool {
    let mut input_chars = sub_input.chars();
    let mut pattern_chars = pattern.chars();

    while let Some(pattern_char) = pattern_chars.next() {
        match pattern_char {
            '\\' => match pattern_chars.next() {
                None => {}
                Some(char) => match char {
                    'd' => {
                        match input_chars.next() {
                            None => return false,
                            Some(input_char) => {
                                if !is_char_digit(input_char) {
                                    return false;
                                }
                            }
                        }
                    }

                    'w' => {
                        match input_chars.next() {
                            None => return false,
                            Some(input_char) => {
                                if !is_char_alphanumeric(input_char) {
                                    return false;
                                }
                            }
                        }
                    }
                    _ => {}
                },
            },

            '[' => match pattern_chars.next() {
                None => return false,
                Some(char) => match char {
                    '^' => {
                        let mut group_chars: Vec<char> = Vec::new();
                        
                        while let Some(group_char) = pattern_chars.next() {
                            match group_char { 
                                ']' => break,
                                _ => group_chars.push(group_char)
                            }
                        }
                        
                        match input_chars.next() {
                            None => return false,
                            Some(input_char) => {
                                if group_chars.contains(&input_char) {
                                    return false;
                                }
                            }
                        }
                    },

                    group_char => {
                        let mut group_chars: Vec<char> = Vec::new();
                        group_chars.push(group_char);

                        while let Some(group_char) = pattern_chars.next() {
                            match group_char {
                                ']' => break,
                                _ => group_chars.push(group_char)
                            }
                        }

                        match input_chars.next() {
                            None => return false,
                            Some(input_char) => {
                                if !group_chars.contains(&input_char) {
                                    return false;
                                }
                            }
                        }
                    }
                },
            },

            literal_char => {
                match input_chars.next() {
                    None => return false,
                    Some(input_char) => {
                        if input_char != literal_char {
                            return false;
                        }
                    }
                }
            }
        }
    }
    
    true
}

fn get_pattern_start_indexes(input: &str, pattern: &str) -> Vec<usize> {
    let input_chars = input.chars();
    let mut pattern_chars = pattern.chars();
    let mut result: Vec<usize> = Vec::new();

    match pattern_chars.next() {
        None => {}
        Some(pattern_char) => match pattern_char {
            '\\' => match pattern_chars.next() {
                None => {}
                Some(char) => match char {
                    'd' => {
                        result = get_indexes_by_filter(input_chars, |input_char| {
                            is_char_digit(input_char)
                        });
                    }

                    'w' => {
                        result = get_indexes_by_filter(input_chars, |input_char| {
                            is_char_alphanumeric(input_char)
                        })
                    }
                    _ => {}
                },
            },

            '[' => match pattern_chars.next() {
                None => {}
                Some(char) => match char {
                    '^' => match pattern_chars.next() {
                        None => {}
                        // TODO: need to check entire group
                        Some(group_char) => {
                            result = get_indexes_by_filter(input_chars, |input_char| {
                                input_char != group_char
                            })
                        }
                    },

                    group_char => {
                        result = get_indexes_by_filter(input_chars, |input_char| {
                            input_char == group_char
                        })
                    }
                },
            },

            literal_char => {
                result = get_indexes_by_filter(input_chars, |input_char| {
                    input_char == literal_char
                })
            }
        },
    }

    result
}

fn get_indexes_by_filter<F: FnMut(char) -> bool>(input_chars: Chars, mut filter: F) -> Vec<usize> {
    input_chars
        .enumerate()
        .filter(|&(_, char)| filter(char))
        .map(|(i, _)| i)
        .collect()
}

#[inline(always)]
fn is_char_alphanumeric(char: char) -> bool {
    is_char_digit(char) || is_char_letter(char)
}

#[inline(always)]
fn is_char_letter(char: char) -> bool {
    is_char_uppercase_letter(char) || is_char_lowercase_letter(char)
}

#[inline(always)]
fn is_char_uppercase_letter(char: char) -> bool {
    char as u8 >= ASCII_UPPERCASE_A && char as u8 <= ASCII_UPPERCASE_Z
}

#[inline(always)]
fn is_char_lowercase_letter(char: char) -> bool {
    char as u8 >= ASCII_LOWERCASE_A && char as u8 <= ASCII_LOWERCASE_Z
}

#[inline(always)]
fn is_char_digit(char: char) -> bool {
    char as u8 >= ASCII_0 && char as u8 <= ASCII_9
}

#[cfg(test)]
mod tests {
    use crate::pattern::match_pattern;

    #[test]
    fn single_digit_matches_1() {
        assert!(match_pattern("1 abc", "\\d"))
    }

    #[test]
    fn single_digit_matches_2() {
        assert!(match_pattern("a1bc", "\\d"))
    }

    #[test]
    fn single_digit_matches_3() {
        assert!(match_pattern("abc 1", "\\d"))
    }
}
