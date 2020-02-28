use tokio::net::UnixStream;
use crate::signald::signaldrequest::SignaldRequest;
use std::io::BufReader;
use std::io::Write;
use std::io::prelude::*;
use std::thread;
use std::sync::mpsc;

pub struct SignaldSocket {
    socket: UnixStream,
}
impl SignaldSocket {
    pub async fn new(socket_path: String) -> SignaldSocket {
        // Create two sockets
        let socket = match UnixStream::connect(&socket_path) {
            Err(_) => panic!("Signald server is not running"),
            Ok(stream) => {
                println!("Connected to socket: {}", &socket_path);
                stream
            }
        };
        //let socket_clone = socket.try_clone().expect("Couldn't clone socket");

        //let (tx, rx) = mpsc::channel();
        //thread::spawn(move || {
            //let stream = BufReader::new(socket);
            //for line in stream.lines() {
                //tx.send(line).unwrap();
            //}
            //println!("Reader exited");
        //});
        //let received = rx.recv().unwrap();
        //println!("{:?}", received);

        SignaldSocket {
            socket: socket
        }
    }

    /**
     * Send a request over the socket
     */
    pub fn send_request(&mut self, request: &SignaldRequest) {
        let formatted_request = request.to_string() + "\n";
        match self.writer.write_all(formatted_request.as_bytes()) {
            Err(_) => panic!("Failed to send message"),
            Ok(_) => println!("Message sent: {}", formatted_request.to_string()),
        }
    }

    pub fn sync(&mut self) {
        //match self.reader.fill_buf() {
            //Err(_) => {},
            //Ok(s) => {
                //println!("{:?}", str::from_utf8(s));
                //self.rea
            //},
        //}

    }
}
