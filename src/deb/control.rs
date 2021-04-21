// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::base::Arch;
use crate::config::Config;
use crate::BuildError;
use std::io::Write;

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

pub fn generate_md5sum() {}

pub fn generate_archive() {}
