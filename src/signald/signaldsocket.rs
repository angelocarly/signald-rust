use crate::signald::signaldrequest::SignaldRequest;
use std::sync::mpsc;
use std::os::unix::net::UnixStream;
use std::thread;
use std::io::{BufReader, BufRead};
use std::sync::mpsc::Receiver;

pub trait SignaldEvents {
    fn on_connect(&self, path: &str) {}
    fn on_message(&self, mesg: &str) {}
    fn on_response(&self, mesg: &str) {}
}

pub struct SignaldSocket {
    socket_path: String,
    socket: Option<UnixStream>,
    rx: Option<Receiver<String>>,
    hooks: Vec<Box<SignaldEvents>>
}
impl SignaldSocket {
    pub fn new(socket_path: String) -> Self {
        Self {
            socket_path: socket_path,
            socket: None,
            rx: None,
            hooks: Vec::new()
        }
    }

    pub fn add_event_hook<E: SignaldEvents + 'static>(&mut self, hook: E) {
        self.hooks.push(Box::new(hook));
    }

    pub fn connect(&mut self) {

        let socket = match UnixStream::connect(self.socket_path.to_string()){
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

        // Create a thread for the reader to use
        let (tx, rx) = mpsc::channel::<String>();
        self.rx = Some(rx);
        thread::spawn(move || {
            let reader = BufReader::new(socket);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        tx.send(l);
                    },
                    Err(_) => {

                    }
                }
            }
        });
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
        let iter = self.rx.as_ref().unwrap();
        for i in iter {
            for hook in &self.hooks {
                hook.on_message(&i);
            }
        }
    }
}
