// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs::File;
use std::io::Write;
use std::path::Path;

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

    let workdir = Path::new(&conf.metadata.workdir);
    let nsis_dir = workdir.join("nsis");
    let nsi_file = nsis_dir.join("app.nsi");

    let mut nsi_fd = File::create(nsi_file)?;

    // Generate nsi script

    write!(nsi_fd, "Name: {}", &conf.metadata.name)?;

    if nsis_conf.unicode {
        write!(nsi_fd, "Unicode True")?;
    } else {
        write!(nsi_fd, "Unicode False")?;
    }

    if nsis_conf.one_click {
        write!(
            nsi_fd,
            "InstallDir \"$LocalAppData\\Programs\\{}\"",
            &conf.metadata.name
        )?;
        write!(nsi_fd, "RequestExecutionlevel User")?;
    } else {
        if nsis_conf.per_machine {
            if arch == Arch::X86_64 {
                write!(
                    nsi_fd,
                    "InstallDir \"$PROGRAMFILES64\\{}\"",
                    &conf.metadata.name
                )?;
            } else {
                write!(nsi_fd, "InstallDir $PROGRAMFILES\\{}", &conf.metadata.name)?;
            }
            write!(nsi_fd, "RequestExecutionlevel Admin")?;
        } else {
            write!(nsi_fd, "RequestExecutionlevel User")?;
        }

        if nsis_conf.allow_to_change_installation_directory {
            write!(nsi_fd, "Page Directory")?;
        }
        write!(nsi_fd, "Page instfiles")?;
    }

    write!(nsi_fd, "Secion Install")?;
    write!(nsi_fd, "  SetOutPath $INSTDIR")?;
    for file in files {
        write!(nsi_fd, "  File {}", &file.from)?;
    }
    write!(nsi_fd, "Section End")?;

    Ok(())
}

fn compile_nsi_script() -> Result<(), BuildError> {
    Ok(())
}
