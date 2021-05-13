// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::BuildError;

pub fn get_folder_size(dir: &Path) -> Result<u64, BuildError> {
    let mut total_size = 0;
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            total_size += fs::metadata(path)?.len();
        }
    }

    Ok(total_size)
}

pub const fn default_true() -> bool {
    true
}

pub const fn default_false() -> bool {
    false
}
