// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::{Deserialize, Serialize};

use crate::base::config::Arch;
use crate::config::get_binary_dir;
use crate::Error;

#[derive(Debug, Deserialize, Serialize)]
struct DownloadEntry {
    arch: Arch,
    url: String,
    filename: String,
    sha256: String,
}

pub fn download() -> Result<(), Error> {
    // 0. get local binary directory.
    let binary_dir = get_binary_dir()?;
    log::info!("binary dir: {:?}", binary_dir);

    // 1. read and parse file list
    // 2. check file exists and file hash matches
    // 3. download file one by one with reqwest crate
    // 4. check downloaded file hash
    Ok(())
}

fn download_file() {}
