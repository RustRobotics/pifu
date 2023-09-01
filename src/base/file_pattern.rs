// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use chrono::prelude::*;
use regex::Regex;
use std::env;
use std::process::Command;

use super::config::{Arch, PlatformTarget};
use crate::config::Config;
use crate::error::{Error, ErrorKind};

/// Get git commit hash of HEAD ref.
pub fn get_git_hash() -> Result<String, Error> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .output()?;
    let hash = String::from_utf8(output.stdout)?;
    Ok(hash.trim().to_string())
}

/// Get date of today, like `20210509`.
pub fn get_date() -> String {
    let now = Local::now();
    now.format("%Y%m%d").to_string()
}

/// Get date and time of today, like `202105090952`.
fn get_date_time() -> String {
    let now = Local::now();
    now.format("%Y%m%d%H%M%S").to_string()
}

/// Get timestamp of now, like `1620525294`.
fn get_timestamp() -> String {
    let now = Local::now();
    now.timestamp().to_string()
}

fn get_env(name: &str) -> Result<String, Error> {
    for (key, value) in env::vars() {
        if name == key {
            return Ok(value);
        }
    }
    Err(Error::from_string(
        ErrorKind::EnvironmentNotSetError,
        format!("Environment with name `{name:?}` not set!"),
    ))
}

/// # Errors
/// Returns error if failed to expand file path.
pub fn expand_file_macro_simple(s: &str) -> Result<String, Error> {
    let mut content = s.to_string();
    let content_git = "${git}";
    if content.contains(content_git) {
        let hash = get_git_hash()?;
        content = content.replace(content_git, &hash);
    }

    let content_date = "${date}";
    if content.contains(content_date) {
        let t = get_date();
        content = content.replace(content_date, &t);
    }
    let content_date_time = "${date-time}";
    if content.contains(content_date_time) {
        let t = get_date_time();
        content = content.replace(content_date_time, &t);
    }
    let content_timestamp = "${timestamp}";
    if content.contains(content_timestamp) {
        let t = get_timestamp();
        content = content.replace(content_timestamp, &t);
    }
    let env_pattern = Regex::new(r"\$\{env\.(\w+)\}")?;
    let mut new_content = content.clone();
    if env_pattern.is_match(&content) {
        for cap in env_pattern.captures_iter(&content) {
            let env_value = get_env(&cap[1])?;
            new_content = new_content.replace(&cap[0], &env_value);
        }
    }

    Ok(new_content)
}

/// # Errors
/// Returns error if failed to expand file path.
pub fn expand_file_macro(
    s: &str,
    conf: &Config,
    arch: Arch,
    target: PlatformTarget,
) -> Result<String, Error> {
    let mut content = expand_file_macro_simple(s)?;

    let content_ext = "${ext}";
    if content.contains(content_ext) {
        let ext_name = target.extension();
        content = content.replace(content_ext, ext_name);
    }
    let content_arch = "${arch}";
    if content.contains(content_arch) {
        let arch_name = arch.to_string();
        content = content.replace(content_arch, &arch_name);
    }
    // TODO(Shaohua): Support ${os} macro

    let key_pattern = Regex::new(r"\$\{(\w+)\}")?;
    let mut new_content = content.clone();
    if key_pattern.is_match(&content) {
        // Expand metadata
        if let serde_json::Value::Object(metadata) = serde_json::to_value(&conf.metadata)? {
            for cap in key_pattern.captures_iter(&content) {
                log::info!("cap1: {:?}", &cap[1]);
                if metadata.get(&cap[1]).is_some() {
                    match metadata[&cap[1]] {
                        serde_json::Value::String(ref new_value) => {
                            new_content = new_content.replace(&cap[0], new_value);
                        }
                        _ => {
                            log::error!("Invalid metadata property: {:?}", &cap[1]);
                        }
                    }
                }
            }
        }

        // TODO(Shaohua): Expand target specific properties.
    }

    Ok(new_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_hash() {
        let hash = get_git_hash();
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().len(), 7);
    }

    #[test]
    fn test_get_date() {
        let t = get_date();
        assert_eq!(t.len(), 8);
    }

    #[test]
    fn test_get_date_time() {
        let t = get_date_time();
        assert_eq!(t.len(), 14);
    }

    #[test]
    fn test_get_timestamp() {
        let t = get_timestamp();
        assert_eq!(t.len(), 10);
    }

    #[test]
    fn test_get_env() {
        let s = get_env("USER");
        assert!(s.is_ok());
    }

    #[test]
    fn test_expand_file_macro_simple() {
        let s = "app-${git}.deb";
        let ret = expand_file_macro_simple(s);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().len(), 15);

        let s = "app-${date}.deb";
        let ret = expand_file_macro_simple(s);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().len(), 16);

        let s = "app-${date-time}.deb";
        let ret = expand_file_macro_simple(s);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().len(), 22);

        let s = "app-${timestamp}.deb";
        let ret = expand_file_macro_simple(s);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().len(), 18);

        let s = "app-${env.USER}.deb";
        let ret = expand_file_macro_simple(s);
        println!("ret: {ret:?}");
        assert!(ret.is_ok());
        assert!(ret.unwrap().len() > 9);
    }
}
