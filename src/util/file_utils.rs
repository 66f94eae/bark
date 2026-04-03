// MIT License
//
// Copyright (c) 2024 66f94eae
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


use crate::module::run_file::RunFile;

use std::path::{Path, PathBuf};

/// Resolve run file path templates (e.g. leading `~`, Windows `%VAR%`).
///
/// `build.rs` injects OS/user independent templates at compile time.
/// At runtime we expand them to an absolute path so the run file can be persisted.
pub fn resolve_runfile_path(path: &str) -> String {
    let mut resolved = path.to_string();

    // 1) Expand leading `~`
    // Examples: "~", "~/a/b", "~\\a\\b"
    if let Some(rest) = resolved.strip_prefix('~') {
        let home: Option<PathBuf> = {
            #[cfg(windows)]
            {
                std::env::var("USERPROFILE")
                    .ok()
                    .map(PathBuf::from)
                    .or_else(|| {
                        let drive = std::env::var("HOMEDRIVE").ok();
                        let p = std::env::var("HOMEPATH").ok();
                        match (drive, p) {
                            (Some(d), Some(p)) => Some(PathBuf::from(format!("{}{}", d, p))),
                            _ => None,
                        }
                    })
            }
            #[cfg(not(windows))]
            {
                std::env::var("HOME").ok().map(PathBuf::from)
            }
        };

        if let Some(home) = home {
            let suffix = rest.trim_start_matches(['/', '\\']);
            resolved = if suffix.is_empty() {
                home.to_string_lossy().to_string()
            } else {
                home.join(suffix).to_string_lossy().to_string()
            };
        }
    }

    // 2) Expand Windows `%VAR%`
    #[cfg(windows)]
    {
        resolved = expand_percent_env_vars(&resolved);
    }

    resolved
}

#[cfg(windows)]
fn expand_percent_env_vars(input: &str) -> String {
    // Strictly expand patterns like `%VAR%` (pairs of `%`).
    // Implementation intentionally avoids regex to keep dependencies unchanged.
    let parts: Vec<&str> = input.split('%').collect();
    if parts.len() < 3 || parts.len() % 2 == 0 {
        return input.to_string();
    }

    let mut out = parts[0].to_string();
    let mut idx = 1;
    while idx + 1 < parts.len() {
        let var_name = parts[idx];
        let after = parts[idx + 1];
        match std::env::var(var_name) {
            Ok(val) => out.push_str(&val),
            Err(_) => {
                out.push('%');
                out.push_str(var_name);
                out.push('%');
            }
        }
        out.push_str(after);
        idx += 2;
    }
    out
}

pub fn write_runfile_to_file(path: &str, content: &RunFile) -> Result<(), std::io::Error> {
    let resolved_path = resolve_runfile_path(path);

    if let Some(parent) = Path::new(&resolved_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    match content.to_string() {
        Ok(content) => std::fs::write(&resolved_path, content),
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "parse content failed",
            ))
        }
    }
}

pub fn read_runfile_from_file(path: &str) -> RunFile {
    let resolved_path = resolve_runfile_path(path);

    match std::fs::read_to_string(&resolved_path) {
        Ok(content) => {
            if let Ok(rf) = RunFile::from_string(&content) {
                RunFile::new(rf, resolved_path)
            } else {
                RunFile::new_empty(resolved_path)
            }
        }
        Err(_e) => RunFile::new_empty(resolved_path),
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use super::*;

    pub fn tmp_path() -> PathBuf {
        std::env::temp_dir().join("run_file_test.toml")
    }

    #[test]
    fn test_write_to_file() {
        let rf = RunFile::new_for_test();
        write_runfile_to_file(tmp_path().to_str().unwrap(), &rf).unwrap();
    }

    #[test]
    fn test_read_from_file() {
        let mut rf = read_runfile_from_file(tmp_path().to_str().unwrap());

        assert_eq!(
            HashMap::from([("nick_name3".to_string(), "nick_name3".to_string())]),
            rf.translate_to_real_devices(vec!["nick_name3".to_string()].as_ref())
        );

        rf.remove_user_info(vec![&"nick_name3".to_string()]);

        write_runfile_to_file(tmp_path().to_str().unwrap(), &rf).unwrap();
    }

    #[test]
    #[cfg(not(windows))]
    fn test_resolve_tilde_home() {
        let old_home = std::env::var("HOME").ok();
        std::env::set_var("HOME", "/tmp/bark-home");

        let resolved = resolve_runfile_path("~/foo/bar");
        assert_eq!(
            PathBuf::from(resolved),
            PathBuf::from("/tmp/bark-home").join("foo").join("bar")
        );

        let resolved_home = resolve_runfile_path("~");
        assert_eq!(
            PathBuf::from(resolved_home),
            PathBuf::from("/tmp/bark-home")
        );

        match old_home {
            Some(v) => std::env::set_var("HOME", v),
            None => std::env::remove_var("HOME"),
        }
    }

    #[test]
    #[cfg(windows)]
    fn test_resolve_percent_localappdata() {
        let old = std::env::var("LOCALAPPDATA").ok();
        std::env::set_var("LOCALAPPDATA", r"C:\Temp\bark-localappdata");

        let resolved = resolve_runfile_path("%LOCALAPPDATA%\\bark\\bark.conf");
        assert_eq!(
            PathBuf::from(resolved),
            PathBuf::from(r"C:\Temp\bark-localappdata")
                .join("bark")
                .join("bark.conf")
        );

        match old {
            Some(v) => std::env::set_var("LOCALAPPDATA", v),
            None => std::env::remove_var("LOCALAPPDATA"),
        }
    }
}