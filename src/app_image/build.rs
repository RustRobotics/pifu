// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::base::fileset::copy_filesets;
use crate::base::utils;
use crate::base::Arch;
use crate::config::{get_binary_dir, Config, LinuxConfig};
use crate::error::{Error, ErrorKind};

pub fn build_app_image(conf: &Config, linux_conf: &LinuxConfig, arch: Arch) -> Result<(), Error> {
    let app_image_conf = &linux_conf.app_image;

    let files = if let Some(files) = app_image_conf.files.as_ref() {
        files
    } else if let Some(files) = linux_conf.files.as_ref() {
        files
    } else {
        return Err(Error::new(
            ErrorKind::FilesNotSet,
            "`files` property not set for app_image format",
        ));
    };

    let workdir = Path::new(&conf.metadata.workdir);
    let app_image_dir_name = "app_image";
    let app_image_dir = workdir.join(app_image_dir_name);
    utils::rmdir(&app_image_dir)?;

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

    compile_app_image(workdir, &app_image_dir_name, arch)
}

fn copy_libraries(
    exe_files: &[String],
    exclude_libs: &[String],
    libs_dir: &Path,
) -> Result<(), Error> {
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

fn get_appimage_tool(arch: Arch) -> Result<PathBuf, Error> {
    let mut binary_dir = get_binary_dir()?;
    binary_dir.push(format!("appimagetool-{}.AppImage", arch));
    Ok(binary_dir)
}

fn compile_app_image<P: AsRef<Path>>(workdir: &Path, dir: &P, arch: Arch) -> Result<(), Error> {
    let appimage_tool = get_appimage_tool(arch)?;
    log::info!("Using appimagetool: {:?}", &appimage_tool);
    let mut cmd = Command::new(appimage_tool.as_os_str());
    if cfg!(not(debug_assertions)) {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }
    let status = cmd
        .env("ARCH", &arch.to_string())
        .current_dir(workdir)
        .arg(dir.as_ref())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|err| {
            Error::from_string(
                ErrorKind::AppImageCompilerError,
                format!("Failed to run `appimagetool` command, error: {:?}, please install with `pifu --download` command", err),
            )
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::from_string(
            ErrorKind::AppImageCompilerError,
            format!("`appimagetool` returns error, workdir: {:?}", workdir),
        ))
    }
}
