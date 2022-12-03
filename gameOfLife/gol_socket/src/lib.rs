use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use gol::Game;

pub fn register_socket(port:&str, debug:bool){
    let mut addr = "0.0.0.0:".to_owned();
    addr.push_str(port);
    let listener = TcpListener::bind(addr).unwrap();
    let mut game = Game::new(Some(50), Some(50));
    game.randomize();
    if debug{
        println!("Socket Ready");
    }

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &mut game, debug);
    }
}

pub fn handle_connection(mut stream: TcpStream,  game: &mut Game, debug: bool){
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    if debug{
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
    let mut response: String = String::new();

    let str= String::from_utf8_lossy(&buffer[..]);

    if str.contains("javascript.js"){
        if debug{
            println!("javascript.js");
        }
        let contents= fs::read_to_string("frontend/javascript.js").unwrap();
        response += "HTTP/1.1 200 OK\n\n";
        response += &contents;
        //println!("{}",response);
    }
    else if str.contains("mystyle.css"){
        if debug{
            println!("mystyle.css");
        }
        let contents = fs::read_to_string("frontend/mystyle.css").unwrap();
        response += "HTTP/1.1 200 OK\n\n";
        response += &contents;
    }
    else if str.contains("ChangeValue"){
        let start = str.find("id=").unwrap();
        let end = str.find("HTTP").unwrap();
        let result = &str[start+3..end-1];
        if debug{
            println!("Change Call for id");
            println!("{}", result);
        }
        let index : usize = result.parse().unwrap();
        game.change_value(index);
        let contents = game.serialize();
        response += "HTTP/1.1 200 OK\n\n";
        response += &contents;
    }
    else if str.contains("Advance"){
        if debug{
            println!("Advance Call");
        }
        game.tick();
        let contents = game.serialize();
        //println!("Recieved call for next generation of board");
        response += "HTTP/1.1 200 OK\n\n";
        response += &contents;

        //println!("{}",response);
    }
    else if str.contains("NewBoard"){
        if debug{
            println!("NewBoard/Resize Call");
        }
        response += "HTTP/1.1 200 OK\n\n";

        let heigth_start = str.find("height=").unwrap_or(50);
        let heigth_end = str.find("&").unwrap_or(str.len()-1);
        let height_result = &str[heigth_start..heigth_end];
        let height_result_after = height_result.split("=").last().unwrap();
        let u32_height: u32 = height_result_after.parse().unwrap();

        let width_start = str.find("width=").unwrap_or(50);
        let width_end = str.find("HTTP/1.1").unwrap_or(str.len())-1;
        let width_result = &str[width_start..width_end];
        let width_result_after = width_result.split("=").last().unwrap();
        let u32_width: u32 = width_result_after.parse().unwrap();

        if debug{
            println!("Height: {}", u32_height);
            println!("Width: {}", u32_width);
        }
        response += &game.change_size(u32_width, u32_height);

    }
    else if str.contains("ResetBoard"){
        response += "HTTP/1.1 200 OK\n\n";
        response += &game.negate_everything();
    }
    else if str.contains("Randomize"){
        game.randomize();
        let contents = game.serialize();
        //println!("Recieved call for next generation of board");
        response += "HTTP/1.1 200 OK\n\n";
        response += &contents;
    }
    else{
        let contents = fs::read_to_string("frontend/index.html").unwrap();

        response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
    }
    response += "\n";

    if debug{
        println!("{}", response);
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
