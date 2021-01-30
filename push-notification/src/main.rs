extern crate ws;

use std::{thread, time};
use ws::{Handler, Sender, WebSocket};

struct Server {
    out: Sender,
}

// Implement Handler and use all the default implemention
impl Handler for Server {}

fn main() {
    let server = WebSocket::new(|out| Server { out }).unwrap();
    let broadcaster = server.broadcaster();

    let periodic = thread::spawn(move || loop {
        broadcaster.send("Meow!").unwrap();
        thread::sleep(time::Duration::from_secs(1));
    });

    server.listen("127.0.0.1:8080").unwrap();
    // Block on the periodic thread to avoid to exit rigth away
    periodic.join().unwrap();
}
