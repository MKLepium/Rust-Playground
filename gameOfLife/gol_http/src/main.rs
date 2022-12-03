#![feature(proc_macro_hygiene, decl_macro)]

use std::thread;

mod lib;

fn main() {
    let http_handler = thread::spawn(|| {
        let x = lib::rocket_setup();
        x.expect("Rocket Server Blew");
    });
    http_handler.join().expect("Error joining http thread");
}
