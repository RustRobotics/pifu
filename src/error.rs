// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io;
use std::path;
use std::string;
use std::time;

/// Prepresent the type of errors.
#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    IoError,

    FilesNotSet,

    Lz2EncodeError,

    WalkDirError,

    StripPrefixError,

    SystemTimeError,

    InvalidConfError,

    AppImageCompilerError,

    NsisCompilerError,

    RpmCompilerError,

    /// Failed to get git commit hash.
    /// `git` command not found or this is not a git repo.
    GitHashError,

    EnvironmentNotSetError,

    Utf8Error,

    JsonError,

    RegexError,

    InvalidDirname,

    GlobPatternError,
    GlobError,
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Error {
            kind,
            message: message.to_owned(),
        }
    }

    pub fn from_string(kind: ErrorKind, message: String) -> Self {
        Error { kind, message }
    }
}

impl From<glob::GlobError> for Error {
    fn from(err: glob::GlobError) -> Self {
        Error::from_string(ErrorKind::GlobError, format!("{}", err))
    }
}

impl From<glob::PatternError> for Error {
    fn from(err: glob::PatternError) -> Self {
        Error::from_string(ErrorKind::GlobPatternError, format!("{}", err))
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::from_string(ErrorKind::RegexError, format!("{}", err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::from_string(ErrorKind::JsonError, format!("{}", err))
    }
}

impl From<time::SystemTimeError> for Error {
    fn from(err: time::SystemTimeError) -> Self {
        Error::from_string(ErrorKind::SystemTimeError, format!("{}", err))
    }
}

impl From<xz2::stream::Error> for Error {
    fn from(err: xz2::stream::Error) -> Self {
        Error::from_string(ErrorKind::Lz2EncodeError, format!("{}", err))
    }
}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Self {
        Error::from_string(ErrorKind::WalkDirError, format!("{}", err))
    }
}

impl From<path::StripPrefixError> for Error {
    fn from(err: path::StripPrefixError) -> Self {
        Error::from_string(ErrorKind::StripPrefixError, format!("{}", err))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::from_string(ErrorKind::IoError, format!("{}", err))
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::from_string(ErrorKind::Utf8Error, format!("{}", err))
    }
}
