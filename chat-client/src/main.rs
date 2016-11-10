use std::net::TcpStream;
use std::io::{BufReader, BufRead};
use std::{thread, time};

fn main() {
    println!("Chat Observer");
    let stream = TcpStream::connect("127.0.0.1:8888").unwrap();
    let mut buffer = BufReader::new(stream);
    loop {
        let mut s = String::new();
        let data = match buffer.read_line(&mut s) {
            Ok(data) =>data,
            Err(e) => panic!("eroooor : {}", e),
        };
        if data > 0 {
            println!("{}",s);
        }
        thread::sleep(time::Duration::from_millis(10));
    }
}
