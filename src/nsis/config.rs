// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::fmt;

use crate::base::fileset::FileSet;

/// NsisConfig is defined based on https://www.electron.build/configuration/nsis
#[derive(Debug, Deserialize)]
pub struct NsisConfig {
    pub files: Option<Vec<FileSet>>,

    /// Boolean - Whether to create one-click installer or assisted.
    #[serde(default = "default_true")]
    pub one_click: bool,

    /// Boolean - Whether to show install mode installer page (choice per-machine or per-user) for assisted installer.
    /// Or whether installation always per all users (per-machine).
    #[serde(default = "default_false")]
    pub per_machine: bool,

    /// Boolean - assisted installer only. Allow requesting for elevation. If false, user will have to restart installer with elevated permissions.
    #[serde(default = "default_true")]
    pub allow_elevation: bool,

    /// Boolean - assisted installer only. Whether to allow user to change installation directory.
    #[serde(default = "default_true")]
    pub allow_to_change_installation_directory: bool,

    /// String - The path to installer icon, relative to the build resources or to the project directory.
    /// Defaults to build/installerIcon.ico or application icon.
    pub installer_icon: String,

    /// String - The path to uninstaller icon, relative to the build resources or to the project directory.
    /// Defaults to build/uninstallerIcon.ico or application icon.
    pub uninstaller_icon: String,

    /// String - assisted installer only. MUI_HEADERIMAGE, relative to the build resources or to the project directory.
    pub installer_header: Option<String>,

    /// String - one-click installer only. The path to header icon (above the progress bar),
    /// relative to the build resources or to the project directory.
    /// Defaults to build/installerHeaderIcon.ico or application icon.
    pub installer_header_icon: Option<String>,

    /// String - assisted installer only. MUI_WELCOMEFINISHPAGE_BITMAP,
    /// relative to the build resources or to the project directory.
    /// Defaults to build/installerSidebar.bmp or ${NSISDIR}\\Contrib\\Graphics\\Wizard\\nsis3-metro.bmp.
    /// Image size 164 × 314 pixels.
    pub installer_sidebar: Option<String>,

    /// String - assisted installer only. MUI_UNWELCOMEFINISHPAGE_BITMAP,
    /// relative to the build resources or to the project directory.
    /// Defaults to installerSidebar option or build/uninstallerSidebar.bmp or build/installerSidebar.bmp
    /// or ${NSISDIR}\\Contrib\\Graphics\\Wizard\\nsis3-metro.bmp.
    pub uninstaller_sidebar: Option<String>,

    /// String - The uninstaller display name in the control panel.
    /// Default is `${productName} ${version}`.
    pub uninstall_display_name: Option<String>,

    /// String - The path to NSIS include script to customize installer. Defaults to build/installer.nsh.
    pub include: Option<String>,

    /// String - The path to NSIS script to customize installer. Defaults to build/installer.nsi.
    pub script: Option<String>,

    /// Boolean - Whether to create Unicode installer.
    #[serde(default = "default_true")]
    pub unicode: bool,

    /// String. Default guid is generated based on `app_id` or `name`.
    pub guid: Option<String>,

    /// Boolean - If warningsAsErrors is true (default): NSIS will treat warnings as errors.
    /// If warningsAsErrors is false: NSIS will allow warnings.
    #[serde(default = "default_true")]
    pub warnings_as_errors: bool,

    /// Boolean - Whether to run the installed application after finish.
    /// For assisted installer corresponding checkbox will be removed.
    #[serde(default = "default_true")]
    pub run_after_finish: bool,

    /// Boolean | “always” - Whether to create desktop shortcut.
    /// Set to always if to recreate also on reinstall (even if removed by user).
    #[serde(default = "default_true")]
    pub create_desktop_shortcut: bool,

    /// Boolean - Whether to create start menu shortcut.
    #[serde(default = "default_true")]
    pub create_start_menu_shortcut: bool,

    #[serde(default = "default_compress_method")]
    pub compress_method: CompressMethod,
}

#[derive(Debug, Deserialize)]
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
            CompressMethod::BZip2 => write!(f, "bzip2"),
            CompressMethod::Lzma => write!(f, "lzma"),
            CompressMethod::Zlib => write!(f, "zlib"),
        }
    }
}

const fn default_false() -> bool {
    false
}

const fn default_true() -> bool {
    true
}

const fn default_compress_method() -> CompressMethod {
    CompressMethod::Lzma
}
