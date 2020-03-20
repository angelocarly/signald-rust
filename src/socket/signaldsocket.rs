use std::sync::{Arc, Mutex};
use std::os::unix::net::UnixStream;
use std::thread;
use std::io::{Write, BufReader, BufRead};
use bus::{Bus, BusReader};
use std::time::Duration;
use crate::signaldresponse::SignaldResponse;
use crate::signaldresponse::ResponseType::BusUpdate;
use crate::signaldrequest::SignaldRequest;
use crate::socket::Socket;

#[allow(dead_code)]
pub struct SignaldSocket {
    socket_path: String,
    socket: UnixStream,
    bus: Arc<Mutex<Bus<SignaldResponse>>>,
}
impl SignaldSocket {
    pub fn connect(socket_path: String, bus_size: usize) -> SignaldSocket {

        // Connect the socket
        let socket = match UnixStream::connect(socket_path.to_string()) {
            Ok(stream) => {
                stream
            }
            Err(_) => {
                panic!("Failed to connect socket");
            }
        };
        let socket_clone = socket.try_clone().unwrap();

        // Create a bus
        let bus = Arc::new(Mutex::new(Bus::new(bus_size)));

        // Broadcast on the bus in a new thread
        let bus_tx = bus.clone();
        thread::spawn(move || {
            let reader = BufReader::new(socket);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        let val = serde_json::from_str(&l).unwrap();
                        let res: SignaldResponse = SignaldResponse::from_value(val);
                        bus_tx.lock().unwrap().broadcast(res);
                    },
                    Err(_) => {}
                }
            }
        });

        // An update message every second to make sure that the receivers can verify the time they're waiting
        // When there are no messages on the bus the receivers would otherwise be stuck waiting
        // This is a hacky implementation and should be changed once recv_deadline can be implemented
        let bus_tx_seconds = bus.clone();
        let update_response = SignaldResponse {
            id: None,
            data: BusUpdate
        };
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                bus_tx_seconds.lock().unwrap().broadcast(update_response.clone());
            }
        });


        Self {
            socket_path: socket_path,
            socket: socket_clone,
            bus: bus,
        }
    }
}
impl Socket for SignaldSocket {
    fn send_request(&mut self, request: &SignaldRequest) {
        let formatted_request = request.to_json_string() + "\n";
        match self.socket.write_all(formatted_request.as_bytes()) {
            Err(_) => panic!("Failed to send message"),
            Ok(_) => {
                //println!("mesg sent {}", formatted_request);
            }
        }
    }

    fn get_rx(&mut self) -> BusReader<SignaldResponse> {
        self.bus.lock().unwrap().add_rx()
    }
}
