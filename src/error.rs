use std::fmt::{Display, Formatter};

pub struct Error {
    pub(crate) message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.message)
    }
}
