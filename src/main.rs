use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, process::exit, fs::{File, create_dir_all}, path::Path};
use serde::{Deserialize, Serialize};

use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Test {
    input: String,
    output: String,
}

fn handle_request(req: &mut TcpStream, counter: &mut u32) {
    let mut buffer = [0; 4096];
    if let Ok(len) = req.read(&mut buffer) {
        let req_str = std::str::from_utf8(&buffer[..len]).unwrap().split('\n').last().unwrap();
        let problem: Value = serde_json::from_str(req_str).unwrap();
        let size: u32 = serde_json::from_value(problem["batch"]["size"].clone()).unwrap();
        if *counter == 0 {
            *counter = size
        }
        let group_str = serde_json::from_value::<String>(problem["group"].clone()).unwrap();
        let group = group_str.split(" - ").collect::<Vec<_>>();
        let judge = group[0];
        let name: String = serde_json::from_value(problem["name"].clone()).unwrap();
        println!("{name}");
        let tests = serde_json::from_value::<Vec<Test>>(problem["tests"].clone()).unwrap();
        let dir = if group.len() == 1 {
            format!("{judge}/{name}")
        } else {
            format!("{judge}/{category}/{name}", category = group[1])
        };
        let path = Path::new(&dir);
        create_dir_all(path).ok();
        for (i, test) in tests.into_iter().enumerate() {
            let i = i + 1;
            let mut input = File::create(format!("{dir}/{i}.in")).unwrap();
            let mut output = File::create(format!("{dir}/{i}.out")).unwrap();
            File::create(format!("{dir}/main.cpp")).unwrap();
            input.write_all(test.input.as_bytes()).ok();
            output.write_all(test.output.as_bytes()).ok();
        }
        *counter -= 1;
        if *counter == 0 {
            exit(0);
        }
    }
}

fn main() {
    let listner = TcpListener::bind("127.0.0.1:1327").unwrap();
    let mut counter = 0;
    for stream in listner.incoming() {
        if let Ok(mut req) = stream {
            handle_request(&mut req, &mut counter);
        }
    }
}
