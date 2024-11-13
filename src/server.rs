use std::error::Error;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Read, Write};

use crate::request::Request;
use crate::response::Response;


#[derive(Debug)]
pub struct Server {
    ip: String,
}

#[allow(dead_code)]
impl Server {
    pub fn new(ip: String) -> Server { Server{ip} }

    pub fn start(&self) -> Result<TcpListener, Box<dyn Error>> {
        Ok(TcpListener::bind(&self.ip).unwrap())
    }

    pub fn handle_request(mut stream: TcpStream) {
        let content: String = Self::parse_request(&stream);
        let response: Response = Request::validate(content);
        stream.write_all(response.to_string().as_bytes()).unwrap();
    }

    fn parse_request(stream: &TcpStream) -> String {
        let mut reader = BufReader::new(stream);
        let buf = reader.fill_buf().unwrap();

        let mut content: Vec<u8> = Vec::new();
        let mut content_reader = BufReader::new(buf);
        content_reader.read_to_end(&mut content).unwrap();

        let content: String = String::from_utf8(content).unwrap();
        content
    }
}
// TODO -> document the handle_request function