// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use flate2::{Compression, GzBuilder};
use std::fs::File;
use std::io;
use std::path::Path;
use xz2::stream::MtStreamBuilder;
use xz2::write::XzEncoder;

use crate::error::{Error, ErrorKind};

#[allow(dead_code)]
pub fn create_gz(in_path: &Path, out_path: &Path) -> Result<(), Error> {
    log::info!("create_gz(), in: {:?}, out: {:?}", in_path, out_path);
    let out_file = File::create(out_path)?;
    let mut encoder = GzBuilder::new().write(out_file, Compression::default());
    let mut in_file = File::open(in_path)?;
    io::copy(&mut in_file, &mut encoder)?;
    encoder.finish()?;

    Ok(())
}

pub fn create_xz2(in_path: &Path, out_path: &Path) -> Result<(), Error> {
    log::info!("create_xz2(), in: {:?}, out: {:?}", in_path, out_path);
    let xz_level = 6;
    let stream = MtStreamBuilder::new()
        .preset(xz_level)
        .threads(num_cpus::get() as u32)
        .encoder()?;

    let out_file = File::create(out_path).map_err(|err| {
        Error::from_string(
            ErrorKind::IoError,
            format!(
                "Failed to create out file: {:?}, error: {:?}",
                out_path, err
            ),
        )
    })?;

    log::info!("Create xz encoder");
    let mut encoder = XzEncoder::new_stream(out_file, stream);
    let mut in_file = File::open(in_path).map_err(|err| {
        Error::from_string(
            ErrorKind::IoError,
            format!("Failed to open file {:?}, err: {:?}", in_path, err),
        )
    })?;
    io::copy(&mut in_file, &mut encoder).map_err(|err| {
        Error::from_string(
            ErrorKind::IoError,
            format!(
                "Failed to copy file from {:?} to {:?}, err: {:?}",
                in_file, out_path, err
            ),
        )
    })?;
    encoder.finish()?;

    Ok(())
}
