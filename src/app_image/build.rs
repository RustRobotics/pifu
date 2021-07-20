// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use regex::Regex;
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
    let app_image_conf = &linux_conf.app_image;

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
    let libs_dir = app_image_dir.join("libs");
    fs::create_dir_all(&app_image_dir)?;

    copy_filesets(files, &conf.metadata.src_dir, &app_image_dir)?;

    if app_image_conf.embed_libs {
        fs::create_dir_all(&libs_dir)?;
        copy_libraries(
            &app_image_conf.exe_files,
            &app_image_conf.exclude_libs,
            &libs_dir,
        )?;
    }

    compile_app_image(&workdir, &app_image_dir_name, arch)
}

fn copy_libraries(
    exe_files: &[String],
    exclude_libs: &[String],
    libs_dir: &Path,
) -> Result<(), BuildError> {
    let pattern = Regex::new(r"\s+(.+)\s+=>\s+(\S+)\s+\(\S+\)")?;
    for exe_file in exe_files {
        let output = Command::new("ldd").arg(exe_file).output()?;
        let stdout = String::from_utf8(output.stdout)?;
        for cap in pattern.captures_iter(&stdout) {
            // TODO(Shaohua): No need to create another string object.
            if !exclude_libs.contains(&cap[1].to_string()) {
                fs::copy(&cap[2], Path::join(libs_dir, &cap[1]))?;
            }
        }
    }
    Ok(())
}

fn compile_app_image<P: AsRef<Path>>(
    workdir: &Path,
    dir: &P,
    arch: Arch,
) -> Result<(), BuildError> {
    let status = Command::new("appimagetool")
        .env("ARCH", &arch.to_string())
        .current_dir(workdir)
        .arg(dir.as_ref())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(BuildError::AppImageCompilerError)
    }
}
