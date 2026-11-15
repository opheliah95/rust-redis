#![allow(unused_imports)]
use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::borrow::Cow;
use regex::Regex;
use regex::RegexSet;



use tokio::io::split;

#[tokio::main]
async fn main(){
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment the code below to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");

                thread::spawn(|| {handle_client(_stream);});
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

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

        resp_handler(incoming);
    }
   
    Ok(())
}


fn resp_handler(cow: Cow<'_, str>) {
    let re_array_start = Regex::new(r"\*(?<len>\d)\\r\\n").unwrap(); // find the len
    let re_array_content = Regex::new(r"$(?<len>\d)\\r\\n").unwrap();
    let Some(arr_len) = re_array_start.captures(&cow) else {
        eprintln!("{} not an array", &cow);
        return;
    };

        eprintln!("the len is {}", &arr_len["len"]);

     let Some(contents) = re_array_content.captures(&cow) else {
        eprintln!("{:?} not not have any val", &cow);
        return;
    };
    eprintln!("the content is {}", &contents["txt"]);
}

fn handle_client(stream: TcpStream) {
    print_stream_content(stream);
}
