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


use std::io::Write;
use cmd::CMD;
use sender::Sender;

mod config;
mod module;
mod cmd;
mod util;
mod sender;

fn main() {
    let cmd: CMD = CMD::parse();

    let msg: bark_dev::msg::Msg = cmd.to_msg();
    
    if let Some(delay) = cmd.delay {
        count_down(delay);
    }

    let mut send: Sender = Sender::new(config::RUN_FILE_BARK.to_string());

    send.send(&msg, &cmd.receiver);
}

// show count down in terminal
fn count_down(delay: u64) {
    let mut stdout: std::io::Stdout = std::io::stdout();
    for i in (0..=delay).rev() {
        print!("\rAfter {} seconds, the message will be sent", i);
        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        if i != 0 {
            stdout.write_all(b"\r").unwrap();
            stdout.write_all(b"\x1b[K").unwrap();
            stdout.flush().unwrap();
        }
    }
}
