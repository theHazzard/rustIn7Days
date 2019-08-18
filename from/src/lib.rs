mod parse;
use parse::*;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum GBPError {
    ParseError(ParseMoneyError),
    OtherError,
}

#[derive(Debug,PartialEq)]
pub struct GBP(i32);

impl FromStr for GBP {
    type Err=GBPError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GBP(parse_sym_money(s, '£', 2)?))
    }
}

impl From<ParseMoneyError> for GBPError {
    fn from(p: ParseMoneyError) -> Self {
        GBPError::ParseError(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let g = "£32.45".parse();
        assert_eq!(g, Ok(GBP(3245)));
    }
}
