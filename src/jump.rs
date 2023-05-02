use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum Jump {
    NoJump,
    GreaterThan,
    Equal,
    GreaterThanOrEqual,
    LessThan,
    NotEqual,
    LessThanOrEqual,
    Always,
}

#[derive(Debug, Error)]
#[error("JumpParseError")]
pub struct JumpParseError;

impl TryFrom<Option<String>> for Jump {
    type Error = JumpParseError;
    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s.try_into(),
            None => Ok(Jump::NoJump),
        }
    }
}

impl TryFrom<String> for Jump {
    type Error = JumpParseError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        use Jump::*;
        match s.as_str() {
            "JGT" => Ok(GreaterThan),
            "JEQ"=> Ok(Equal),
            "JGE" => Ok(GreaterThanOrEqual),
            "JLT" => Ok(LessThan),
            "JNE" => Ok(NotEqual),
            "JLE" => Ok(LessThanOrEqual),
            "JMP" => Ok(Always),
            _ => Err(JumpParseError)
        }
    }
}
