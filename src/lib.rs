// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
#![allow(clippy::module_name_repetitions, clippy::multiple_crate_versions)]

mod app_image;
pub mod base;
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
