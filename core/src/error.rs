use crate::error::MainError::Dyn;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum MainError {
    RONError(ron::Error),

    IOError(std::io::Error),
    Dyn(Box<dyn Error>),
    Custom(String),
}

impl From<Box<dyn Error>> for MainError {
    fn from(e: Box<dyn Error>) -> Self {
        Dyn(e)
    }
}

impl From<std::io::Error> for MainError {
    fn from(io: std::io::Error) -> Self {
        MainError::IOError(io)
    }
}

impl From<ron::Error> for MainError {
    fn from(de: ron::de::Error) -> Self {
        MainError::RONError(de)
    }
}

impl From<String> for MainError {
    fn from(str: String) -> Self {
        MainError::Custom(str)
    }
}

impl Display for MainError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            MainError::RONError(e) => Display::fmt(e, f),
            MainError::IOError(e) => Display::fmt(e, f),
            MainError::Dyn(e) => Display::fmt(e, f),
            MainError::Custom(e) => Display::fmt(e, f),
        }
    }
}
