extern crate tui;
use std::net::TcpStream;
use std::io::{BufReader, BufRead};
use std::{thread, time};
use std::string;
use std::vec::Vec;
use tui::{Terminal, TermionBackend};
use tui::widgets::{Widget, Block, border, List};
use tui::layout::{Group, Rect, Direction, Size};
use tui::terminal::Backend;
use tui::style::{Style, Color, Modifier};

struct Application {
    
    //First string : the message, second one : the user
    message_list : Vec<(String, String)>,
    user_color : Vec<(String, Style)>,
    size : Rect,
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
                     .items(&app.message_list.iter().map(|&(ref mess,ref usr)| (format!("{} : {}", usr, mess), &default_style))
                            .collect::<Vec<(String, &Style)>>())
                     .render(term, &chunks[0]);
                });
    term.draw().unwrap();
}



fn main() {
    println!("Chat Observer");
    let mut backend = TermionBackend::new().unwrap();
    backend.clear();
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = Application{size : Rect::default(),
                            message_list : Vec::new(),
                            user_color : Vec::new()}; 
    let stream = TcpStream::connect("127.0.0.1:8888").unwrap();
    let mut buffer = BufReader::new(stream);
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
            println!("{}",s);
        }
    }
}
