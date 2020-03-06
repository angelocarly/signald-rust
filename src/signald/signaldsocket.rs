use crate::signald::signaldrequest::SignaldRequest;
use std::sync::{mpsc, Arc, Mutex};
use std::os::unix::net::UnixStream;
use std::thread;
use std::io::{Write, BufReader, BufRead};
use std::sync::mpsc::{Receiver, TryIter};
use bus::{Bus, BusReader};

pub struct SignaldSocket {
    socket_path: String,
    socket: UnixStream,
    bus: Arc<Mutex<Bus<String>>>,
}
impl SignaldSocket {
    pub fn connect(socket_path:String) -> SignaldSocket {

        let socket = match UnixStream::connect(socket_path.to_string()){
            Ok(stream) => {
                println!("Connected to socket");
                stream
            }
            Err(err) => {
                panic!("Failed to connect socket");
            }
        };
        let socket_clone = socket.try_clone().unwrap();

        // Create a thread for the reader to use
        let bus = Arc::new(Mutex::new(Bus::new(10)));
        //let (tx, rx) = mpsc::channel::<String>();
        let bus_tx = bus.clone();
        thread::spawn(move || {
            let reader = BufReader::new(socket);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        //tx.send(l);
                        bus_tx.lock().unwrap().broadcast(l);
                    },
                    Err(_) => {

                    }
                }
            }
        });


        Self {
            socket_path: socket_path,
            socket: socket_clone,
            bus: bus,
        }
    }

    pub fn send_request(&mut self, request: &SignaldRequest) {
        let formatted_request = request.to_json_string() + "\n";
        match self.socket.write_all(formatted_request.as_bytes()) {
            Err(_) => panic!("Failed to send message"),
            Ok(_) => {
                println!("mesg sent {}", formatted_request);
            }
        }
    }

    pub fn get_rx(&mut self) -> BusReader<String> {
        self.bus.lock().unwrap().add_rx()
    }
}
