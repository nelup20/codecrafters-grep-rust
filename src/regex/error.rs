
#[derive(Debug)]
pub enum RegexParsingError {
    EmptyRegex,
    InvalidCharClass,
    InvalidStart,
    InvalidEnd,
    InvalidOptionalQuantifier,
    InvalidOneOrMoreQuantifier,
    InvalidBackreference
}
