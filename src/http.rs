use std::io::Result as IOResult;
use std::io::prelude::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct Status {
    code: u32,
    reason: String,
}

impl Status {
    pub fn new(code: u32, reason: &str) -> Status {
        Status {
            code,
            reason: String::from(reason),
        }
    }

    pub fn ok() -> Status {
        Status::new(200, "Ok")
    }

    pub fn not_found() -> Status {
        Status::new(404, "Not Found")
    }
}

pub fn response(status: Status, content: String) -> String {
    format!(
        "HTTP/1.1 {} {} \r\n\r\n{}",
        status.code,
        status.reason,
        content
    )
}

pub fn connect(port: u32) -> IOResult<(String, TcpListener)> {
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address)?;
    Ok((address, listener))
}

pub fn read_bytes(mut stream: &TcpStream) -> IOResult<[u8; 512]> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    Ok(buffer)
}

pub fn write_string(data: String, mut stream: &TcpStream) -> IOResult<()> {
    stream.write(data.as_bytes())?;
    stream.flush()?;
    Ok(())
}
