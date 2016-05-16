use std::error::Error;
use std::fmt;
use std::io::Read;
use postgres::Result as PgResult;
use postgres::types::{FromSql, SessionInfo, Type};
use postgres::error::Error as PgError;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub struct InvalidOutcome(String);

impl fmt::Display for InvalidOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid outcome: {:?}", self.0)
    }
}

impl Error for InvalidOutcome {
    fn description(&self) -> &str {
        "Invalid outcome"
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Outcome {
    Incomplete,
    Loss,
    Win,
}

impl AsRef<str> for Outcome {
    fn as_ref(&self) -> &str {
        match *self {
            Outcome::Incomplete => "Incomplete",
            Outcome::Loss => "Loss",
            Outcome::Win => "Win",
        }
    }
}

impl FromSql for Outcome {
    fn from_sql<R: Read>(_: &Type, raw: &mut R, _: &SessionInfo) -> PgResult<Self> {
        let mut buf = vec![];
        raw.read_to_end(&mut buf)?;
        
        let value = String::from_utf8(buf).map_err(|err| PgError::Conversion(box err))?;
        match value.trim() {
            "Incomplete" => Ok(Outcome::Incomplete),
            "Loss" => Ok(Outcome::Loss),
            "Win" => Ok(Outcome::Win),
            _ => Err(PgError::Conversion(box InvalidOutcome(value))),
        }
    }
    
    fn accepts(ty: &Type) -> bool {
        super::accepts_strings(ty)
    }
}

impl Serialize for Outcome {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_str(self.as_ref())
    }
}