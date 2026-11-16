#![allow(unused_imports)]
use codecrafters_redis::Data;
use regex::Regex;
use regex::RegexSet;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
type KeyVal = Arc<Mutex<HashMap<String, Data>>>;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // a heapmap to store data
    let mut key_val: KeyVal = Arc::new(Mutex::new(HashMap::new()));

    //tcp listener
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(_stream) => {
                    println!("accepted new connection");
                    let value = key_val.clone();

                    thread::spawn(move || {
                        handle_client(_stream, &value);
                    });
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }
}

fn print_stream_content(mut stream: TcpStream, key_val: &KeyVal) {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            return;
        }

        let incoming = String::from_utf8_lossy(&buffer[..bytes_read]);

        resp_handler(incoming, &mut stream, &key_val);
    }
}

fn handle_client(stream: TcpStream, key_val: &KeyVal) {
    print_stream_content(stream, &key_val);
}

fn write_output_to_stream(mut stream: &TcpStream, output: &str) {
    stream.write_all(output.as_bytes()).unwrap();
    stream.flush().unwrap();
    return;
}

fn format_bulk_string(input: &str) -> String {
    let arg_len = input.len();
    let output = format!("${arg_len}\r\n{input}\r\n");
    return output;
}

fn resp_handler(cow: Cow<'_, str>, stream: &TcpStream, key_val: &KeyVal) {
    let null_string = "$-1\r\n";
    let re_array_start = Regex::new(r"\*(?<len>\d)\r\n").unwrap(); // find the len
    let re_array_content = Regex::new(r"\$(?<len>\d+)\r\n(?<txt>(.*))\r\n").unwrap();
    let Some(arr_len) = re_array_start.captures(&cow) else {
        eprintln!("{} not an array", &cow);
        return;
    };

    //eprintln!("the array len is {}", &arr_len["len"]);
    let captures: Vec<&str> = re_array_content
        .captures_iter(&cow)
        .map(|c| c.name("txt").unwrap().as_str())
        .collect();

    eprintln!("captured array: {:?}", captures);
    match captures.as_slice() {
        ["ECHO", content] => {
            //let args = &captures[1..];
            if captures.len() == 2 {
                let output = format_bulk_string(content);
                write_output_to_stream(stream, &output);
            }
        }
        ["SET", ..] => {
            let key = captures[1].to_string();
            let val = captures[2].to_string();
            let mut px: u64 = 0;
            let mut ex: u64 = 0;

            if captures.len() > 3 {
                if captures.contains(&"PX") {
                    px = captures[4].parse().unwrap();
                } else if captures.contains(&"EX") {
                    ex = captures[4].parse().unwrap();
                }

                eprintln!("ex is {ex} in sec and px is {px} in mil")
            }
            {
                let mut map = key_val.lock().unwrap();
                // eprintln!("SET {key} = {val}");
                // init new field and start timer
                let mut data = Data::new(val, ex, px);
                data.init_timer();
                map.insert(key, data);
                write_output_to_stream(stream, "+OK\r\n");
            }
        }
        ["GET", ..] => {
            let key = captures[1];
            {
                {
                    let mut map = key_val.lock().unwrap();
                    eprintln!("all keys in map {:?} vs the key ={key}", map.keys());
                    match map.get_mut(key) {
                        Some(data) => {
                            data.check_timer(); // see if key expired
                            if &data.value == null_string {
                                write_output_to_stream(stream, null_string);
                                return;
                            }
                            let output = format_bulk_string(&data.value);
                            write_output_to_stream(stream, &output);
                        }
                        None => {
                            write_output_to_stream(stream, null_string);
                        }
                    }
                }
            }
        }
        ["PING", ..] => {
            write_output_to_stream(stream, "+PONG\r\n");
        }
        _ => {}
    }
}
