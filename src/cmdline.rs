// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use clap::{Arg, Command};
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::base::{expand_file_macro_simple, Arch, PlatformTarget};
use crate::build;
use crate::config::Config;
use crate::download;
use crate::error::{Error, ErrorKind};

const OPT_CONFIG: &str = "config";
const OPT_OS: &str = "os";
const OPT_TARGET: &str = "target";
const OPT_ARCH: &str = "arch";
const OPT_DOWNLOAD: &str = "download";
const OPT_IGNORE_ERROR: &str = "ignore-error";

pub fn read_cmdline() -> Result<(), Error> {
    let matches = Command::new("Pifu - Cross platform package builder")
        .version("0.3.3")
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("General package builder")
        .arg(
            Arg::new(OPT_CONFIG)
                .short('c')
                .long(OPT_CONFIG)
                .value_name("toml file")
                .help("Specify a custom toml config file")
                .takes_value(true),
        )
        .arg(
            Arg::new(OPT_OS)
                .long(OPT_OS)
                .multiple_occurrences(true)
                .help("Build for specific OS platform")
                .takes_value(true),
        )
        .arg(
            Arg::new(OPT_TARGET)
                .long(OPT_TARGET)
                .short('t')
                .multiple_occurrences(true)
                .help("Build specific target")
                .takes_value(true),
        )
        .arg(
            Arg::new(OPT_ARCH)
                .long(OPT_ARCH)
                .short('a')
                .multiple_occurrences(true)
                .help("Build specific architecture")
                .takes_value(true),
        )
        .arg(
            Arg::new(OPT_DOWNLOAD)
                .long(OPT_DOWNLOAD)
                .help("Download required tools from github")
                .takes_value(false),
        )
        .arg(
            Arg::new(OPT_IGNORE_ERROR)
                .long(OPT_IGNORE_ERROR)
                .help("Ignore build errors and continue")
                .takes_value(false),
        )
        .get_matches();

    if matches.is_present(OPT_DOWNLOAD) {
        return download::download();
    }

    // read config
    let mut config_file = matches.value_of(OPT_CONFIG).unwrap_or("pkg/pifu.toml");
    if !Path::new(config_file).exists() {
        config_file = "pifu.toml";
    }
    log::info!("config file: {:?}", config_file);

    let config_content = fs::read_to_string(config_file)
        .unwrap_or_else(|_| panic!("Failed to read config at {}", config_file));
    let mut conf: Config = toml::from_str(&config_content).expect("Invalid config");

    conf.metadata.build_id = expand_file_macro_simple(&conf.metadata.build_id)?;

    let mut options = build::BuildOptions {
        ignore_error: matches.is_present(OPT_IGNORE_ERROR),
        ..Default::default()
    };

    if let Some(os_list) = matches.values_of(OPT_OS) {
        options.targets.clear();
        for os in os_list {
            if os == "linux" {
                options.targets.extend([
                    PlatformTarget::Deb,
                    PlatformTarget::Rpm,
                    PlatformTarget::AppImage,
                ]);
            } else if os == "win" {
                options.targets.push(PlatformTarget::Nsis);
            } else {
                log::error!("Invalid --os {}", &os);
                return Err(Error::from_string(
                    ErrorKind::CmdlineError,
                    format!("Invalid --os {}, available values are `linux` or `win`", os),
                ));
            }
        }
    }

    if let Some(target_list) = matches.values_of(OPT_TARGET) {
        options.targets.clear();
        for target in target_list {
            if let Ok(target) = PlatformTarget::from_str(target) {
                options.targets.push(target);
            } else {
                return Err(Error::from_string(
                    ErrorKind::CmdlineError,
                    format!("Invalid --target {}, available values are `deb`, `rpm`, `app_image` or `nsis`", target),
                ));
            }
        }
    }

    if let Some(arch_list) = matches.values_of(OPT_ARCH) {
        options.arches.clear();
        for arch in arch_list {
            if let Ok(arch) = Arch::from_str(arch) {
                options.arches.push(arch);
            } else {
                return Err(Error::from_string(
                    ErrorKind::CmdlineError,
                    format!(
                        "Invalid --arch {}, available values are `x86_64`, `x86` or `aarch64`",
                        arch
                    ),
                ));
            }
        }
    }

    log::debug!("options: {:#?}", options);
    build::build(conf, &options)
}
