// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use walkdir::WalkDir;

use crate::base::Arch;
use crate::config::Config;
use crate::Error;

pub fn generate_control(
    conf: &Config,
    arch: Arch,
    size: u64,
    dest_file: &Path,
) -> Result<(), Error> {
    log::info!("generate_control() dest: {:?}", dest_file);
    let dest_dir = dest_file.parent().unwrap();
    fs::create_dir_all(dest_dir)?;
    let mut fd = File::create(dest_file)?;

    let metadata = &conf.metadata;
    writeln!(&mut fd, "Package: {}", metadata.name)?;
    writeln!(&mut fd, "Version: {}", metadata.version)?;
    writeln!(&mut fd, "Architecture: {}", arch_name(arch))?;

    let linux = conf.linux.as_ref().expect("Linux conf is not set");
    let deb = &linux.deb;
    if let Some(section) = deb.section.as_ref() {
        writeln!(&mut fd, "Section: {}", section)?;
    }
    writeln!(&mut fd, "Priority: {}", deb.priority)?;
    writeln!(&mut fd, "Standards-Version: 3.9.4")?;
    writeln!(&mut fd, "Maintainer: {}", metadata.author)?;
    writeln!(&mut fd, "Installed-Size: {}", size)?;

    if let Some(ref depends) = deb.depends {
        writeln!(&mut fd, "Depends: {}", depends)?;
    }
    if let Some(ref conflicts) = deb.conflicts {
        writeln!(&mut fd, "Conflicts: {}", conflicts)?;
    }
    if let Some(ref breaks) = deb.breaks {
        writeln!(&mut fd, "Breaks: {}", breaks)?;
    }
    if let Some(ref replaces) = deb.replaces {
        writeln!(&mut fd, "Replaces: {}", replaces)?;
    }
    if let Some(ref provides) = deb.provides {
        writeln!(&mut fd, "Provides: {}", provides)?;
    }

    writeln!(&mut fd, "Homepage: {}", metadata.homepage)?;
    writeln!(&mut fd, "Description: {}", metadata.description)?;

    Ok(())
}

fn md5_file(file: &Path) -> Result<String, Error> {
    log::info!("md5_file() file: {:?}", file);
    let mut in_file = File::open(file)?;
    let mut context = md5::Context::new();
    io::copy(&mut in_file, &mut context)?;
    let digest = context.compute();
    let hash = format!("{:x}", digest);

    Ok(hash)
}

pub fn generate_md5sum(dir: &Path, dest_file: &Path) -> Result<(), Error> {
    log::info!(
        "generate_md5sum() dir: {:?}, dest_file: {:?}",
        dir,
        dest_file
    );
    let dest_dir = dest_file.parent().unwrap();
    fs::create_dir_all(dest_dir)?;
    let mut dest_fd = File::create(dest_file)?;

    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let filename = path.strip_prefix(dir)?;
            let hash = md5_file(path)?;
            write!(dest_fd, "{} {}\n", hash, filename.display())?;
        }
    }

    Ok(())
}

pub fn generate_deb_binary(path: &Path) -> Result<(), Error> {
    let mut fd = File::create(path)?;
    writeln!(fd, "2.0")?;
    Ok(())
}

pub const fn arch_name(arch: Arch) -> &'static str {
    match arch {
        Arch::X86 => "i386",
        Arch::X86_64 => "amd64",
        Arch::AArch64 => "arm64",
    }
}
