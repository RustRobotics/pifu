// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::path::Path;

use crate::base::archive;
use crate::base::compress;
use crate::base::fileset;
use crate::base::utils;
use crate::base::Arch;
use crate::config::{Config, LinuxConfig};
use crate::deb::control;
use crate::BuildError;

pub fn build_deb(conf: &Config, linux_conf: &LinuxConfig, arch: Arch) -> Result<(), BuildError> {
    let deb_conf = if let Some(deb_conf) = linux_conf.deb.as_ref() {
        deb_conf
    } else {
        // TODO(Shaohua): Returns error
        return Ok(());
    };

    let files = if let Some(files) = deb_conf.files.as_ref() {
        files
    } else if let Some(files) = linux_conf.files.as_ref() {
        files
    } else {
        return Err(BuildError::FilesNotSet);
    };

    let workdir = Path::new(&conf.metadata.workdir);
    let deb_dir = workdir.join("deb");
    let data_dir = deb_dir.join("data");
    let src_dir = Path::new(&conf.metadata.src_dir);
    let control_dir = deb_dir.join("control");

    fileset::copy_filesets(files, &src_dir, &data_dir)?;

    let data_tar_file = deb_dir.join("data.tar");
    archive::create_tar_chown(&data_dir, &data_tar_file)?;

    let data_xz_file = deb_dir.join("data.tar.xz");
    compress::create_xz2(&data_tar_file, &data_xz_file)?;

    let md5sum_file = control_dir.join("md5sum");
    control::generate_md5sum(&data_dir, &md5sum_file)?;

    let file_size = utils::get_folder_size(&data_dir)?;
    let control_file = control_dir.join("control");
    control::generate_control(conf, arch, file_size, &control_file)?;

    let control_tar_file = deb_dir.join("control.tar");
    archive::create_tar_without_rootdir(&control_dir, &control_tar_file)?;

    let control_xz_file = deb_dir.join("control.tar.xz");
    compress::create_xz2(&control_tar_file, &control_xz_file)?;

    let deb_binary_file = deb_dir.join("debian-binary");
    control::generate_deb_binary(&deb_binary_file)?;

    let deb_filename = format!(
        "{}_{}_{}.deb",
        conf.metadata.name,
        conf.metadata.version,
        control::arch_name(arch)
    );
    let deb_file = workdir.join(deb_filename);
    let xz_files = vec![&deb_binary_file, &control_xz_file, &data_xz_file];
    archive::create_ar_files(&xz_files, &deb_file)?;

    Ok(())
}
