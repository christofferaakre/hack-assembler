use std::fmt::Display;

use anyhow::anyhow;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Zero,
    One,
    MinusOne,
    D,
    A,
    NotD,
    NotA,
    MinusD,
    MinusA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
}

#[derive(Debug, Error)]
pub struct OperationParseError(String);

impl Display for OperationParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OperationParseError({})", self.0)
    }
}

impl TryFrom<String> for Operation {
    type Error = OperationParseError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        use Operation::*;
        match s.as_str() {
            "0" => Ok(Zero),
            "1" => Ok(One),
            "-1" => Ok(MinusOne),
            "D" => Ok(D),
            "A" => Ok(A),
            "!D" => Ok(NotD),
            "!A" => Ok(NotA),
            "-D" => Ok(MinusD),
            "-A" => Ok(MinusA),
            "D+1" => Ok(DPlusOne),
            "A+1" => Ok(APlusOne),
            "D-1" => Ok(DMinusOne),
            "A-1" => Ok(AMinusOne),
            "D+A" => Ok(DPlusA),
            "D-A" => Ok(DMinusA),
            "A-D" => Ok(DMinusA),
            "D&A" => Ok(DAndA),
            "D|A" => Ok(DOrA),
            other => Err(OperationParseError(other.to_owned()))
        }
    }
}
