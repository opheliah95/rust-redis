#![allow(unused_imports)]
use regex::Regex;
use regex::RegexSet;
use std::borrow::Cow;
use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::thread;

use tokio::io::split;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment the code below to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");

                thread::spawn(|| {
                    handle_client(_stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn print_stream_content(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            break;
        }

        let incoming = String::from_utf8_lossy(&buffer[..bytes_read]);

        resp_handler(incoming, &mut stream);
    }
}

fn resp_handler(cow: Cow<'_, str>, mut stream: &TcpStream) {
    let re_array_start = Regex::new(r"\*(?<len>\d)\r\n").unwrap(); // find the len
    let re_array_content = Regex::new(r"\$(?<len>\d)\r\n(?<txt>(.*))\r\n").unwrap();
    let Some(arr_len) = re_array_start.captures(&cow) else {
        eprintln!("{} not an array", &cow);
        return;
    };

    eprintln!("the len is {}", &arr_len["len"]);
    let captures: Vec<&str> = re_array_content
        .captures_iter(&cow)
        .map(|c| c.name("txt").unwrap().as_str())
        .collect();

    eprintln!("captured array: {:?}", captures);
    match captures.as_slice() {
        ["ECHO", ..] => {
            let args = &captures[1..];
            if args.len() == 1 {
                let content = args[0];
                let arg_len = content.len();
                let output = format!("${arg_len}\r\n{content}\r\n");
                stream.write_all(output.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
        ["PING", ..] => {
            stream.write_all(b"+PONG\r\n").unwrap();
            stream.flush().unwrap();
        }
        _ => {}
    }
    // let res: Vec<(&str, &str)> = re_array_content
    //     .captures_iter(&cow)
    //     .map(|caps| {
    //         let len = caps.name("len").unwrap().as_str();
    //         let txt = caps.name("txt").unwrap().as_str();
    //         (len, txt)
    //     })
    //     .collect();

    // for (len, item) in res {
    //     eprintln!("{item} has len of {len}");
    // }
}

fn handle_client(stream: TcpStream) {
    print_stream_content(stream);
}
