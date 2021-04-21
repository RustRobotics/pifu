// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::config::{Config, LinuxConfig};
use crate::BuildError;

pub fn build_deb(conf: &Config, linux_conf: &LinuxConfig) -> Result<(), BuildError> {
    let deb_conf = if let Some(deb_conf) = linux_conf.deb.as_ref() {
        deb_conf
    } else {
        // TODO(Shaohua): Returns error
        return Ok(());
    };

    Ok(())
}
