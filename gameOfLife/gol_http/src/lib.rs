use std::sync::Arc;
use std::sync::Mutex;
use rocket::*;

use gol::Game;

struct ManagedGame {
    game_mutex_arc: Arc<Mutex<Game>>,
}

#[get("/")]
fn index() -> rocket::response::content::Html<&'static str> {
    const DATA: &'static str = include_str!("../../frontend/index.html");
    rocket::response::content::Html(DATA)
}

#[get("/")]
fn javascript() -> rocket::response::content::JavaScript<&'static str> {
    const DATA: &'static str = include_str!("../../frontend/javascript.js");
    rocket::response::content::JavaScript(DATA)
}

#[get("/")]
fn mystyle() -> rocket::response::content::Css<&'static str> {
    const DATA: &'static str = include_str!("../../frontend/mystyle.css");
    rocket::response::content::Css(DATA)
}

#[get("/")]
fn advance(manager : &State<ManagedGame>) -> String{
    let str:String;
    {
        let a = manager.game_mutex_arc.clone();
        let mut game = a.lock().unwrap();
        (*game).tick();
        str = (*game).serialize();
    }
    str
}

#[get("/")]
fn randomize(manager : &State<ManagedGame>) -> String{
    let str:String;
    {
        let a = manager.game_mutex_arc.clone();
        let mut game = a.lock().unwrap();
        (*game).randomize();
        str = (*game).serialize();
    }
    str
}

#[get("/")]
fn reset_board(manager : &State<ManagedGame>) -> String{
    let str:String;
    {
        let a = manager.game_mutex_arc.clone();
        let mut game = a.lock().unwrap();
        str = (*game).negate_everything();
        //(*game).serialize();
    }
    str
}

#[get("/?<height>&<width>")]
fn new_board(manager : &State<ManagedGame>, height: u32, width:u32 ) -> String{
    let str:String;
    {
        let a = manager.game_mutex_arc.clone();
        let mut game = a.lock().unwrap();
        str = (*game).change_size(height, width);
    }
    str
}

#[get("/?<id>")]
fn change_value(manager : &State<ManagedGame>, id : usize ) -> String{
    let str:String;
    {
        let a = manager.game_mutex_arc.clone();
        let mut game = a.lock().unwrap();
        (*game).change_value(id);
        str = (*game).serialize();
    }
    str
}

#[rocket::main]
pub async fn rocket_setup() -> Result<(), rocket::Error> {
    let mut game = Game::new(Some(50), Some(50));
    game.randomize();

    let game_mutex: std::sync::Mutex<Game> = std::sync::Mutex::new(game);
    let game_arc : Arc<Mutex<Game>> = std::sync::Arc::new(game_mutex);
    let manager:ManagedGame = ManagedGame{ game_mutex_arc: game_arc };
    
    let config_dummy = rocket::Config::figment()
                    .merge(("address", "0.0.0.0"))
                    .merge(("port", 8000))
                    .merge(("keep_alive", 0)); // To get compareable results in Wireshark

    let config = rocket::Config::from(config_dummy);
    
    rocket::custom(config)
        .manage(manager)
        .mount("/", routes![index])
        .mount("/javascript.js", routes![javascript])
        .mount("/mystyle.css", routes![mystyle])
        .mount("/Advance", routes![advance])
        .mount("/ResetBoard", routes![reset_board])
        .mount("/NewBoard", routes![new_board])
        .mount("/ChangeValue", routes![change_value])
        .mount("/Randomize", routes![randomize])
        .launch()
        .await
}