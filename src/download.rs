// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::{Deserialize, Serialize};
use shell_rs::hashsum;
use std::fs::File;
use std::path::Path;

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
    let download_list_str = include_str!("download-list.toml");
    let entry_list: Vec<DownloadEntry> = toml::from_str(download_list_str)?;

    for entry in &entry_list {
        // 2. check file exists and file hash matches
        let filepath = Path::new(&binary_dir).join(&entry.filename);
        if filepath.exists() {
            if let Ok(file_hash) = hashsum::sha256sum(&filepath, &hashsum::Options::default()) {
                if file_hash == entry.sha256 {
                    log::info!("Skip exists file: {:?}", &filepath);
                    continue;
                } else {
                    log::error!(
                        "Hash mismatch, expected {:?}, got {:?}",
                        entry.sha256,
                        &file_hash
                    );
                }
            }
        }

        for _retry in 0..3 {
            // 3. download file one by one with reqwest crate
            if let Err(err) = download_file(&entry.url, &filepath) {
                log::error!("Failed to download {:?}, got error: {:?}", &entry.url, err);
                continue;
            }

            // 4. check downloaded file hash
            match hashsum::sha256sum(&filepath, &hashsum::Options::default()) {
                Ok(file_hash) => {
                    if file_hash == entry.sha256 {
                        break;
                    } else {
                        log::error!(
                            "Hash mismatch, expected {:?}, got {:?}",
                            entry.sha256,
                            &file_hash
                        );
                    }
                }
                Err(err) => log::error!("err: {:?}", err),
            }
        }
    }

    Ok(())
}

fn download_file<P: AsRef<Path>>(url: &str, filepath: P) -> Result<(), Error> {
    log::info!("Downloading {} to {:?}", url, filepath.as_ref());
    let mut response = reqwest::blocking::get(url)?;
    let mut fd = File::create(filepath)?;
    std::io::copy(&mut response, &mut fd)
        .map(drop)
        .map_err(|err| err.into())
}
