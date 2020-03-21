use std::fmt::Display;

#[derive(Hash)]
pub struct Auto;
impl Display for Auto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "auto")
    }
}

pub struct Inherit;
impl Display for Inherit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "inherit")
    }
}

pub struct Unset;
impl Display for Unset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unset")
    }
}

pub struct Initial;
impl Display for Initial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "initial")
    }
}

pub struct MaxContent;
impl Display for MaxContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "max-content")
    }
}

pub struct MinContent;
impl Display for MinContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "min-content")
    }
}
