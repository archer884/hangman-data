use std::error::Error;
use std::fmt;
use std::io::Read;
use postgres::Result as PgResult;
use postgres::types::{FromSql, SessionInfo, Type};
use postgres::error::Error as PgError;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub struct InvalidDifficulty(String);

impl fmt::Display for InvalidDifficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid difficulty: {:?}", self.0)
    }
}

impl Error for InvalidDifficulty {
    fn description(&self) -> &str {
        "Invalid difficulty"
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl AsRef<str> for Difficulty {
    fn as_ref(&self) -> &str {
        match *self {
            Difficulty::Easy => "Easy",
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Hard",
        }
    }
}

impl FromSql for Difficulty {
    fn from_sql<R: Read>(_: &Type, raw: &mut R, _: &SessionInfo) -> PgResult<Self> {
        let mut buf = vec![];
        raw.read_to_end(&mut buf)?;
        
        let value = String::from_utf8(buf).map_err(|err| PgError::Conversion(box err))?;
        match value.trim() {
            "Easy" => Ok(Difficulty::Easy),
            "Normal" => Ok(Difficulty::Normal),
            "Hard" => Ok(Difficulty::Hard),
            _ => Err(PgError::Conversion(box InvalidDifficulty(value))),
        }
    }
    
    fn accepts(ty: &Type) -> bool {
        super::accepts_strings(ty)
    }
}

impl Serialize for Difficulty {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_str(self.as_ref())
    } 
}