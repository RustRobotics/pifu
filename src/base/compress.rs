// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use flate2::{Compression, GzBuilder};
use std::fs::File;
use std::io;
use std::path::Path;
use xz2::write::XzEncoder;

use crate::BuildError;

pub fn create_gz(in_path: &Path, out_path: &Path) -> Result<(), BuildError> {
    let out_file = File::create(out_path)?;
    let mut gz = GzBuilder::new().write(out_file, Compression::default());

    let mut in_file = File::open(in_path)?;
    io::copy(&mut in_file, &mut gz)?;
    gz.finish()?;

    Ok(())
}

pub fn create_xz2(in_path: &Path, out_path: &Path) -> Result<(), BuildError> {
    let out_file = File::create(out_path)?;
    let xz_level = 9;
    let mut xz = XzEncoder::new(out_file, xz_level);

    let mut in_file = File::open(in_path)?;
    io::copy(&mut in_file, &mut xz)?;
    xz.finish()?;

    Ok(())
}
