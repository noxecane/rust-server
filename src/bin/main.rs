extern crate server;

use server::http;
use server::resource;
use server::http::Status;
use server::fast::Pool;

use std::thread;
use std::time::Duration;

fn main() {
    let (address, listener) = http::connect(8080).unwrap();
    let pool = Pool::new(4);
    println!("Listening on {}", address);

    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();
        pool.execute(move || {
            let request = http::read_bytes(&stream).unwrap();
            let response = on_connect(request);
            http::write_string(response, &stream).unwrap();
        });
    }
}

fn on_connect(buffer: [u8; 512]) -> String {
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        http::response(Status::ok(), resource::load_file("index.html"))
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        http::response(Status::ok(), resource::load_file("index.html"))
    } else {
        http::response(Status::not_found(), resource::load_file("404.html"))
    }
}

