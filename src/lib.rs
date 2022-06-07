// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![deny(warnings, clippy::all, clippy::cargo)]

mod app_image;
mod base;
mod build;
mod cmdline;
mod config;
mod deb;
mod download;
mod error;
mod nsis;
mod rpm;

pub use cmdline::read_cmdline;
pub use error::Error;
