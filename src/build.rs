// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::app_image::build_app_image;
use crate::base::config::get_target_arch;
use crate::base::PlatformTarget;
use crate::config::Config;
use crate::deb::build_deb;
use crate::nsis::build_nsis;
use crate::rpm::build_rpm;
use crate::Error;

pub fn build(conf: Config, cross_build: bool) -> Result<(), Error> {
    // build packages one by one

    build_linux(&conf, cross_build)?;
    build_windows(&conf)
}

fn build_linux(conf: &Config, cross_build: bool) -> Result<(), Error> {
    log::info!("build_linux() conf: {:#?}", conf);

    // Skip if `linux` section is not set.
    let linux_conf = if let Some(linux_conf) = conf.linux.as_ref() {
        linux_conf
    } else {
        return Ok(());
    };

    let arches = if cross_build {
        linux_conf.arch.clone()
    } else {
        if let Some(target_arch) = get_target_arch() {
            vec![target_arch]
        } else {
            // No arch matches.
            return Ok(());
        }
    };

    if linux_conf.targets.contains(&PlatformTarget::Deb) {
        for arch in &arches {
            build_deb(conf, linux_conf, *arch)?;
        }
    }
    if linux_conf.targets.contains(&PlatformTarget::Rpm) {
        for arch in &arches {
            build_rpm(conf, linux_conf, *arch)?;
        }
    }
    if linux_conf.targets.contains(&PlatformTarget::AppImage) {
        for arch in &arches {
            build_app_image(conf, linux_conf, *arch)?;
        }
    }

    Ok(())
}

fn build_windows(conf: &Config) -> Result<(), Error> {
    let windows_conf = if let Some(windows_conf) = conf.windows.as_ref() {
        windows_conf
    } else {
        return Ok(());
    };

    if windows_conf.targets.contains(&PlatformTarget::Nsis) {
        for arch in &windows_conf.arch {
            build_nsis(conf, windows_conf, *arch)?;
        }
    }

    Ok(())
}
