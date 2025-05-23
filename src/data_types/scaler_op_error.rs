use std::fmt::Display;


#[derive(Debug)]
pub enum ScalarOpErr {
    // Bad(Token,String),
    ExpectNumber,
    ExpectNumbers,
    ExpectStrings,
    ExpectStringsOrNumbers,
}

impl Display for ScalarOpErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
