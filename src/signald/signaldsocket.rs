use crate::signald::signaldrequest::SignaldRequest;
//use tokio::net::UnixStream;
//use tokio::prelude::*;
use async_std::os::unix::net::UnixStream;
use async_std::io::BufReader;
use async_std::fs::File;
use async_std::prelude::*;
//use std::io::BufReader;
//use std::io::Write;
//use std::thread;
//use std::sync::mpsc;

pub trait SignaldEvents {
    fn on_connect(&self, path: &str) {}
    fn on_message(&self, mesg: &str) {}
    fn on_response(&self, mesg: &str) {}
}

pub struct SignaldSocket {
    socket_path: String,
    socket: Option<UnixStream>,
    hooks: Vec<Box<SignaldEvents>>
}
impl SignaldSocket {
    pub async fn new(socket_path: String) -> Self {
        Self {
            socket_path: socket_path,
            socket: None,
            hooks: Vec::new()
        }
    }

    pub fn add_event_hook<E: SignaldEvents + 'static>(&mut self, hook: E) {
        self.hooks.push(Box::new(hook));
    }

    pub async fn connect(&mut self) {

        let socket = match UnixStream::connect(self.socket_path.to_string()).await {
            Ok(stream) => {
                for hook in &self.hooks {
                    hook.on_connect(self.socket_path.as_str());
                }
                stream
            }
            Err(err) => {
                panic!("AAAA");
            }
        };

        let reader = async_std::io::BufReader::new(&socket);
        let mut lines = reader.lines();
        for line in lines.next().await {
            match line {
                Ok(l) => {
                    for hook in &self.hooks {
                        hook.on_message(&l);
                    }
                },
                Err(_) => {

                }
            }
        }
    }

    /**
     * Send a request over the socket
     */
    pub fn send_request(&mut self, request: &SignaldRequest) {
        let formatted_request = request.to_string() + "\n";
        // match self.socket.write_all(formatted_request.as_bytes()) {
        //     Err(_) => panic!("Failed to send message"),
        //     Ok(_) => println!("Message sent: {}", formatted_request.to_string()),
        // }
    }

    pub fn sync(&mut self) {
        // self.socket.poll_read();
        //match self.reader.fill_buf() {
            //Err(_) => {},
            //Ok(s) => {
                //println!("{:?}", str::from_utf8(s));
                //self.rea
            //},
        //}

    }
}
