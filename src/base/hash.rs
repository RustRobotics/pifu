// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use sha2::Digest;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

/// # Errors
/// Returns error if failed to open file or read its content.
pub fn sha256sum<P: AsRef<Path>>(file: P) -> Result<String, io::Error> {
    let mut digest = sha2::Sha256::new();
    let mut reader = File::open(&file)?;

    let mut buffer = Vec::with_capacity(16 * 1024);
    loop {
        let n_read = reader.read_to_end(&mut buffer)?;
        if n_read == 0 {
            break;
        }
        digest.update(&buffer[..n_read]);
    }

    let result = digest.finalize();
    Ok(hex::encode(result))
}
