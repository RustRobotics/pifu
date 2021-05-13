// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs;
use std::path::Path;
use std::process::Command;

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
    let app_image_dir_name = "app_image";
    let app_image_dir = workdir.join(app_image_dir_name);
    fs::create_dir_all(&app_image_dir)?;

    let src = Path::new(".");
    copy_filesets(files, src, &app_image_dir)?;

    compile_app_image(&workdir, &app_image_dir_name)
}

fn compile_app_image<P: AsRef<Path>>(workdir: &Path, dir: &P) -> Result<(), BuildError> {
    let status = Command::new("appimagetool")
        .current_dir(workdir)
        .arg(dir.as_ref())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(BuildError::AppImageCompilerError)
    }
}
