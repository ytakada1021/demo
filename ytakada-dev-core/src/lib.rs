use std::fmt::{Display, Result, Formatter};
use std::error::Error;

pub mod post;

#[derive(Debug, Clone)]
pub struct AssertError;

impl Display for AssertError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "invalid first item to double")
    }
}

impl Error for AssertError {}
