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


use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    nick_name: String,
    device_token: String
}

impl UserInfo {
    pub fn new(nick_name: &str, device_token: &str) -> UserInfo {
        UserInfo {
            nick_name: nick_name.to_string(),
            device_token: device_token.to_string()
        }
   }
   
   pub fn get_nick_name(&self) -> &str {
       &self.nick_name
   }
   pub fn get_device_token(&self) -> &str {
       &self.device_token
   }
}

impl std::str::FromStr for UserInfo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (nick_name, device_token) = s.split_once(":").unwrap_or_else(|| ("",""));
        if nick_name.is_empty() || device_token.is_empty() {
            return Err("Please input valid char like \"alias:device_token\"".to_string());
        }
        Ok(UserInfo::new(nick_name, device_token))
    }
}

impl std::fmt::Display for UserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.nick_name, self.device_token)
    }
}