const ASCII_0: u8 = 48;
const ASCII_9: u8 = 57;
const ASCII_UPPERCASE_A: u8 = 65;
const ASCII_UPPERCASE_Z: u8 = 90;
const ASCII_LOWERCASE_A: u8 = 97;
const ASCII_LOWERCASE_Z: u8 = 122;

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        "\\d" => input_line
            .chars()
            .find(|&char| char_is_digit(char))
            .is_some(),

        "\\w" => input_line
            .chars()
            .find(|&char| char_is_digit(char) || char_is_letter(char))
            .is_some(),

        character_group if character_group.starts_with("[") => {
            if character_group.contains('^') {
                let negative_char_group = &character_group[2..character_group.len()];

                input_line
                    .chars()
                    .any(|char| !negative_char_group.contains(char))
            } else {
                let positive_char_group = &character_group[1..character_group.len()];

                positive_char_group
                    .chars()
                    .any(|char| input_line.contains(char))
            }
        }

        char => input_line.contains(char),
    }
}

#[inline(always)]
fn char_is_letter(char: char) -> bool {
    char_is_uppercase_letter(char) || char_is_lowercase_letter(char)
}

#[inline(always)]
fn char_is_uppercase_letter(char: char) -> bool {
    char as u8 >= ASCII_UPPERCASE_A && char as u8 <= ASCII_UPPERCASE_Z
}

#[inline(always)]
fn char_is_lowercase_letter(char: char) -> bool {
    char as u8 >= ASCII_LOWERCASE_A && char as u8 <= ASCII_LOWERCASE_Z
}

#[inline(always)]
fn char_is_digit(char: char) -> bool {
    char as u8 >= ASCII_0 && char as u8 <= ASCII_9
}
