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

const HEAD_NICKE_NAME: &str = "ALIAS";
const HEAD_DEVICE_TOKEN: &str = "DEVICE_TOKEN";

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

    pub fn pretty_print(users: Vec<UserInfo>) {
        let max_nicke_name_len: usize = std::cmp::max(users.iter().map(|u| u.nick_name.len()).max().unwrap_or_else(|| 1), HEAD_NICKE_NAME.len());
        let max_device_token_len: usize = std::cmp::max(users.iter().map(|u| u.device_token.len()).max().unwrap_or_else(|| 1), HEAD_DEVICE_TOKEN.len());
    
        println!("{:<max_nicke_name_len$}    {:<max_device_token_len$}", HEAD_NICKE_NAME, HEAD_DEVICE_TOKEN);
        println!("{:-<max_nicke_name_len$}    {:-<max_device_token_len$}", "", "");

        users.iter().for_each(|u| {
            println!("{:<max_nicke_name_len$}    {:<max_device_token_len$}", u.nick_name, u.device_token);
        });
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_user_info_from_str() {
        let mut us: Vec<UserInfo> = Vec::new();
        for i in vec![1,9,10,99,100,999,1000] {
            let u = format!("alias_{}:xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx{}", i, i);
            let u = u.parse::<UserInfo>().unwrap();
            us.push(u);
        }

        UserInfo::pretty_print(us);
        println!();
        UserInfo::pretty_print(vec![]);

    }
}