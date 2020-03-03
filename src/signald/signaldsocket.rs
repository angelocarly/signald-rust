use crate::signald::signaldrequest::SignaldRequest;
use std::sync::mpsc;
use std::os::unix::net::UnixStream;
use std::thread;
use std::io::{Write, BufReader, BufRead};
use std::sync::mpsc::Receiver;
use futures::AsyncWriteExt;

pub trait SignaldEvents {
    fn on_connect(&self, path: &str) {}
    fn on_message(&self, mesg: &str) {}
    fn on_response(&self, mesg: &str) {}
    fn on_sent(&self, mesg: &str) {}
}

pub struct SignaldSocket {
    socket_path: String,
    socket: Option<UnixStream>,
    reader_receiver: Option<Receiver<String>>,
    hooks: Vec<Box<SignaldEvents>>
}
impl SignaldSocket {
    pub fn new(socket_path: String) -> Self {
        Self {
            socket_path: socket_path,
            socket: None,
            reader_receiver: None,
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
        self.socket = Some(socket.try_clone().unwrap());

        // Create a thread for the reader to use
        let (tx, rx) = mpsc::channel::<String>();
        self.reader_receiver = Some(rx);
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
        match self.socket.as_ref().unwrap().write_all(formatted_request.as_bytes()) {
            Err(_) => panic!("Failed to send message"),
            Ok(_) => {
                for hook in &self.hooks {
                    hook.on_sent(&request.to_string());
                }
            }
        }
    }

    // Read all requests that have come in
    pub fn sync(&mut self) {
        let iter = self.reader_receiver.as_ref().unwrap().try_iter();
        for i in iter {
            for hook in &self.hooks {
                hook.on_message(&i);
            }
        }
    }
}
