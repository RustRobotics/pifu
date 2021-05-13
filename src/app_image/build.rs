// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs::{self, File};
use std::path::Path;

use crate::base::fileset::copy_filesets;
use crate::base::Arch;
use crate::config::{Config, LinuxConfig};
use crate::BuildError;

pub fn build_app_image(
    conf: &Config,
    linux_conf: &LinuxConfig,
    arch: Arch,
) -> Result<(), BuildError> {
    let app_image_conf = if let Some(app_image_conf) = linux_conf.app_image.as_ref() {
        app_image_conf
    } else {
        return Err(BuildError::InvalidConfError);
    };

    let files = if let Some(files) = app_image_conf.files.as_ref() {
        files
    } else if let Some(files) = linux_conf.files.as_ref() {
        files
    } else {
        return Err(BuildError::FilesNotSet);
    };

    let workdir = Path::new(&conf.metadata.workdir);
    let app_image_dir = workdir.join("app_image");
    fs::create_dir_all(&app_image_dir)?;

    let src = Path::new(".");
    copy_filesets(files, src, &app_image_dir)?;

    Ok(())
}
