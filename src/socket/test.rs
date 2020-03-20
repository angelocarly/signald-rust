use std::sync::{Arc, Mutex};
use std::io::{BufReader, BufRead};
use bus::{Bus, BusReader};
use crate::signaldresponse::SignaldResponse;
use crate::signaldrequest::SignaldRequest;
use crate::socket::Socket;
use crate::signaldresponse::ResponseType::BusUpdate;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub struct TestSignaldSocket {
    bus: Arc<Mutex<Bus<SignaldResponse>>>,
}
impl TestSignaldSocket {
    pub fn new(bus_size: usize) -> TestSignaldSocket {
        let bus = Arc::new(Mutex::new(Bus::new(bus_size)));

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
            bus,
        }
    }

    pub fn get_tx(&mut self) -> Arc<Mutex<Bus<SignaldResponse>>> {
        self.bus.clone()
    }
}
impl Socket for TestSignaldSocket {
    fn send_request(&mut self, request: &SignaldRequest) {
        unimplemented!()
    }

    fn get_rx(&mut self) -> BusReader<SignaldResponse> {
        self.bus.lock().unwrap().add_rx()
    }
}
