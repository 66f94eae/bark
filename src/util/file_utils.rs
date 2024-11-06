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



pub fn write_runfile_to_file(path: &str, content: &RunFile) -> Result<(), std::io::Error> {
    match content.to_string() {
        Ok(content) => std::fs::write(path, content),
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "parse content failed"))
    }
}

pub fn read_runfile_from_file(path: &str) -> RunFile {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            if let Ok(rf) = RunFile::from_string(&content) {
                RunFile::new(rf, path.to_string())
            } else {
                RunFile::new_empty(path.to_string())
            }
        },
        Err(_e) => {
            RunFile::new_empty(path.to_string())
        }
    }
}


#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf};

    use super::*;
    
    
    pub fn tmp_path() ->  PathBuf {
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
        
        assert_eq!(HashMap::from([("nick_name3".to_string(), "nick_name3".to_string())]), rf.translate_to_real_devices(vec!["nick_name3".to_string()].as_ref()));

        rf.remove_user_info(vec![&"nick_name3".to_string()]);

        write_runfile_to_file(tmp_path().to_str().unwrap(), &rf).unwrap();
    }
}