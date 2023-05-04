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
    M,
    NotM,
    MinusM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
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
            "N" => Ok(M),
            "!M" => Ok(NotM),
            "-M" => Ok(MinusM),
            "M+1" => Ok(MPlusOne),
            "M-1" => Ok(MMinusOne),
            "D+M" => Ok(DPlusM),
            "D-M" => Ok(DMinusM),
            "M-D" => Ok(MMinusD),
            "D&M" => Ok(DAndM),
            "D|M" => Ok(DOrM),
            other => Err(OperationParseError(other.to_owned())),
        }
    }
}

impl Operation {
    // TODO: this could be a lot better, maybe a macro
    pub fn codegen(&self) -> Vec<bool> {
        use Operation::*;
        let s = match self {
            Zero => "0101010",
            One => "0111111",
            MinusOne => "0111010",
            D => "0001100",
            A => "0110000",
            NotD => "0001101",
            NotA => "0110001",
            MinusD => "0001111",
            MinusA => "0110011",
            DPlusOne => "0011111",
            APlusOne => "0110111",
            DMinusOne => "0001110",
            AMinusOne => "0110010",
            DPlusA => "0001110",
            DMinusA => "0010011",
            AMinusD => "0000111",
            DAndA => "0000000",
            DOrA => "0010101",
            M => "1110000",
            NotM => "1110001",
            MinusM => "1110011",
            MPlusOne => "1110111",
            MMinusOne => "1110010",
            DPlusM => "1000010",
            DMinusM => "1010011",
            MMinusD => "1000111",
            DAndM => "1000000",
            DOrM => "1010101",
        };

        let bits: Vec<bool> = s.chars().map(|c| match c {
            '0' => false,
            '1' => true,
            _ => unreachable!("There should only be 0s and 1s")
        }).collect(); 

        assert_eq!(bits.len(), 7);
        bits
            
    }
}
