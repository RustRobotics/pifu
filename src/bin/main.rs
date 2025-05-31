// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use pifu::{read_cmdline, Error};

fn main() -> Result<(), Error> {
    if cfg!(debug_assertions) {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    read_cmdline()
}
