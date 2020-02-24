use crate::signald::signaldrequest::SignaldRequestBuilder;
use crate::signald::signaldrequest::SignaldRequest;
use std::os::unix::net::UnixStream;
use std::io::Write;
use std::io::BufReader;
use std::io::prelude::*;

/**
 * Main signald class
 * Responsible for all the communication to the signald socket
 */
pub struct Signald {
    // The signald socket
    socket: UnixStream,
    // A request builder which is reused to limit memory allocation
    request_builder: SignaldRequestBuilder,
    // A count of all the sent messages on this socket
    message_count: u32,
}
impl Signald {
    /**
     * Connect the socket on @socket_path
     * @Returns a new Signald instance
     */
    pub fn connect(socket_path: String) -> Signald {
        // Connect the socket
        let socket = match UnixStream::connect(&socket_path) {
            Err(_) => panic!("Signald server is not running"),
            Ok(stream) => {
                println!("Connected to socket: {}", &socket_path);
                stream
            }
        };

        Signald {
            socket: socket,
            request_builder: SignaldRequestBuilder::new(),
            message_count: 0,
        }
    }
    /**
     * Send a request over the socket
     */
    pub fn send_request(&mut self, request: &SignaldRequest) {
        let formatted_request = request.to_string() + "\n";
        match self.socket.write_all(formatted_request.as_bytes()) {
            Err(_) => panic!("Failed to send message"),
            Ok(_) => println!("Message sent: {}", formatted_request.to_string()),
        }
        self.message_count += 1;
    }
    /**
     * DEBUGGING PURPOSES
     * Opens a never ending loop that prints all the signald responses.
     */
    pub fn read_requests(&self) {
        let stream = BufReader::new(&self.socket);
        for line in stream.lines() {
            println!("{}", line.unwrap());
        }
    }
    /**
     * Enable receiving user events like received messages
     */
    pub fn subscribe(&mut self, username: String) {
        self.request_builder.flush();
        self.request_builder.set_type("subscribe".to_string());
        self.request_builder.set_username(username);
        let request = self.request_builder.build();

        self.send_request(&request);
    }
    /**
     * Link an existing signal account
     */
    pub fn link(&mut self) {
        self.request_builder.flush();
        self.request_builder.set_type("link".to_string());
        let request = self.request_builder.build();
        
        self.send_request(&request);
    }
    /**
     * Get the current signal version
     */
    pub fn version(&mut self) {
        self.request_builder.flush();
        self.request_builder.set_type("version".to_string());
        let request = self.request_builder.build();
        
        self.send_request(&request);
    }
    /**
     * Query all the contacts
     */
    pub fn list_contacts(&mut self, username: String) {
        self.request_builder.flush();
        self.request_builder.set_type("list_contacts".to_string());
        self.request_builder.set_username(username);
        self.request_builder.set_id(self.message_count.to_string());
        let request = self.request_builder.build();
        
        self.send_request(&request);
    }
}
