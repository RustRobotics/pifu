// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use clap::{value_parser, Arg, ArgAction, Command};
use std::fs;
use std::path::{Path, PathBuf};
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

/// # Errors
/// Returns error if failed to parse cmdline or failed to read config file.
#[allow(clippy::too_many_lines)]
pub fn read_cmdline() -> Result<(), Error> {
    let matches = Command::new("Pifu - Cross platform package builder")
        .version("0.3.5")
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("General package builder")
        .arg(
            Arg::new(OPT_CONFIG)
                .short('c')
                .long(OPT_CONFIG)
                .action(ArgAction::Set)
                .value_name("toml file")
                .help("Specify a custom toml config file")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new(OPT_OS)
                .long(OPT_OS)
                .action(ArgAction::Set)
                .help("Build for specific OS platform"),
        )
        .arg(
            Arg::new(OPT_TARGET)
                .short('t')
                .long(OPT_TARGET)
                .action(ArgAction::Set)
                .help("Build specific target"),
        )
        .arg(
            Arg::new(OPT_ARCH)
                .short('a')
                .long(OPT_ARCH)
                .action(ArgAction::Set)
                .help("Build specific architecture"),
        )
        .arg(
            Arg::new(OPT_DOWNLOAD)
                .long(OPT_DOWNLOAD)
                .help("Download required tools from github"),
        )
        .arg(
            Arg::new(OPT_IGNORE_ERROR)
                .long(OPT_IGNORE_ERROR)
                .help("Ignore build errors and continue"),
        )
        .get_matches();

    if matches.get_count(OPT_DOWNLOAD) > 0 {
        return download::download();
    }

    // read config
    let mut config_file: String = (*matches
        .get_one::<&str>(OPT_CONFIG)
        .unwrap_or(&"pkg/pifu.toml"))
    .to_string();
    if !Path::new(&config_file).exists() {
        config_file = "pifu.toml".to_owned();
    }
    log::info!("config file: {:?}", config_file);

    let config_content = fs::read_to_string(&config_file).map_err(|err| {
        Error::from_string(
            ErrorKind::IoError,
            format!("Failed to read config at {config_file}, err: {err}"),
        )
    })?;
    let mut conf: Config = toml::from_str(&config_content).map_err(|_err| {
        Error::from_string(
            ErrorKind::TomlError,
            format!("Invalid toml config, {config_file}"),
        )
    })?;

    conf.metadata.build_id = expand_file_macro_simple(&conf.metadata.build_id)?;

    let mut options = build::BuildOptions {
        ignore_error: matches.get_count(OPT_IGNORE_ERROR) > 0,
        ..Default::default()
    };

    if let Some(os_list) = matches.get_many::<&str>(OPT_OS) {
        options.targets.clear();
        for os in os_list {
            if *os == "linux" {
                options.targets.extend([
                    PlatformTarget::Deb,
                    PlatformTarget::Rpm,
                    PlatformTarget::AppImage,
                ]);
            } else if *os == "win" {
                options.targets.push(PlatformTarget::Nsis);
            } else {
                log::error!("Invalid --os {}", &os);
                return Err(Error::from_string(
                    ErrorKind::CmdlineError,
                    format!("Invalid --os {os}, available values are `linux` or `win`"),
                ));
            }
        }
    }

    if let Some(target_list) = matches.get_many::<&str>(OPT_TARGET) {
        options.targets.clear();
        for target in target_list {
            if let Ok(target) = PlatformTarget::from_str(target) {
                options.targets.push(target);
            } else {
                return Err(Error::from_string(
                    ErrorKind::CmdlineError,
                    format!("Invalid --target {target}, available values are `deb`, `rpm`, `app_image` or `nsis`"),
                ));
            }
        }
    }

    if let Some(arch_list) = matches.get_many::<&str>(OPT_ARCH) {
        options.arches.clear();
        for arch in arch_list {
            if let Ok(arch) = Arch::from_str(arch) {
                options.arches.push(arch);
            } else {
                return Err(Error::from_string(
                    ErrorKind::CmdlineError,
                    format!(
                        "Invalid --arch {arch}, available values are `x86_64`, `x86` or `aarch64`"
                    ),
                ));
            }
        }
    }

    log::debug!("options: {:#?}", options);
    build::build(&conf, &options)
}
