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

use super::msg::Msg;


pub struct BaseItem<'a> {
    msg: &'a Msg,
    team_id: &'a str,
    auth_key_id: &'a str,
    topic: &'a str,
    key: &'a [u8],
    run_file: &'a str,
}

impl <'a> BaseItem<'a> {
    pub fn new (msg: &'a Msg, team_id: &'a str, auth_key_id: &'a str, topic: &'a str, key: &'a [u8], run_file: &'a str) -> Self {
        Self {
            msg,
            team_id,
            auth_key_id,
            topic,
            key,
            run_file
        }
    }

    pub fn msg(&self) -> &Msg {
        self.msg
    }
    pub fn run_file(&self) -> &str {
        self.run_file
    }

    pub fn auth_key_id(&self) -> &str {
        self.auth_key_id
    }

    pub fn team_id(&self) -> &str {
        self.team_id
    }

    pub fn key(&self) -> &[u8] {
        self.key
    }
    
    pub fn topic(&self) -> &str {
        self.topic
    }
}