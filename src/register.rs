use thiserror::Error;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Register {
    A,
    D,
    M,
}

#[derive(Debug, Error)]
#[error("RegisterParseError")]
pub struct RegisterParseError;

impl TryFrom<char> for Register {
    type Error = RegisterParseError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Register::*;
        match c {
            'A' => Ok(A),
            'D' => Ok(D),
            'M' => Ok(M),
            _ => Err(RegisterParseError),
        }
    }
}
