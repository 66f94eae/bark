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


use clap::{ArgMatches, Command, CommandFactory, FromArgMatches};

use crate::module::msg::Msg;


#[derive(clap::Parser, Debug)]
#[clap(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = "cli msg sender hio",
    long_about = env!("CARGO_PKG_DESCRIPTION"),
    override_usage = env!("CARGO_PKG_NAME").to_owned() + " [OPTIONS] -m <MSG> -r <RECEIVER>..."
)]
pub struct CMD {
    /// title
    #[clap(short, long, required = false, default_value = "Notification")]
    pub title: String,
    /// msg content
    #[clap(short, long)]
    pub msg: String,
    /// send to whom in format of user1,user2...
    #[clap(short, long, value_delimiter = ',')]
    pub receiver: Vec<String>,
    /// after how many seconds to send, positive number [1..]
    #[clap(short, long, required = false, value_parser = clap::value_parser!(u64).range(1..))]
    pub delay: Option<u64>,
    /// Push Interruption Level(active, timeSensitive, passive)
    #[clap(short, long, required = false, default_value = "active")]
    pub level: String,
    /// Push Badge
    #[clap(short, long, required = false, value_parser = clap::value_parser!(u64).range(0..=9999999999))]
    pub badge: Option<u64>,
    /// Group messages
    /// pushes will be displayed in groups in the notification center
    #[clap(short, long, required = false, verbatim_doc_comment)]
    pub group: Option<String>,
    /// You can set different ringtones for the push
    #[clap(short, long, required = false, default_value = "chime.caf")]
    pub sound: String,
    /// Set a custom icon for the push
    /// the set icon will replace the default Bark icon
    #[clap(short, long, required = false, default_value = "https://github.com/66f94eae/bark/raw/main/bot.jpg", verbatim_doc_comment)]
    pub icon: String,
    /// Pass true to save the push else will not save the push
    /// if not passed, it will be decided according to the app's internal settings
    #[clap(long, required = false, verbatim_doc_comment)]
    pub archive: Option<bool>,
    /// Pass false to disable
    /// Automatically copy push content below iOS 14.5
    /// above iOS 14.5, you need to manually long-press the push or pull down the push
    #[clap(long, required = false, default_value = "true", verbatim_doc_comment)]
    pub auto_copy: Option<bool>,
    /// When copying the push, specify the content to copy
    /// if this parameter is not provided, the entire push content will be copied
    #[clap(short, long, required = false, verbatim_doc_comment)]
    pub copy: Option<String>,
    /// The URL to jump to when clicking the push, supports URL Scheme and Universal Link
    #[clap(short, long, required = false)]
    pub url: Option<String>,
    /// aes128
    #[clap(long, required = false, conflicts_with_all = &["aes192", "aes256"])]
    pub aes128: bool,
    /// aes256
    #[clap(long, required = false, conflicts_with_all = &["aes128", "aes192"])]
    pub aes256: bool,
    /// aes192
    #[clap(long, required = false, conflicts_with_all = &["aes128", "aes256"])]
    pub aes192: bool,
    /// cbc mode
    #[clap(long, required = false, conflicts_with_all = &["ecb", "gcm"])]
    pub cbc: bool,
    /// ecb mode
    #[clap(long, required = false, conflicts_with_all = &["cbc", "gcm"])]
    pub ecb: bool,
    /// gcm mode
    #[clap(long, required = false, conflicts_with_all = &["cbc", "ecb"])]
    pub gcm: bool,
    /// encryption key
    #[clap(short, long, required = false)]
    pub key: Option<String>,
    /// iv
    #[clap(long, required = false)]
    pub iv: Option<String>,
}

impl CMD {
    pub fn parse() -> Self {
        let mut cmd = CMD::command();
        let long_version: String = format!("{}\ncommit: {}\nbuild-date: {}", env!("CARGO_PKG_VERSION"), env!("GIT_COMMIT"),env!("BUILD_DATE"));
        let long_version: &str = Box::leak(long_version.into_boxed_str());
        cmd = cmd.long_version(long_version);
     
        let mut matches: ArgMatches = cmd.get_matches_mut();

        match CMD::from_arg_matches(&mut matches) {
            Ok(c) => {
                c.validate(&mut cmd);
                c
            },
            Err(e) => e.exit(),
        }

    }

    fn validate(&self, cmd: &mut Command) {
        let type_set: u8 = self.aes128 as u8 + self.aes192 as u8 + self.aes256 as u8;
        let mode_set: u8 = self.cbc as u8 + self.ecb as u8 + self.gcm as u8;

        if type_set > 1 {
            cmd.error(clap::error::ErrorKind::ArgumentConflict, "aes128, aes192, aes256 can only set one at the same time")
                .exit();
        }

        if mode_set > 1 {
            cmd.error(clap::error::ErrorKind::ArgumentConflict, "cbc, ecb, gcm mode can only set one at the same time")
                .exit(); 
        }

        match type_set + mode_set {
            0 => {
                if self.key.is_some() {
                    cmd.error(clap::error::ErrorKind::MissingRequiredArgument, "ase encryption type and mode are required when key is set")
                        .exit();
                }
            },
            1 => {
                let err_msg: &str;
                if type_set == 1 {
                    err_msg = "mode is required when aes encryption type is set";
                 } else {
                    err_msg = "asencryption type is required when mode is set";
                 }
                cmd.error(clap::error::ErrorKind::MissingRequiredArgument, err_msg)
                    .exit();
            },
            2 => {
                if self.key.is_none() {
                    cmd.error(clap::error::ErrorKind::MissingRequiredArgument, "key is required when aes encryption type and mode are set")
                        .exit();
                }
            },
            _ => {
                // This should never happen
                panic!("invalid aes encryption type and mode");
            },
        }

        if self.receiver.is_empty() {
            cmd.error(clap::error::ErrorKind::MissingRequiredArgument, "receiver is required and can not be empty")
                .exit();
        }
        
    }

    pub fn to_msg(&self) -> Msg {

        let mut msg: Msg = Msg::new(self.title.clone(), self.msg.clone());
        msg.set_level(self.level.clone());
        if let Some(badge) = self.badge {
            msg.set_badge(badge);
        }
        msg.set_sound(self.sound.clone());
        msg.set_icon(self.icon.clone());
        if let Some(group) = self.group.clone() {
            msg.set_group(group);
        }
        if let Some(archive) = self.archive {
            msg.set_is_archive(if archive { 1 } else { 0 });
        }
        if let Some(auto_copy) = self.auto_copy {
            msg.set_auto_copy(auto_copy as u8);
        }
        if let Some(copy) = self.copy.clone() {
            msg.set_copy(copy);
        }

        if let Some(url) = self.url.clone() {
            msg.set_url(url);
        }

        if self.aes128 {
            msg.set_enc_type("aes128".to_string());
        } else if self.aes192 {
            msg.set_enc_type("aes192".to_string());
        } else if self.aes256 {
            msg.set_enc_type("aes256".to_string());
        }

        if self.cbc {
            msg.set_mode("cbc".to_string());
        } else if self.ecb {
            msg.set_mode("ecb".to_string());
        } else if self.gcm {
            msg.set_mode("gcm".to_string());
        }

        if let Some(key) = self.key.clone() {
            msg.set_key(key);

            if let Some(iv) = self.iv.clone() {
                msg.set_iv(iv);
            } else {
                let mut iv: [u8; 16] = [0u8; 16];
                openssl::rand::rand_bytes(&mut iv).unwrap();
                msg.set_iv(iv.iter().map(|b| format!("{:02x}", b)).collect::<String>().split_off(16));
            }

        }
        msg
    }
}