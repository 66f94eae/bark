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


use std::collections::HashMap;

use chrono::Local;
use git2::Repository;

fn main() {
    let build_date: String = Local::now().format("%Y-%m-%d").to_string();
    let git_commit: String = get_git_commit();
    let os_param: HashMap<String, String> = compile_by_os();

    println!("-------- {} --------","start cargo set environment variables ");
    println!("cargo:rustc-env=GIT_COMMIT={}", git_commit);
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
    for (key, value) in os_param {
        println!("cargo:rustc-env={}={}", key, value);
    }
    println!("-------- {} --------","finish cargo set environment variables");
}

fn compile_by_os() -> HashMap<String, String> {
    let mut os_param: HashMap<String, String> = HashMap::<String,String>::new();
    if cfg!(target_os = "windows") {
        println!("-------- {} --------","detect windows platform");

        os_param.insert("RUN_FILE_BARK".to_string(), std::env::temp_dir().join("bark").to_str().unwrap().to_string());
    } else if cfg!(target_os = "macos") {
        println!("-------- {} --------","detect macos platform");

        os_param.insert("RUN_FILE_BARK".to_string(), "/usr/local/run/bark".to_string());
    } else if cfg!(target_os = "linux") {
        println!("-------- {} --------","detect linux platform");

        os_param.insert("RUN_FILE_BARK".to_string(), "/run/bark".to_string());
    } else {
        panic!("unsupported platform");
    }

    os_param
}

fn get_git_commit() -> String {
    let mut git_commit = "unknown".to_string();
    if let Ok(repository) = Repository::open(".") {
        let _ = repository.head().and_then(|head| {
            head.target().and_then(|oid| {
                let _ = repository.find_commit(oid).and_then(|c|{
                    git_commit = c.id().to_string()[..7].to_string();
                    Ok(())
                });
                Some(oid)
            });
        Ok(())
        });
    }
    git_commit
}