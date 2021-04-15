// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DebConfig {
    pub priority: String,

    pub section: Option<String>,
    pub depends: Option<String>,
    pub recommends: Option<String>,
    pub conflicts: Option<String>,
    pub replace: Option<String>,
}
