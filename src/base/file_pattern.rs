// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::process::Command;

use crate::BuildError;

/// Get git commit hash of HEAD ref.
fn get_git_hash() -> Result<String, BuildError> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .output()?;
    let hash = String::from_utf8(output.stdout)?;
    Ok(hash)
}

pub fn expand_file_pattern(s: &str) -> String {
    String::from(s)
}

#[cfg(test)]
mod tests {
    use super::get_git_hash;

    #[test]
    fn test_git_hash() {
        let hash = get_git_hash();
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().len(), 8);
    }
}
