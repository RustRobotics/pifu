// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use clap::{App, Arg};
use std::fs;

use crate::base::{expand_file_macro_simple, PlatformTarget};
use crate::config::Config;
use crate::deb::build_deb;
use crate::nsis::build_nsis;
use crate::BuildError;

pub fn build() -> Result<(), BuildError> {
    // read config
    // build packages one by one

    let matches = App::new("Rust builder")
        .version("0.1.0")
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
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or("pifu.toml");
    log::info!("config file: {:?}", config_file);

    let config_content =
        fs::read_to_string(config_file).expect(&format!("Failed to read {}", config_file));
    let mut conf: Config = toml::from_str(&config_content).expect("Invalid config");

    conf.metadata.build_id = expand_file_macro_simple(&conf.metadata.build_id)?;
    build_linux(&conf)?;
    build_windows(&conf)
}

fn build_linux(conf: &Config) -> Result<(), BuildError> {
    let linux_conf = if let Some(linux_conf) = conf.linux.as_ref() {
        linux_conf
    } else {
        return Ok(());
    };

    if linux_conf.targets.contains(&PlatformTarget::Deb) {
        for arch in &linux_conf.arch {
            build_deb(conf, linux_conf, *arch)?;
        }
    }

    Ok(())
}

fn build_windows(conf: &Config) -> Result<(), BuildError> {
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
