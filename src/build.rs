// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use clap::{App, Arg};
use std::fs;
use std::path::Path;

use crate::app_image::build_app_image;
use crate::base::config::get_target_arch;
use crate::base::{expand_file_macro_simple, PlatformTarget};
use crate::config::Config;
use crate::deb::build_deb;
use crate::nsis::build_nsis;
use crate::rpm::build_rpm;
use crate::Error;

pub fn build() -> Result<(), Error> {
    // read config
    // build packages one by one

    let matches = App::new("Pifu package builder")
        .version("0.2.4")
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("General package builder")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("toml file")
                .help("Specify a custom toml config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cross-build")
                .short("x")
                .long("cross-build")
                .help("Enable corss build")
                .takes_value(false),
        )
        .get_matches();

    let mut config_file = matches.value_of("config").unwrap_or("pkg/pifu.toml");
    if !Path::new(config_file).exists() {
        config_file = "pifu.toml";
    }
    log::info!("config file: {:?}", config_file);

    let config_content = fs::read_to_string(config_file)
        .expect(&format!("Failed to read config at {}", config_file));
    let mut conf: Config = toml::from_str(&config_content).expect("Invalid config");

    conf.metadata.build_id = expand_file_macro_simple(&conf.metadata.build_id)?;

    let cross_build = matches.is_present("cross-build");

    build_linux(&conf, cross_build)?;
    build_windows(&conf)
}

fn build_linux(conf: &Config, cross_build: bool) -> Result<(), Error> {
    log::info!("build_linux() conf: {:?}", conf);

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
