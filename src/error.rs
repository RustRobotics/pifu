use std::io;

#[derive(Debug)]
pub enum BuilderError {
    IoError(io::Error),
}

impl From<io::Error> for BuilderError {
    fn from(err: io::Error) -> BuilderError {
        BuilderError::IoError(err)
    }
}
