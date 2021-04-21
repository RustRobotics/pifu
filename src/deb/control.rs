// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

use crate::base::Arch;
use crate::config::Config;
use crate::BuildError;

pub fn generate_control(conf: &Config, arch: Arch, size: usize) -> Result<(), BuildError> {
    let mut control: Vec<u8> = Vec::with_capacity(1024);

    let metadata = &conf.metadata;
    writeln!(&mut control, "Package: {}", metadata.name)?;
    writeln!(&mut control, "Version: {}", metadata.version)?;
    writeln!(&mut control, "Architecture: {}", arch)?;

    let linux = conf.linux.as_ref().unwrap();
    let deb = linux.deb.as_ref().unwrap();
    if let Some(section) = deb.section.as_ref() {
        writeln!(&mut control, "Section: {}", section)?;
    }
    writeln!(&mut control, "Priority: {}", deb.priority)?;
    writeln!(&mut control, "Standards-Version: 3.9.4")?;
    writeln!(&mut control, "Maintainer: {}", metadata.author)?;
    writeln!(&mut control, "Installed-Size: {}", size)?;

    if let Some(ref depends) = deb.depends {
        writeln!(&mut control, "Depends: {}", depends)?;
    }
    if let Some(ref conflicts) = deb.conflicts {
        writeln!(&mut control, "Conflicts: {}", conflicts)?;
    }
    if let Some(ref breaks) = deb.breaks {
        writeln!(&mut control, "Breaks: {}", breaks)?;
    }
    if let Some(ref replaces) = deb.replaces {
        writeln!(&mut control, "Replaces: {}", replaces)?;
    }
    if let Some(ref provides) = deb.provides {
        writeln!(&mut control, "Provides: {}", provides)?;
    }

    writeln!(&mut control, "Description: {}", metadata.description)?;

    Ok(())
}

fn md5_file(file: &Path) -> Result<String, BuildError> {
    let mut in_file = File::open(file)?;
    let mut context = md5::Context::new();
    io::copy(&mut in_file, &mut context)?;
    let digest = context.compute();
    let hash = format!("{:x}", digest);
    Ok(hash)
}

pub fn generate_md5sum(dir: &Path, dest_file: &Path) -> Result<(), BuildError> {
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
