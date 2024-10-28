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


use openssl::symm::{Cipher, Crypter, Mode};


pub struct Msg {
    /// Push Title
    title: String,
    
    /// Push Content
    body: String,
    
    /// Push Interruption Level
    /// active: Default value, the system will immediately display the notification on the screen.
    /// timeSensitive: Time-sensitive notification, can be displayed while in focus mode.
    /// passive: Only adds the notification to the notification list, will not display on the screen.
    level: Option<String>,
    
    /// Push Badge, can be any number
    badge: Option<u64>,
    
    /// Pass 0 to disable; Automatically copy push content below iOS 14.5; above iOS 14.5, you need to manually long-press the push or pull down the push
    auto_copy: Option<u8>,
    
    /// When copying the push, specify the content to copy; if this parameter is not provided, the entire push content will be copied
    copy: Option<String>,
    
    /// You can set different ringtones for the push
    sound: Option<String>,
    
    /// Set a custom icon for the push; the set icon will replace the default Bark icon
    icon: Option<String>,
    
    /// Group messages; pushes will be displayed in groups in the notification center
    group: Option<String>,
    
    /// Pass 1 to save the push; passing anything else will not save the push; if not passed, it will be decided according to the app's internal settings
    is_archive: Option<u8>,
    
    /// The URL to jump to when clicking the push, supports URL Scheme and Universal Link
    url: Option<String>,

    /// iv
    iv: Option<String>,
    /// encrypt type
    enc_type: Option<String>,
    /// encrypt mode
    mode: Option<String>,
    /// encrypt key
    key: Option<String>,
    /// cipher
    cipher: Option<Cipher>,
}

impl Msg {
    pub fn new(title: String, body: String) -> Self {
        Msg {
            title,
            body,
            level: None,
            badge: None,
            auto_copy: None,
            copy: None,
            sound: None,
            icon: None,
            group: None,
            is_archive: None,
            url: None,
            iv: None,
            enc_type: None,
            mode: None,
            key: None,
            cipher: None,
        }
    }

    pub fn set_level(&mut self, level: String) -> &mut Self {
        match level.to_lowercase().as_str() {
            "active" => self.level = Some("active".to_string()),
            "timesensitive" => self.level = Some("timeSensitive".to_string()),
            "passive" => self.level = Some("passive".to_string()),
            _ => self.level = None,
        }
        self
    }

    pub fn set_badge(&mut self, badge: u64) -> &mut  Self {
        if badge > 0 {
            self.badge = Some(badge);
        } else {
            self.badge = None;
        }
        self
    }

    pub fn set_auto_copy(&mut self, auto_copy: u8) -> &mut Self {
        match auto_copy {
            0 => self.auto_copy = Some(0),
            _ => self.auto_copy = None,
        }
        self
    }

    pub fn set_copy(&mut self, copy: String) -> &mut Self {
        if copy.trim().is_empty() {
            self.copy = None;
        } else {
            self.copy = Some(copy);
        }
        self
    }

    pub fn set_sound(&mut self, sound: String) -> &mut Self {
        self.sound = Some(sound);
        self
    }

    pub fn set_icon(&mut self, icon: String) -> &mut Self {
        if icon.trim().is_empty() {
            self.icon = None;
        } else {
            self.icon = Some(icon);
        }
        self
    }

    pub fn set_group(&mut self, group: String) -> &mut Self {
        self.group = Some(group);
        self
    }

    pub fn set_is_archive(&mut self, is_archive: u8) -> &mut Self {
        match is_archive {
            1 => self.is_archive = Some(1),
            _ => self.is_archive = None,
        }
        self
    }

    pub fn set_url(&mut self, url: String) -> &mut Self {
        if url.trim().is_empty() {
            self.url = None;
        } else {
            self.url = Some(url);
        }
        self
    }

    pub fn set_iv(&mut self, iv: String) -> &mut Self {
        if iv.trim().is_empty() {
            self.iv = None;
        } else {
            self.iv = Some(iv);
        }
        self
    }

