extern crate tui;
extern crate termion;
extern crate rustc_serialize;

use std::net::TcpStream;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::{thread, time};
use std::string;
use std::vec::Vec;
use std::collections::BTreeMap;

use tui::{Terminal, TermionBackend};
use tui::widgets::{Widget, Block, border, List};
use tui::layout::{Group, Rect, Direction, Size};
use tui::terminal::Backend;
use tui::style::{Style, Color, Modifier};

use rustc_serialize::json::{Json};
use rustc_serialize::json;

use termion::event;

#[derive(RustcDecodable, RustcEncodable)]
 enum Message {
    Author{name : String, style : String},
    Mess{author : String, data : String},
}

struct Application {
    //First string : the message, second one : the user
    message_list : Vec<(String, String)>,
    user_color : BTreeMap<String, Style>,
    size : Rect,
}

impl Application {

    fn handleMessage(&mut self,mess : Message) {
        match mess {
        Message::Mess{author, data} => {self.message_list.push((author, data));},
        Message::Author{name, style} => {self.user_color.insert(name, matchStyle(style)).unwrap();},
        }
    }
    
    fn onMessageReceived(&mut self, mess : String) {
        let message : Message = json::decode(&*mess).unwrap();
        self.handleMessage(message);
    } 
}

fn matchStyle(style : String) -> Style {
    return Style::default().fg(Color::Yellow);
}
fn sendMessage(stream : &TcpStream, mess : Message) {
    let mut buffer = BufWriter::new(stream);
    buffer.write_all(&json::encode(&mess).unwrap().into_bytes()); 
}

fn draw(term : &mut Terminal<TermionBackend>, app : &Application) {
    let size = term.size().unwrap();
    let mut default_style =  Style::default();
    default_style.fg(Color::Yellow);
    Group::default().direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Fixed(10), Size::Max(20), Size::Min(10)])
        .render(term, &size, |term, chunks| {
                 List::default()
                     .block(Block::default() 
                            .borders(border::ALL)
                            .title("Messages"))
                     .items(&app.message_list.iter().map(|&(ref author,ref mess)| (format!("{} : {} ", author, mess), &default_style))
                            .collect::<Vec<(String, &Style)>>())
                     .render(term, &chunks[0]);
                });
    term.draw().unwrap();
}

fn main() {
    let mut backend = TermionBackend::new().unwrap();
    backend.clear();
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = Application{size : Rect::default(),
                            message_list : Vec::new(),
                            user_color : BTreeMap::new()}; 
    let stream = TcpStream::connect("127.0.0.1:8888").unwrap();
    let mut buffer = BufReader::new(&stream);
    let first_message = Message::Author{name : "Paul".to_string(), style : "Red".to_string()};
    sendMessage(buffer.get_ref(), first_message);
    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }
        draw(&mut terminal, &app); 
        let mut s = String::new();
        let data = match buffer.read_line(&mut s) {
            Ok(data) =>data,
            Err(e) => panic!("eroooor : {}", e),
        };
        if data > 0 {
            app.onMessageReceived(s);
        }
    }
}
