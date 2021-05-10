// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use chrono::prelude::*;
use regex::Regex;
use std::env;
use std::process::Command;

use super::config::{Arch, PlatformTarget};
use crate::config::Config;
use crate::BuildError;

/// Get git commit hash of HEAD ref.
pub fn get_git_hash() -> Result<String, BuildError> {
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

fn get_env(name: &str) -> Result<String, BuildError> {
    for (key, value) in env::vars() {
        if name == key {
            return Ok(value);
        }
    }
    Err(BuildError::EnvironmentNotSetError)
}

pub fn expand_file_macro_simple(s: &str) -> Result<String, BuildError> {
    let mut content = s.to_string();
    if content.find("${git}").is_some() {
        let hash = get_git_hash()?;
        content = content.replace("${git}", &hash);
    }

    if content.find("${date}").is_some() {
        let t = get_date();
        content = content.replace("${date}", &t);
    }
    if content.find("${date-time}").is_some() {
        let t = get_date_time();
        content = content.replace("${date-time}", &t);
    }
    if content.find("${timestamp}").is_some() {
        let t = get_timestamp();
        content = content.replace("${timestamp}", &t);
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

pub fn expand_file_macro(
    s: &str,
    conf: &Config,
    arch: Arch,
    target: PlatformTarget,
) -> Result<String, BuildError> {
    let mut content = expand_file_macro_simple(s)?;

    if content.find("${ext}").is_some() {
        let ext_name = target.extension();
        content = content.replace("${ext}", ext_name);
    }
    if content.find("${arch}").is_some() {
        let arch_name = arch.to_string();
        content = content.replace("${arch}", &arch_name);
    }
    // TODO(Shaohua): Support ${os} macro

    let key_pattern = Regex::new(r"\$\{(\w+)\}")?;
    let mut new_content = content.clone();
    if key_pattern.is_match(&content) {
        // Expand metadata
        match serde_json::to_value(&conf.metadata)? {
            serde_json::Value::Object(metadata) => {
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
            _ => {}
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
        println!("ret: {:?}", ret);
        assert!(ret.is_ok());
        assert!(ret.unwrap().len() > 9);
    }
}