    fn set_cipher(&mut self) -> &mut Self {
        if self.enc_type.is_none() || self.mode.is_none() {
            return self;
        }
        let enc_type = self.enc_type.as_ref().unwrap().clone();
        let mode = self.mode.as_ref().unwrap().clone();

        let cipher: Cipher = match enc_type.to_lowercase().as_str() {
            "aes128" => {
                match mode.to_lowercase().as_str() {
                    "cbc" => {
                        Cipher::aes_128_cbc()
                    },
                    "ecb" => {
                        Cipher::aes_128_ecb()
                    },
                    "gcm" => {
                        Cipher::aes_128_gcm()
                    },
                    _ => panic!("Invalid mode"),
                }
            },
            "aes192" => {
                match mode.to_lowercase().as_str() {
                    "cbc" => {
                        Cipher::aes_192_cbc()
                    },
                    "ecb" => {
                        Cipher::aes_192_ecb()
                    },
                    "gcm" => {
                        Cipher::aes_192_gcm()
                    },
                    _ => panic!("Invalid mode"),
                }
            }, 
            "aes256" => {
                match mode.to_lowercase().as_str() {
                    "cbc" => {
                        Cipher::aes_256_cbc()
                    },
                    "ecb" => {
                        Cipher::aes_256_ecb()
                    },
                    "gcm" => {
                        Cipher::aes_256_gcm()
                    },
                    _ => panic!("Invalid mode"),
                }
            },
            _ => panic!("Invalid type"),
        };
        self.cipher = Some(cipher);
        self
    }

    pub fn set_enc_type(&mut self, enc_type: String) -> &mut Self {
       if self.enc_type.is_some() {
            panic!("Encrypt type can only be set once");
        }
        match enc_type.to_lowercase().as_str() {
            "aes128" => self.enc_type = Some("aes128".to_string()),
            "aes192" => self.enc_type = Some("aes192".to_string()),
            "aes256" => self.enc_type = Some("aes256".to_string()),
            _ => panic!("Invalid encrypt type"),
        }
        self.set_cipher();
        self
    }

    pub fn set_mode(&mut self, mode: String) -> &mut Self {
        if self.mode.is_some() {
            panic!("Encrypt mode can only be set once");
        }
        match mode.to_lowercase().as_str() {
            "cbc" => self.mode = Some("cbc".to_string()),
            "ecb" => self.mode = Some("ecb".to_string()),
            "gcm" => self.mode = Some("gcm".to_string()),
            _ => panic!("Invalid encrypt mode"),
        }
        self.set_cipher();
        self
    }

    pub fn set_key(&mut self, key: String) -> &mut Self {
        self.key = Some(key);
        self
    }
    
    fn json(&self, encry_body: Option<String>) -> String {
        let mut body: String = format!("{{\"aps\":{{\"mutable-content\":1,\"category\":\"myNotificationCategory\",\"interruption-level\":\"{level}\",", level = self.level.as_ref().unwrap_or(&"active".to_string()));

        if let Some(badge) = self.badge {
            body += &format!("\"badge\":{badge},", badge = badge);
        }

        if let Some(sound) = &self.sound {
            body += &format!("\"sound\":\"{sound}\",", sound = sound);
        }

        if let Some(group) = &self.group {
            body += &format!("\"thread-id\":\"{group}\",", group = group);
        }

        let alert: String = format!("\"alert\":{{\"title\":\"{title}\",\"body\":\"{body}\"}}}}",
            title = self.title, body = if encry_body.is_some() {"NoContent"} else {self.body.as_str()}
        );

        body = body + &alert;

        if let Some(icon) = &self.icon {
            body += &format!(",\"icon\":\"{icon}\"", icon = icon);
        }

        if let Some(auto_copy) = self.auto_copy {
            body += &format!(",\"autoCopy\":{auto_copy}", auto_copy = auto_copy);
        }

        if let Some(is_archive) = self.is_archive {
            body += &format!(",\"isArchive\":{is_archive}", is_archive = is_archive);
        }

        if let Some(copy) = &self.copy {
            body += &format!(",\"copy\":\"{copy}\"", copy = copy);
        }

        if let Some(url) = &self.url {
            body += &format!(",\"url\":\"{url}\"", url = url);
        }

        if let Some(iv) = &self.iv {
            body += &format!(",\"iv\":\"{iv}\"", iv = iv);
        }
        
        if let Some(encry_body) = encry_body {
            body += &format!(",\"ciphertext\":\"{encry_body}\"", encry_body = encry_body);
        }

        body + "}"
    }

