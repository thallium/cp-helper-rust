use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    process::exit,
};

use config::Config;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Test {
    input: String,
    output: String,
}

fn filter_str(s: &String) -> String {
    s.chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c)
            } else if c.is_whitespace() {
                Some('_')
            } else {
                None
            }
        })
        .collect()
}

fn handle_request(req: &mut TcpStream, counter: &mut u32, path_prefix: &str) {
    let mut buffer = [0; 4096];
    if let Ok(len) = req.read(&mut buffer) {
        let req_str = std::str::from_utf8(&buffer[..len])
            .unwrap()
            .split('\n')
            .last()
            .unwrap();
        let problem: Value = serde_json::from_str(req_str).unwrap();
        let size: u32 = serde_json::from_value(problem["batch"]["size"].clone()).unwrap();
        if *counter == 0 {
            *counter = size
        }
        let group_str = serde_json::from_value::<String>(problem["group"].clone()).unwrap();
        let group = group_str.split(" - ").collect::<Vec<_>>();
        let judge = group[0];
        let mut name: String = serde_json::from_value(problem["name"].clone()).unwrap();
        println!("{name}");
        name = filter_str(&name);
        let tests = serde_json::from_value::<Vec<Test>>(problem["tests"].clone()).unwrap();
        let dir = if group.len() == 1 {
            Path::new(path_prefix).join(judge).join(name)
        } else {
            let category = filter_str(&group[1].into());
            Path::new(path_prefix).join(judge).join(category).join(name)
        };
        create_dir_all(dir.as_path()).ok();
        File::create(dir.join("main.cpp")).unwrap();
        for (i, test) in tests.into_iter().enumerate() {
            let i = i + 1;
            let mut input = File::create(dir.join(format!("{i}.in"))).unwrap();
            let mut output = File::create(dir.join(format!("{i}.out"))).unwrap();
            input.write_all(test.input.as_bytes()).ok();
            output.write_all(test.output.as_bytes()).ok();
        }
        *counter -= 1;
        if *counter == 0 {
            println!(
                "Test cases stored at:\n{dir}",
                dir = dir.parent().unwrap().to_str().unwrap()
            );
            exit(0);
        }
    }
}

fn main() {
    let mut builder = Config::builder();
    let config_files = [
        dirs::config_dir().unwrap().join("cp-helper/config.toml"),
        dirs::home_dir()
            .unwrap()
            .join(".config/cp-helper/config.toml"),
    ];
    for config_file in config_files {
        if config_file.exists() {
            builder = builder.add_source(config::File::from(config_file))
        }
    }
    let config = builder
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let listner = TcpListener::bind("127.0.0.1:1327").unwrap();
    let mut counter = 0;
    for stream in listner.incoming() {
        if let Ok(mut req) = stream {
            handle_request(
                &mut req,
                &mut counter,
                config.get("contest_path").unwrap_or(&String::from(".")),
            );
        }
    }
}
