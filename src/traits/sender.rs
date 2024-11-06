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


use crate::module::{msg::Msg, run_file::RunFile};


pub trait Sender {
    fn run_file(&self) -> RunFile;
    fn run_file_path(&self) -> &str;
    fn auth_key_id(&self) -> &str;
    fn team_id(&self) -> &str;
    fn key(&self) -> &[u8];
    fn msg(&self) -> &Msg;
    fn topic(&self) -> &str;

    
    fn send(&self, devices: &Vec<String>) {
        let alias_devices: std::collections::HashMap<String, String> = self.run_file().translate_to_real_devices(&devices);
        let devices: Vec<String> = alias_devices.iter().map(|(_k, v)| v.to_string()).collect::<Vec<String>>();
        if let Some(failed) =  crate::apns::send(self.msg(), self.topic(), devices, &self.get_token()) {
            failed.iter().for_each(|(device, msg)| {
                eprintln!("Send to {} failed: {}", device, msg);
            });
        }
    }

    fn get_token(&self) -> String {
        
        let mut run_file = self.run_file();
        if let Some(token) = run_file.get_token(self.auth_key_id(), self.team_id(), self.key()) {
            token
        } else {
            panic!("token is not valid")
        }
        
    }
    
}