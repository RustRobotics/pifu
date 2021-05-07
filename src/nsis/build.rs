// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs::{self, File};
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

    log::info!("create nsis folder");
    let workdir = Path::new(&conf.metadata.workdir);
    let nsis_dir = workdir.join("nsis");
    fs::create_dir_all(&nsis_dir)?;
    let nsi_file = nsis_dir.join("app.nsi");

    let mut nsi_fd = File::create(nsi_file)?;

    // Generate nsi script

    writeln!(nsi_fd, "Name {}", &conf.metadata.name)?;

    if nsis_conf.unicode {
        writeln!(nsi_fd, "Unicode True")?;
    } else {
        writeln!(nsi_fd, "Unicode False")?;
    }

    if nsis_conf.one_click {
        writeln!(
            nsi_fd,
            "InstallDir \"$LocalAppData\\Programs\\{}\"",
            &conf.metadata.name
        )?;
        writeln!(nsi_fd, "RequestExecutionlevel User")?;
    } else {
        if nsis_conf.per_machine {
            if arch == Arch::X86_64 {
                writeln!(
                    nsi_fd,
                    "InstallDir \"$PROGRAMFILES64\\{}\"",
                    &conf.metadata.name
                )?;
            } else {
                writeln!(nsi_fd, "InstallDir $PROGRAMFILES\\{}", &conf.metadata.name)?;
            }
            writeln!(nsi_fd, "RequestExecutionlevel Admin")?;
        } else {
            writeln!(nsi_fd, "RequestExecutionlevel User")?;
        }

        if nsis_conf.allow_to_change_installation_directory {
            writeln!(nsi_fd, "Page Directory")?;
        }
        writeln!(nsi_fd, "Page instfiles")?;
    }

    writeln!(nsi_fd, "Icon \"{}\"", nsis_conf.installer_icon)?;
    writeln!(nsi_fd, "UninstallIcon \"{}\"", nsis_conf.uninstaller_icon)?;
    // TODO(Shaohua): Set out file.

    writeln!(nsi_fd, "SetCompressor /SOLID {}", nsis_conf.compress_method)?;

    writeln!(nsi_fd, "Section Install")?;
    writeln!(nsi_fd, "  SetOutPath $INSTDIR")?;
    for file in files {
        writeln!(nsi_fd, "  File {}", &file.from)?;
    }
    writeln!(nsi_fd, "SectionEnd")?;

    Ok(())
}

fn compile_nsi_script() -> Result<(), BuildError> {
    Ok(())
}
