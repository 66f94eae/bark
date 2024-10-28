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


use std::{thread, time::Duration};

use crate::{config, module::msg::Msg};


pub fn do_send(msg: &Msg, topic: &str, devices: Vec<String>, token: String) {
    
    let client: reqwest::blocking::Client = reqwest::blocking::ClientBuilder::new().http2_prior_knowledge().build().unwrap();
    let mut tasks: Vec<thread::JoinHandle<()>> = Vec::new();
    for device  in devices.into_iter() {
        let token: String = token.clone();
        let topic: String = String::from(topic);
        let body: String = msg.serialize();
        let client: reqwest::blocking::Client = client.clone();

        tasks.push(
            thread::spawn(move || {
                let resp: Result<reqwest::blocking::Response, reqwest::Error> = client
                    .post(format!("https://{host}/3/device/{device}", host = config::APNS_HOST, device = device))
                        .bearer_auth(token)
                        .header("apns-push-type", "alert")
                        .header("apns-topic", topic)
                        .body(body)
                        .send();
                match resp {
                    Ok(r) => {
                        if r.status() != reqwest::StatusCode::OK {
                            println!("send to {} failed: {:?}", device, r.text().unwrap());
                        }
                    }
                    Err(e) => {
                        println!("can not connect to apns server: {:?}", e);
                    }
                }
            })
        );
    }

    while !tasks.is_empty() {
        if let Some(task) = tasks.pop() {
            if !task.is_finished() {
                tasks.insert(0, task);
            }
        }
        thread::sleep(Duration::from_millis(2));
    }

}