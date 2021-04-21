// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::config::{Config, LinuxTarget};
use crate::deb::build_deb;
use crate::BuildError;

pub fn build_linux(conf: &Config) -> Result<(), BuildError> {
    let linux_conf = if let Some(linux_conf) = conf.linux.as_ref() {
        linux_conf
    } else {
        return Ok(());
    };

    if linux_conf.targets.contains(&LinuxTarget::Deb) {
        build_deb(conf, linux_conf)?;
    }

    Ok(())
}
