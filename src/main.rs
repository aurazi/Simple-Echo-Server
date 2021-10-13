use std::net::{Shutdown, TcpStream, TcpListener};
use std::io::{Write,Read};
use std::thread::{spawn};
use std::string::String;

const BUFFER_SIZE: usize = 64;
const ECHO_REPLY: &'static [u8] = b"_reply";
const ADDRESS: &'static str = "127.0.0.1:8080";

fn handle_stream(mut stream: TcpStream) {
    let mut read_buffer = [0 as u8; BUFFER_SIZE];
    match stream.read(&mut read_buffer) {
        Ok(bytes_read) => {
            if bytes_read > BUFFER_SIZE {
                panic!("Stream is too large");
            } else if bytes_read == 0 {
                panic!("Connection disconnected");
            } else {
                let slice = [&read_buffer[0..bytes_read], &ECHO_REPLY].concat();
                stream.write(&slice[..]).unwrap();
                stream.flush().unwrap();
            }
        },
        Err(err) => {
            stream.shutdown(Shutdown::Both)
                            .expect("Shutdown failed");
            panic!("{}",err);
        }
    }
}

fn client_send(message: &'static [u8]) {
    spawn(move ||{
        loop {
            let mut stream = TcpStream::connect(ADDRESS).unwrap();
            stream.write(&message).unwrap();
            stream.flush().unwrap();

            let mut read_buffer = [0 as u8; BUFFER_SIZE];
            stream.read(&mut read_buffer).unwrap();

            let string = String::from_utf8_lossy(&read_buffer);
            println!("{}",string);
        }}
    );
}

fn main() {
    client_send(b"hi there!");

    let listener = TcpListener::bind(ADDRESS).unwrap();
    for rstream in listener.incoming() {
        match rstream {
            Ok(stream) => {
                spawn(move || {
                    handle_stream(stream);
                });
            }
            Err(err) => {
                panic!("Stream unwrap failed: {}", err);
            }
        }
    }
}