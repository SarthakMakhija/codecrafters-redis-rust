use std::io::{Read, Write};
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                respond_to(stream);
            }
            Err(err) => {
                println!("error: {}", err);
            }
        }
    }
}

fn respond_to(mut stream: TcpStream) {
    thread::spawn(move || {
        loop {
            let mut content = [0u8; 14];
            let result = stream.read(&mut content);
            match result {
                Ok(_) => {
                    stream.write_all(pong_response().as_bytes()).unwrap();
                }
                Err(_err) => {
                    break;
                }
            }
        }
    });
}

fn pong_response() -> &'static str {
    return "+PONG\r\n";
}
