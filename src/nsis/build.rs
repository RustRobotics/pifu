// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::base::Arch;
use crate::config::{Config, WindowsConfig};
use crate::BuildError;

pub fn build_nsis(
    conf: &Config,
    windows_conf: &WindowsConfig,
    arch: Arch,
) -> Result<(), BuildError> {
    let nsis_conf = if let Some(nsis_conf) = windows_conf.nsis.as_ref() {
        nsis_conf
    } else {
        // TODO(Shaohua): Returns error
        return Ok(());
    };

    let files = if let Some(files) = nsis_conf.files.as_ref() {
        files
    } else if let Some(files) = windows_conf.files.as_ref() {
        files
    } else {
        return Err(BuildError::FilesNotSet);
    };

    Ok(())
}
