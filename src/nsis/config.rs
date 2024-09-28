// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::base::fileset::FileSet;
use crate::base::utils::{default_false, default_true};

/// `NsisConfig` is defined based on <https://www.electron.build/configuration/nsis>
#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct NsisConfig {
    pub files: Option<Vec<FileSet>>,

    /// Boolean - Whether to create one-click installer or assisted.
    #[serde(default = "default_true")]
    pub one_click: bool,

    /// Boolean - Whether to show install mode installer page (choice per-machine or per-user)
    /// for assisted installer.
    ///
    /// Or whether installation always per all users (per-machine).
    #[serde(default = "default_false")]
    pub per_machine: bool,

    /// Boolean - assisted installer only.
    ///
    /// Allow requesting for elevation.
    /// If false, user will have to restart installer with elevated permissions.
    #[serde(default = "default_true")]
    pub allow_elevation: bool,

    /// Boolean - assisted installer only.
    ///
    /// Whether to allow user to change installation directory.
    #[serde(default = "default_true")]
    pub allow_to_change_installation_directory: bool,

    /// String - The path to installer icon.
    pub installer_icon: String,

    /// String - The path to uninstaller icon.
    pub uninstaller_icon: String,

    /// String - assisted installer only. `MUI_HEADERIMAGE`
    pub installer_header: Option<String>,

    /// String - one-click installer only.
    ///
    /// The path to header icon (above the progress bar),
    /// Image format is bmp, and image size is 150x57 pixels.
    pub installer_header_icon: Option<String>,

    /// String - assisted installer only. `MUI_WELCOMEFINISHPAGE_BITMAP`.
    ///
    /// Image format is bmp, and image size 164 × 314 pixels.
    pub installer_sidebar: Option<String>,

    /// String - assisted installer only. `MUI_UNWELCOMEFINISHPAGE_BITMAP`.
    ///
    /// Image format is bmp, and image size 164 × 314 pixels.
    pub uninstaller_sidebar: Option<String>,

    /// String - The uninstaller display name in the control panel.
    ///
    /// Default is `${product_name} ${version}`.
    pub uninstall_display_name: Option<String>,

    /// String - The path to NSIS include script to customize installer.
    pub include: Option<String>,

    /// String - The path to NSIS script to customize installer.
    ///
    /// Not recommanded.
    pub script: Option<String>,

    /// String - The artifact file name template.
    ///
    /// Default is `${product_name} Setup ${version}.${ext}`.
    #[serde(default = "default_artifact_name")]
    pub artifact_name: String,

    /// Boolean - one-click installer only. Whether to delete app data on uninstall.
    #[serde(default = "default_false")]
    pub delete_app_data_on_uninstall: bool,

    /// Boolean - Whether to create Unicode installer.
    #[serde(default = "default_true")]
    pub unicode: bool,

    /// String.
    ///
    /// Default guid is generated based on `app_id` or `name`.
    pub guid: Option<String>,

    /// Boolean.
    ///
    /// If warningsAsErrors is true (default): NSIS will treat warnings as errors.
    /// If warningsAsErrors is false: NSIS will allow warnings.
    #[serde(default = "default_true")]
    pub warnings_as_errors: bool,

    /// Boolean - Whether to run the installed application after finish.
    ///
    /// For assisted installer corresponding checkbox will be removed.
    #[serde(default = "default_true")]
    pub run_after_finish: bool,

    /// Boolean - Whether to run app on user login to desktop environment.
    #[serde(default = "default_false")]
    pub run_on_startup: bool,

    /// Boolean | “always” - Whether to create desktop shortcut.
    ///
    /// Set to always if to recreate also on reinstall (even if removed by user).
    #[serde(default = "default_true")]
    pub create_desktop_shortcut: bool,

    /// Boolean - Whether to create start menu shortcut.
    #[serde(default = "default_true")]
    pub create_start_menu_shortcut: bool,

    #[serde(default = "default_compress_method")]
    pub compress_method: CompressMethod,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CompressMethod {
    #[serde(alias = "bzip2")]
    BZip2,

    #[serde(alias = "lzma")]
    Lzma,

    #[serde(alias = "zlib")]
    Zlib,
}

impl fmt::Display for CompressMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BZip2 => write!(f, "bzip2"),
            Self::Lzma => write!(f, "lzma"),
            Self::Zlib => write!(f, "zlib"),
        }
    }
}

fn default_artifact_name() -> String {
    "${product_name} Setup ${version}.${ext}".to_string()
}

const fn default_compress_method() -> CompressMethod {
    CompressMethod::Lzma
}
