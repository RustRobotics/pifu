// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use super::config::RpmConfig;
use crate::base::archive;
use crate::base::compress;
use crate::base::fileset::copy_filesets;
use crate::base::utils;
use crate::base::Arch;
use crate::config::{Config, LinuxConfig};
use crate::error::{Error, ErrorKind};

pub fn build_rpm(conf: &Config, linux_conf: &LinuxConfig, _arch: Arch) -> Result<(), Error> {
    let rpm_conf = &linux_conf.rpm;

    let workdir = Path::new(&conf.metadata.workdir);
    let rpm_dir = workdir.join("rpm");
    utils::rmdir(&rpm_dir)?;

    fs::create_dir_all(&rpm_dir)?;
    let spec_file = rpm_dir.join(format!("{}.spec", &conf.metadata.name));
    let mut spec_fd = File::create(&spec_file).map_err(|err| {
        Error::from_string(
            ErrorKind::IoError,
            format!(
                "Failed to create rpm spec file {:?}, error: {:?}",
                &spec_file, err
            ),
        )
    })?;
    let source_dir = rpm_dir.join(&format!(
        "{}-{}",
        &conf.metadata.name, &conf.metadata.version
    ));
    fs::create_dir_all(&source_dir)?;

    generate_spec_file(conf, rpm_conf, &mut spec_fd)?;

    // Copy files.
    let files = if let Some(files) = rpm_conf.files.as_ref() {
        files
    } else if let Some(files) = linux_conf.files.as_ref() {
        files
    } else {
        return Err(Error::new(
            ErrorKind::FilesNotSet,
            "`files` property not set for rpm format",
        ));
    };
    copy_filesets(files, &conf.metadata.src_dir, &source_dir)?;

    // Create binary tarbal.
    let source_tar_file = rpm_dir.join(format!("{}.tar", &conf.metadata.name));
    archive::create_tar(&source_dir, &source_tar_file)?;
    let source_xz_file = rpm_dir.join(format!("{}.tar.xz", &conf.metadata.name));
    compress::create_xz2(&source_tar_file, &source_xz_file)?;

    // Create rpmbuild folder.
    let rpm_source_dir = rpm_dir.join("SOURCES");
    fs::create_dir_all(&rpm_source_dir)?;
    let new_source_xz_file = rpm_source_dir.join(format!("{}.tar.xz", &conf.metadata.name));
    fs::rename(&source_xz_file, new_source_xz_file)?;

    generate_rpm_file(&spec_file, &rpm_dir)?;

    let rpm_dir = rpm_dir.to_str().unwrap();
    move_rpm_files(rpm_dir)
}

fn generate_spec_file(
    conf: &Config,
    rpm_conf: &RpmConfig,
    spec_fd: &mut File,
) -> Result<(), Error> {
    log::info!("generate_spec_file()");

    // Generate spec file.
    writeln!(spec_fd, "Name: {}", &conf.metadata.name)?;
    writeln!(spec_fd, "Version: {}", &conf.metadata.version)?;
    writeln!(spec_fd, "Release: 1%{{?dist}}")?;
    // TODO(Shaohua): Replace with short_description.
    writeln!(spec_fd, "Summary: {}", &conf.metadata.description)?;
    writeln!(spec_fd, "License: {}", &conf.metadata.license)?;
    writeln!(spec_fd, "URL: {}", &conf.metadata.homepage)?;
    writeln!(spec_fd, "Packager: {}", &conf.metadata.author)?;

    let source_tar_filename = format!("{}.tar.xz", &conf.metadata.name);
    writeln!(spec_fd, "Source0: {}", &source_tar_filename)?;

    if let Some(required_pkgs) = rpm_conf.required_pkgs.as_ref() {
        for pkg in required_pkgs {
            writeln!(spec_fd, "Required: {}", pkg)?;
        }
    }

    writeln!(spec_fd, "\n%description\n{}", &conf.metadata.description)?;
    writeln!(
        spec_fd,
        r#"%prep  
%setup -q           

%build 

%install
cp -rfa * %{{buildroot}}

%pre

%post

%preun

%postun

%clean

%files
/*
"#
    )?;

    Ok(())
}

fn generate_rpm_file(spec_file: &Path, rpm_dir: &Path) -> Result<(), Error> {
    log::info!(
        "generate_rpm_file() spec: {:?}, rpm_dir: {:?}",
        spec_file,
        rpm_dir
    );
    let def = format!("_topdir {}", fs::canonicalize(rpm_dir)?.display());

    let mut cmd = Command::new("rpmbuild");
    //if cfg!(not(debug_assertions)) {
    cmd.stdout(Stdio::null()).stderr(Stdio::null());
    //}
    // Change rootdir of rpm build.
    let status = cmd
        .arg("-D")
        .arg(&def)
        .arg("-bb")
        .arg(spec_file)
        .status()
        .map_err(|err| {
            Error::from_string(
                ErrorKind::RpmCompilerError,
                format!("Failed to run `rpmbuild` command, error: {:?}, please check `rpm` package is installed", err),
            )
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::from_string(
            ErrorKind::RpmCompilerError,
            format!("rpm file: {:?}", spec_file),
        ))
    }
}

fn move_rpm_files(rpm_dir: &str) -> Result<(), Error> {
    let rpm_files = format!("{}/RPMS/*/*.rpm", rpm_dir);
    utils::mv(&rpm_files, rpm_dir)
}
