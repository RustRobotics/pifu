// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! # About
//! Pifu(aka 蚍蜉), is a cross platform package builder.
//!
//! ## Install
//! ```bash
//! cargo install pifu
//! ```
//!
//! ## Build dependencies
//! - exe: [nsis](https://nsis.sourceforge.io/)
//! - appimage: [appimagetool](https://github.com/AppImage/AppImageKit/releases), libc-bin (for `ldd`)
//! - rpm: rpm (for `rpmbuild` command)
//! - dmg: genisoimage (to generate dmg file), dmg2img (to test dmg file)
//!
//! ## Run dependencies
//! - appimage: glusterfs-client
//!
//! ## Related projects
//! - [nsis](https://nsis.sourceforge.io)
//! - [cargo bundle](https://github.com/burtonageo/cargo-bundle)
//! - [cargo deb](https://github.com/mmstick/cargo-deb)
//! - [electron builder](https://github.com/electron-userland/electron-builder)
//! - [fpm](https://github.com/jordansissel/fpm)
//! - [appimagekit](https://github.com/AppImage/appimagekit)
//! - [linuxdeploy](https://github.com/linuxdeploy/linuxdeploy)
//! - [linuxdeployqt](https://github.com/probonopd/linuxdeployqt)
//! - [appimage builder](https://github.com/AppImageCrafters/appimage-builder)
//! - [create dmg](https://github.com/create-dmg/create-dmg)
//! - [node appdmg](https://github.com/LinusU/node-appdmg)
//! - [dmgbuild](https://github.com/al45tair/dmgbuild)
//!
//! ## Misc
//! Linux desktop file can be validated by `desktop-file-validate` command,
//! which is included in `desktop-file-utils` package.

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
