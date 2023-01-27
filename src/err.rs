// custom error type

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct PatchError {
    pub message: String,
}

impl PatchError {
    pub(crate) fn new(msg: String) -> PatchError {
        PatchError { message: msg }
    }
}

impl Display for PatchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Debug for PatchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PatchError: {}", self.message)
    }
}

impl Error for PatchError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Copy, Clone)]
pub enum TargetState {
    Present,
    Patched,
    Missing,
    Invalid,
}
