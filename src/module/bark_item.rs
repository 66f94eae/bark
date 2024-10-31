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


use crate::{config, module::base_item::BaseItem, traits::sender::Sender, util::file_utils};

use super::{msg::Msg, run_file::RunFile};

const TEAM_ID: &str = "5U8LBRXG3A";
const AUTH_KEY_ID: &str = "LH4T9V5U4R";
const TOPIC: &str = "me.fin.bark";

const KEY: &[u8] = r#"-----BEGIN PRIVATE KEY----- 
MIGTAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBHkwdwIBAQQg4vtC3g5L5HgKGJ2+ 
T1eA0tOivREvEAY2g+juRXJkYL2gCgYIKoZIzj0DAQehRANCAASmOs3JkSyoGEWZ 
sUGxFs/4pw1rIlSV2IC19M8u3G5kq36upOwyFWj9Gi3Ejc9d3sC7+SHRqXrEAJow 
8/7tRpV+ 
-----END PRIVATE KEY-----
"#.as_bytes();
pub struct Bark<'a> {
    base: BaseItem<'a>,
}


impl <'a> Bark<'a> {
    pub fn new(msg: &'a Msg) -> Self {
        Self {
            base : BaseItem::new(msg, TEAM_ID, AUTH_KEY_ID, TOPIC, KEY, config::RUN_FILE_BARK)
        }
    }

}

impl <'a> Sender for Bark<'a> {
    fn msg(&self) -> &Msg {
        self.base.msg()
    }
    fn run_file(&self) -> RunFile {
        file_utils::read_runfile_from_file(self.run_file_path())
    }

    fn run_file_path(&self) -> &str {
        &self.base.run_file()
    }

    fn auth_key_id(&self) -> &str {
        self.base.auth_key_id()
    }

    fn team_id(&self) -> &str {
        self.base.team_id()
    }

    fn key(&self) -> &[u8] {
        self.base.key()
    }
    
    fn topic(&self) -> &str {
        self.base.topic()
    }
}