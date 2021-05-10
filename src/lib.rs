// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub mod base;
mod build;
pub mod config;
mod deb;
mod error;
mod nsis;

pub use build::build;
pub use error::BuildError;
