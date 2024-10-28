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

use openssl::pkey::PKey;

use crate::module::msg::Msg;


pub trait Sender {
    fn run_file(&self) -> &str;
    fn auth_key_id(&self) -> &str;
    fn team_id(&self) -> &str;
    fn key(&self) -> &[u8];
    fn msg(&self) -> &Msg;
    fn topic(&self) -> &str;

    
    fn send(&self, devices: Vec<String>) {
        crate::apns::do_send(self.msg(), self.topic(), devices, self.get_token());
    }

    fn get_token(&self) -> String {
        let now: u64 = std::time::SystemTime::UNIX_EPOCH.elapsed().expect("get timestamp failed").as_secs();

        match std::fs::read_to_string(self.run_file()) {
            Ok(token) => {
                if token.split(" ").nth(0).or_else(|| Some("0")).is_some_and(|time| now - time.parse::<u64>().expect("parse file error") < 2700) {
                    // token is valid
                    match token.split(" ").nth(1) {
                        Some(token) => return token.to_string(),
                        None => return self.gen_token_and_persist(now)
                    }
                }
                return self.gen_token_and_persist(now);

            },
            Err(_) => {
                return self.gen_token_and_persist(now);
            }
        }
    }
    fn gen_token_and_persist(&self, time_stamp: u64) -> String{
        let jwt_header: String = Self::clean_str(
            openssl::base64::encode_block(
                format!("{{ \"alg\": \"ES256\", \"kid\": \"{}\" }}", self.auth_key_id())
                .as_bytes()
            )
        );

        let jwt_claims: String = Self::clean_str(
            openssl::base64::encode_block(
                format!("{{ \"iss\": \"{}\", \"iat\": {} }}", 
                        self.team_id(), time_stamp 
                        )
                .as_bytes()
            )
        );

        let mut singer: openssl::sign::Signer<'_> = openssl::sign::Signer::new(
                                openssl::hash::MessageDigest::sha256(),
                                &PKey::from_ec_key(
                                                openssl::ec::EcKey::private_key_from_pem(self.key()).expect("init key data failed")
                                            ).expect("generate private key failed")
                                ).expect("init signer failed");

        let jwt_header: String = format!("{}.{}", jwt_header, jwt_claims);
        singer.update(jwt_header.as_bytes()).expect("fill sign data failed");

        let sign: Vec<u8> = singer.sign_to_vec().expect("sign failed");

        let jwt_signature: String = Self::clean_str(openssl::base64::encode_block(&sign));
        let token: String= format!("{}.{}", jwt_header, jwt_signature);

        std::fs::write(self.run_file(), format!("{} {}",time_stamp, token).as_bytes()).expect("write token failed");
        token
    }

    fn clean_str(str: String) -> String {
        str.replace("+", "-")
            .replace("/", "_")
            .replace("=", "")
    }
}