    pub fn to_json(&self) -> String {
        // let body: String = format!("{{\"aps\":{{\"interruption-level\":\"critical\",\"mutable-content\":1,\"alert\":{{\"title\":\"{title}\",\"body\":\"{body}\"}},\"category\":\"myNotificationCategory\",\"sound\":\"chime.caf\"}},\"icon\":\"{icon}\"}}",
        // title = self.title, body = self.body, icon= self.icon
        //     );
        self.json(None)
    }


    pub fn encrypt(&self) -> Result<String, Box<dyn std::error::Error>> {
        if self.enc_type.is_none() || self.mode.is_none() || self.key.is_none() {
            panic!("Encrypt type, mode, and key must be set");
        }

        let key: String = self.key.as_ref().unwrap().clone();

        let original: String = format!("{{\"body\":\"{}\"}}", self.body);
        let original: &[u8] = original.as_bytes();

        let cipher: Cipher = self.cipher.unwrap();
        
        let mut crypter: Crypter = Crypter::new(cipher, Mode::Encrypt, key.as_bytes(), Some(self.iv.as_ref().unwrap().as_bytes())).unwrap();
        crypter.pad(true); // Enable PKCS7 padding
        let mut buffer: Vec<u8> = vec![0; original.len() + cipher.block_size()];
        let count: usize = crypter.update(&original, &mut buffer).unwrap();
        let rest: usize = crypter.finalize(&mut buffer[count..]).unwrap();
        buffer.truncate(count + rest);
        Ok(self.json(Some(openssl::base64::encode_block(&buffer))))
    }

    pub fn serialize(&self) -> String {
        println!("json: {}", self.to_json());
        if self.cipher.is_some() {
            match self.encrypt() {
                Ok(encrypted) => {
                    println!("encrypted: {}", encrypted);
                    encrypted
                },
                Err(e) => panic!("Error encrypting message: {}", e),
             }
        } else {
            self.to_json()
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_json_all_field() {
        let mut msg = Msg::new("Test Title".to_string(), "Test Body".to_string());
        msg.set_level("timeSensitive".to_string());
        msg.set_badge(1);
        msg.set_auto_copy(1);
        msg.set_copy("Test Copy".to_string());
        msg.set_sound("chime.caf".to_string());
        msg.set_icon("icon.png".to_string());
        msg.set_group("Test Group".to_string());
        msg.set_is_archive(1);
        msg.set_url("https://example.com".to_string());
        let json = msg.to_json();
        println!("{}", json);
        assert_eq!(json, "{\"aps\":{\"mutable-content\":1,\"category\":\"myNotificationCategory\",\"interruption-level\":\"timeSensitive\",\"badge\":1,\"sound\":\"chime.caf\",\"thread-id\":\"Test Group\",\"alert\":{\"title\":\"Test Title\",\"body\":\"Test Body\"},\"icon\":\"icon.png\"},\"isArchive\":1,\"copy\":\"Test Copy\",\"url\":\"https://example.com\"}");
    }

    #[test]
    fn test_to_json_part_field() {
        let mut msg = Msg::new("Test Title".to_string(), "Test Body".to_string());
        msg.set_level("passive".to_string());
        msg.set_badge(1);
        msg.set_auto_copy(1);
        msg.set_copy("".to_string());
        msg.set_sound("chime.caf".to_string());
        msg.set_icon("icon.png".to_string());
        let json = msg.to_json();
        println!("{}", json);
        assert_eq!(json, "{\"aps\":{\"mutable-content\":1,\"category\":\"myNotificationCategory\",\"interruption-level\":\"passive\",\"badge\":1,\"sound\":\"chime.caf\",\"alert\":{\"title\":\"Test Title\",\"body\":\"Test Body\"},\"icon\":\"icon.png\"}}");
    }

    #[test]
    fn test_to_json_default() {
        let msg = Msg::new("Test Title".to_string(), "Test Body".to_string());
        let json = msg.to_json();
        println!("{}", json);
        assert_eq!(json, "{\"aps\":{\"mutable-content\":1,\"category\":\"myNotificationCategory\",\"interruption-level\":\"active\",\"alert\":{\"title\":\"Test Title\",\"body\":\"Test Body\"}}}");
    }
}
