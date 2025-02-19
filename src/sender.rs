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


use bark_dev::{bark::Bark, msg::Msg};

use crate::{module::run_file::RunFile, util::file_utils};


pub struct Sender {
    run_file_path: String,
    bark: Bark
}

impl Sender {

    pub fn new(run_file_path: String) -> Self {
        let rf: RunFile = Self::run_file_inner(&run_file_path);
        if let Some(token) = rf.get_token() {
            let bark = Bark::born(token.get_refresh_at(), token.get_token().to_string());
            Self {
                run_file_path,
                bark
            }
        } else {
            Self {
                run_file_path,
                bark: Bark::new()
            }
        }
   }
    
    fn run_file(&self) -> RunFile {
       Self::run_file_inner(&self.run_file_path)
    }

    fn run_file_inner(path: &str) -> RunFile {
        file_utils::read_runfile_from_file(path)
    }

    pub fn send(&mut self, msg: &Msg, devices: &Vec<String>) {
        let alias_devices: std::collections::HashMap<String, String> = self.run_file().translate_to_real_devices(&devices);
        let devices = alias_devices
            .iter()
            .map(|(_alias, real_device)| real_device.to_string())
            .collect::<Vec<String>>();

        let send_result: Option<Vec<String>> = self.bark.send(msg, devices);
        let (time_stamp, token) = self.bark.token();
        self.run_file().set_token(time_stamp, token.as_str());

        if let Some(failed) =  send_result {
            failed.iter().for_each(|device| {
                eprintln!("Send to {} failed", alias_devices.get(device).unwrap());
            });
        }
    }
}