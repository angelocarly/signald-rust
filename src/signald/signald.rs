use crate::signald::signaldrequest::SignaldRequestBuilder;
use crate::signald::signaldrequest::SignaldRequest;
use std::os::unix::net::UnixStream;
use std::io::Write;
use std::io::BufReader;
use std::io::prelude::*;

pub struct Signald {
    socket: UnixStream,
    requestBuilder: SignaldRequestBuilder,
    message_count: u32,
    // socket_path: String,
}
impl Signald {
    pub fn connect(socket_path: String) -> Signald {
        // Connect to socket
        let socket = match UnixStream::connect(&socket_path) {
            Err(_) => panic!("Signald server is not running"),
            Ok(stream) => {
                println!("Connected to socket: {}", &socket_path);
                stream
            }
        };

        Signald {
            socket: socket,
            requestBuilder: SignaldRequestBuilder::new(),
            message_count: 0,
            // socket_path: socket_path,
        }
    }
    pub fn send_request(&mut self, request: &SignaldRequest) {
        let formatted_request = request.to_string() + "\n";
        match self.socket.write_all(formatted_request.as_bytes()) {
            Err(_) => panic!("Failed to send message"),
            Ok(_) => println!("Message sent: {}", formatted_request.to_string()),
        }
        self.message_count += 1;
    }
    pub fn read_requests(&self) {
        let stream = BufReader::new(&self.socket);
        for line in stream.lines() {
            println!("{}", line.unwrap());
        }
    }
    pub fn subscribe(&mut self, username: String) {
        self.requestBuilder.flush();
        self.requestBuilder.set_type("subscribe".to_string());
        self.requestBuilder.set_username(username);
        let request = self.requestBuilder.build();

        self.send_request(&request);
    }
    pub fn link(&mut self) {
        self.requestBuilder.flush();
        self.requestBuilder.set_type("link".to_string());
        let request = self.requestBuilder.build();
        
        self.send_request(&request);
    }
    pub fn version(&mut self) {
        self.requestBuilder.flush();
        self.requestBuilder.set_type("version".to_string());
        let request = self.requestBuilder.build();
        
        self.send_request(&request);
    }
    pub fn list_contacts(&mut self, username: String) {
        self.requestBuilder.flush();
        self.requestBuilder.set_type("list_contacts".to_string());
        self.requestBuilder.set_username(username);
        self.requestBuilder.set_id(self.message_count.to_string());
        let request = self.requestBuilder.build();
        
        self.send_request(&request);
    }
}
