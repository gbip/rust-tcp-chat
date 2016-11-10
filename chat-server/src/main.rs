use std::net::{TcpListener, TcpStream};
use std::vec::Vec;
use std::string::String;
use std::io::Write;

struct Application {
    clients : std::vec::Vec<TcpStream>,
}

impl Application {

    fn publish(&self, message : String) {

        for mut client in &self.clients {
        
            client.write_fmt(format_args!("{}",message)).expect("Internal error");

        }
    }

    fn add_client(&mut self, client : TcpStream) {
        
        self.clients.push(client);
        println!("New client connected");
        self.publish(String::from("A client has just connected"));
    }


}

fn main() {
    println!("Server chat");
    let mut app = Application{clients : Vec::new()};
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {app.add_client(stream);}
            Err(e) => println!("Connection refused from a client : {}", e),
        }
    }


}
