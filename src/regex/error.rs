
#[derive(Debug)]
pub enum RegexError {
    EmptyRegex,
    InvalidCharClass,
    InvalidStart,
    InvalidEnd,
    InvalidCharGroup,
    InvalidOptionalQuantifier,
    InvalidOneOrMoreQuantifier,
}
