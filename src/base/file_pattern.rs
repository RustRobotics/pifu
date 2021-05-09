// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use chrono::prelude::*;
use std::process::Command;

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

// TODO(Shaohua): Replace all instance
pub fn expand_file_pattern(s: &str) -> Result<String, BuildError> {
    let mut content = s.to_string();
    if let Some(git_offset) = content.find("${git}") {
        let hash = get_git_hash()?;
        content.replace_range(git_offset..git_offset + 6, &hash);
    }

    if let Some(date_offset) = content.find("${date}") {
        let t = get_date();
        content.replace_range(date_offset..date_offset + 7, &t);
    }
    if let Some(date_time_offset) = content.find("${date-time}") {
        let t = get_date_time();
        content.replace_range(date_time_offset..date_time_offset + 12, &t);
    }
    if let Some(timestamp_offset) = content.find("${timestamp}") {
        let t = get_timestamp();
        content.replace_range(timestamp_offset..timestamp_offset + 12, &t);
    }

    Ok(content)
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
    fn test_expand_file_pattern() {
        let s = "app-${git}.deb";
        let ret = expand_file_pattern(s);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().len(), 15);

        let s = "app-${date}.deb";
        let ret = expand_file_pattern(s);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().len(), 16);

        let s = "app-${date-time}.deb";
        let ret = expand_file_pattern(s);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().len(), 22);

        let s = "app-${timestamp}.deb";
        let ret = expand_file_pattern(s);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().len(), 18);
    }
}
