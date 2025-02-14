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
#[derive(Serialize, Deserialize, Clone)]
pub struct Token {
    refresh_at: u64,
    token: String
}

impl Token {
    pub fn new(refresh_at: u64, token: &str) -> Token {
        Token {
            refresh_at: refresh_at,
            token: token.to_string()
        }
    }

    pub fn get_token(&self) -> &str {
       &self.token
    }

    pub fn get_refresh_at(&self) -> u64 {
       self.refresh_at
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    impl Token {
        pub fn new_for_test(refresh_at: u64,token: &str) -> Token {
            Token {
                refresh_at: refresh_at,
                token: token.to_string()
           }
        }
    }

    #[test]
    fn test_token() {
        let token = Token {
            refresh_at: 0,
            token: "token".to_string()
        };

        let r = toml::to_string_pretty(&token);
        assert_eq!("refresh_at = 0\ntoken = \"token\"\n", r.unwrap());
    }
}