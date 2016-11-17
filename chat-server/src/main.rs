extern crate rustc_serialize;

use std::net::{TcpListener, TcpStream};
use std::vec::Vec;
use std::string::String;
use std::io::{Write,BufWriter, BufReader, BufRead};
use std::thread;
use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;


use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
 enum Message {
        Author{name : String, style : String},
        Mess{author : String, data : String},
 }

struct Client {
    thread : std::thread::Thread,
    stream : TcpStream,
}

impl Client {
    fn send_message(self, message : Message) {
        let mut buffer = BufWriter::new(&self.stream);
        buffer.write_all(&json::encode(&message).unwrap().into_bytes());
        buffer.flush().expect("Error while writing to TCP");
    }
}

struct Application {
    clients : std::vec::Vec<TcpStream>,
    listeners : std::vec::Vec<std::thread::JoinHandle<()>>,
    receiver : Receiver<String>,
    sender : Sender<String>,
}

fn startListening(stream : Receiver<TcpStream>, sender : Sender<String>) {
    let client = stream.recv().expect("Error TcpStream received invalid");
    let mut buffer = BufReader::new(client);
    loop {
        let mut s = String::new();
        let data = match buffer.read_line(&mut s) {
            Ok(data) =>data,
            Err(e) => panic!("eroooor : {}", e),
        };
        if data > 0 {
            sender.send(s);
        }
    }
}
 

impl Application {

    fn publish(&self, message : Message) {
        for client in &self.clients {
            let mut buffer = BufWriter::new(client);
            buffer.write_all(&json::encode(&message).unwrap().into_bytes());
            //writeln!(buffer, "{}", message).unwrap();
            buffer.flush().expect("Error while writing to TCP");
        }
    }

    fn addClient(&mut self, client : TcpStream) {
        self.clients.push(client);
        println!("New client connected");
        self.publish(Message::Mess{author : "Server".to_string(), data : "A client has just connected".to_string()});
        let stream_clone = self.clients.last().unwrap().try_clone().unwrap();
        let (send, rec) = mpsc::channel();
        let sender = self.sender.clone();
        self.listeners.push(thread::spawn(move || startListening(rec, sender)));
        send.send(stream_clone);
    }
    
    fn onMessageReceived(& self , mess : String){
        let message : Message = json::decode(&*mess).unwrap();
        self.publish(message);
    }

}


fn main() {
    println!("Server chat");
    let (send, rec) : (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut app = Application{clients :Vec::new(), listeners : Vec::new(), receiver : rec, sender : send};
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {app.addClient(stream);}
            Err(e) => println!("Connection refused from a client : {}", e),
        }
    }


}
