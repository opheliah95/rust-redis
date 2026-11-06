#![allow(unused_imports)]
use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};

use tokio::io::split;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment the code below to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");

                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn write_to_stream(mut stream: TcpStream) {
    let mut writer: BufWriter<&TcpStream> = BufWriter::new(&stream);
}
fn print_stream_content(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        let incoming = String::from_utf8_lossy(&buffer[..bytes_read]);

        // Basic check for PING command anywhere in the incoming RESP payload
        if incoming.to_uppercase().contains("PING") {
            stream.write_all(b"+PONG\r\n")?;
            stream.flush()?;
        }
    }
    // let spilt_args: Vec<&str> = msg.trim().split_whitespace().collect();
    // // io::stdout().flush().expect("Failed to flush stdout");
    // for (idx, arg) in spilt_args.iter().enumerate() {
    //     eprintln!("idx={idx} arg={arg}, is_ping={}", *arg == "PING");
    //     if *arg == "*1\r\n$4\r\nPING\r\n" {
    //         //eprintln!("=== ping success ===");
    //         stream.write_all(b"+PONG\r\n").unwrap();
    //         //print!("+PONG\r\n");
    //         stream.flush()?;
    //     }
    // }
    Ok(())
}
fn handle_client(stream: TcpStream) {
    print_stream_content(stream);
}
