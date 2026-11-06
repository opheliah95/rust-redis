#![allow(unused_imports)]
use std::io::{self, Write};
use std::io::{BufRead, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};

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
fn print_stream_content(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut msg = String::new();
    reader.read_line(&mut msg).expect("Cannot retreive message");
    // print!("redis> {msg}");
    // io::stdout().flush().expect("Failed to flush stdout");
    print!("PONG\n");
    io::stdout().flush().expect("Failed to flush stdout");
}
fn handle_client(stream: TcpStream) {
    print_stream_content(stream);
}
