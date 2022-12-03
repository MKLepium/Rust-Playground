use std::thread;

mod lib;

fn main() {
    let socket_handler = thread::spawn(|| {
        lib::register_socket("8001", false);
    });
    socket_handler.join().expect("Error joining Socket Thread");

}
