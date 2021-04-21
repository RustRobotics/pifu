// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use clap::{App, Arg};
use std::fs;

use crate::build_linux::build_linux;
use crate::config::Config;
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

    let config_file = matches.value_of("config").unwrap_or("rs-builder.toml");
    log::info!("config file: {:?}", config_file);

    let config_content =
        fs::read_to_string(config_file).expect(&format!("Failed to read {}", config_file));

    let conf: Config = toml::from_str(&config_content).expect("Invalid config");

    build_linux(&conf)
}
