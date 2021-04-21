// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs::File;
use std::path::Path;

use crate::BuildError;

pub fn create_archive(dir: &Path, to: &Path) -> Result<(), BuildError> {
    let to_file = File::create(to)?;
    let mut archive = tar::Builder::new(to_file);
    archive.append_dir_all(".", dir)?;
    archive.finish()?;

    Ok(())
}
