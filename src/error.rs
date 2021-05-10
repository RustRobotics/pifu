// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io;
use std::path;
use std::string;
use std::time;

#[derive(Debug)]
pub enum BuildError {
    IoError(io::Error),

    FilesNotSet,

    Lz2EncodeError,

    WalkDirError(walkdir::Error),

    StripPrefixError(path::StripPrefixError),

    SystemTimeError,

    InvalidConfError,

    NsisCompilerError,

    /// Failed to get git commit hash.
    /// `git` command not found or this is not a git repo.
    GitHashError,

    EnvironmentNotSetError,

    Utf8Error,

    JsonError(serde_json::Error),

    RegexError(regex::Error),
}

impl From<regex::Error> for BuildError {
    fn from(err: regex::Error) -> Self {
        BuildError::RegexError(err)
    }
}

impl From<serde_json::Error> for BuildError {
    fn from(err: serde_json::Error) -> Self {
        BuildError::JsonError(err)
    }
}

impl From<time::SystemTimeError> for BuildError {
    fn from(_err: time::SystemTimeError) -> Self {
        BuildError::SystemTimeError
    }
}

impl From<xz2::stream::Error> for BuildError {
    fn from(_err: xz2::stream::Error) -> Self {
        BuildError::Lz2EncodeError
    }
}

impl From<walkdir::Error> for BuildError {
    fn from(err: walkdir::Error) -> Self {
        BuildError::WalkDirError(err)
    }
}

impl From<path::StripPrefixError> for BuildError {
    fn from(err: path::StripPrefixError) -> Self {
        BuildError::StripPrefixError(err)
    }
}

impl From<io::Error> for BuildError {
    fn from(err: io::Error) -> Self {
        BuildError::IoError(err)
    }
}

impl From<string::FromUtf8Error> for BuildError {
    fn from(_err: string::FromUtf8Error) -> Self {
        BuildError::Utf8Error
    }
}
