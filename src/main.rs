use std::io::Write;
// Uncomment this block to pass the first stage
use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                stream.write(pong_response().as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn pong_response() -> &'static str {
    return "+PONG\r\n";
}
