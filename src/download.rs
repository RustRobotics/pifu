// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::{Deserialize, Serialize};

use crate::base::config::Arch;
use crate::Error;

#[derive(Debug, Deserialize, Serialize)]
struct DownloadEntry {
    arch: Arch,
    url: String,
    filename: String,
    sha256: String,
}

pub fn download() -> Result<(), Error> {
    Ok(())
}

fn download_file() {}
