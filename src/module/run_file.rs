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


use std::{collections::HashMap, process::exit};

use crate::util::{file_utils, time_utils};

use super::{token::Token, user_info::UserInfo};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RunFile {
    user_info: Option<Vec<UserInfo>>,
    token: Option<Token>,
    #[serde(skip_serializing, skip_deserializing)]
    path: String
}

impl RunFile {
    
    pub fn new_empty(path: String) -> Self {
        Self {
            user_info: None,
            token: None,
            path: path
        }
    }

    pub fn new(rf: RunFile, path: String) -> Self {
        Self {
            path,
            ..rf
        }
    }
    pub fn get_token(&mut self, auth_key_id: &str, team_id: &str, key: &[u8]) -> Option<String> {
        
        if self.token.is_none() 
            || self.token.as_mut().is_some_and(|t| {
            t.get_refresh_at() - time_utils::curr_time_secs() > 2700
        }) {
            self.token = Some(Token::refresh_token(auth_key_id, team_id, key));
            self.save();
        }

        Some(self.token.as_ref().unwrap().get_token().to_string())
    }
    // pub fn set_token(&mut self, token: &str) {
    //     self.token = Token::new(token);
    // }
    pub fn get_user_info(&self) -> Vec<UserInfo> {
        if let Some(user_info) = &self.user_info {
            user_info.clone()
        } else {
            Vec::new()
        }
    }
    pub fn get_user_info_by_name(&self, name: &str) -> Option<UserInfo> {
        self.get_user_info().iter().find(|u| u.get_nick_name() == name).cloned()
    }
    pub fn add_user_info(&mut self, user_infos: Vec<&UserInfo>) {
        let mut users: Vec<UserInfo> = self.get_user_info();
        let mut tmp_users: Vec<UserInfo> = Vec::<UserInfo>::new();

        for user in user_infos {
            if users.iter().any(|u| u.get_nick_name() == user.get_nick_name()) {
                eprintln!("user {} already exists", user.get_nick_name());
                exit(0x0100);
            }
            tmp_users.push(user.clone());
        }
        
        users.append(&mut tmp_users);
        self.user_info = Some(users);
        
        self.save();
    }
    pub fn remove_user_info(&mut self, names: Vec<&String>) {
        let mut users: Vec<UserInfo> = self.get_user_info();
        for name in names {
            users.iter().position(|u| u.get_nick_name() == name)
                .and_then(|i| {
                    users.remove(i);
                    Some(i)
                });
        }
        self.user_info = Some(users);
        self.save();
    }
    
    pub fn to_string(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(&self)
    }

    pub fn from_string(str: &str) -> Result<RunFile, toml::de::Error> {
        toml::from_str(str)
    }

    /// translate alias to device token
    /// 
    /// if not found, return the name itself
    /// 
    /// return {alias1: real_device_token1, not_found_alias2: not_found_alias2 ... }
    pub fn translate_to_real_devices(&self, names: &Vec<String>) -> HashMap<String,String> {
        let mut devices: HashMap<String, String> = HashMap::<String, String>::new();
        let user_dict: HashMap<String, String> = self.get_user_info().iter().map( |user| (user.get_nick_name().to_string(), user.get_device_token().to_string()))
            .collect::<HashMap::<String, String>>();
        
        for name in names {
            devices.insert(name.clone(), user_dict.get(name).or_else(|| Some(name)).unwrap().to_string());
        }
        
        devices
    }

    fn save(&self) {
        let _ = file_utils::write_runfile_to_file(&self.path, self);
    }
}


#[cfg(test)]
mod tests {


    use super::*;


    impl RunFile {
        pub fn new_for_test() -> RunFile {
            RunFile {
                user_info: Some(
                    vec!(
                        UserInfo::new(
                            "nick_name1",
                            "device_token1"
                        ), 
                        UserInfo::new(
                            "nick_name2",
                            "device_token2"
                        )
                    )
                ),
               token: Some(Token::new_for_test(0, "token")),
               path: "test_run_file.toml".to_string()
            }
        }
    }


    #[test]
    fn test_run_file() {
        let run_file = RunFile::new_for_test();

        let r = run_file.to_string().unwrap();
        assert_eq!("[[user_info]]\nnick_name = \"nick_name1\"\ndevice_token = \"device_token1\"\n\n[[user_info]]\nnick_name = \"nick_name2\"\ndevice_token = \"device_token2\"\n\n[token]\nrefresh_at = 0\ntoken = \"token\"\n", r);
        
        let de_run_file = RunFile::from_string(&r).unwrap();
        let rr = de_run_file.to_string().unwrap();
        assert_eq!(r, rr);
    }

    #[test]
    fn test_find_device_by_name() {
        
        let run_file = RunFile::new_for_test();
        
        assert_eq!(HashMap::from([("nick_name1".to_string(), "device_token1".to_string())]), run_file.translate_to_real_devices(vec!["nick_name1".to_string()].as_ref()));
        assert_eq!(HashMap::from([("nick_name3".to_string(), "nick_name3".to_string())]), run_file.translate_to_real_devices(vec!["nick_name3".to_string()].as_ref()));
    }
}