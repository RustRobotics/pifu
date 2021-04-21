use std::io;

#[derive(Debug)]
pub enum BuildError {
    IoError(io::Error),

    FilesNotSet,
}

impl From<io::Error> for BuildError {
    fn from(err: io::Error) -> BuildError {
        BuildError::IoError(err)
    }
}
