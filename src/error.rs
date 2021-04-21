use std::io;

#[derive(Debug)]
pub enum BuildError {
    IoError(io::Error),

    FilesNotSet,

    Lz2EncodeError,
}

impl From<xz2::stream::Error> for BuildError {
    fn from(_err: xz2::stream::Error) -> Self {
        BuildError::Lz2EncodeError
    }
}

impl From<io::Error> for BuildError {
    fn from(err: io::Error) -> BuildError {
        BuildError::IoError(err)
    }
}
