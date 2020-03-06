use crate::signald::signaldrequest::SignaldRequestBuilder;
use crate::signald::signaldrequest::SignaldRequest;
use crate::signald::signaldsocket::{SignaldSocket};
use bus::Bus;
use serde_json::Value;

/// Responsible for all the communication to the signald socket
pub struct Signald {
    // The signald socket
    socket: SignaldSocket,
    // A request builder which is reused to limit memory allocation
    request_builder: SignaldRequestBuilder,
    // A count of all the sent messages on this socket
    message_count: u32,
}
impl Signald {

    /// Connect the Signald socket
    pub fn connect(socket_path: String) -> Signald {
        Signald {
            socket: SignaldSocket::connect(socket_path),
            request_builder: SignaldRequestBuilder::new(),
            message_count: 0,
        }
    }
    /// Add a subscriber hook to be notified on signald events
    pub fn send_request(&mut self, request: &SignaldRequest) {
        self.socket.send_request(&request);
        self.message_count += 1;
    }
    /// Enable receiving user events such as received messages
    pub fn subscribe(&mut self, username: String) {
        self.request_builder.flush();
        self.request_builder.set_type("subscribe".to_string());
        self.request_builder.set_username(username);
        let request = self.request_builder.build();

        self.send_request(&request);
    }
    /// Disable receiving user events such as received messages
    pub fn unsubscribe(&mut self, username: String) {
        self.request_builder.flush();
        self.request_builder.set_type("unsubscribe".to_string());
        self.request_builder.set_username(username);
        let request = self.request_builder.build();

        self.send_request(&request);
    }
    /// Link an existing signal account
    pub fn link(&mut self) {
        self.request_builder.flush();
        self.request_builder.set_type("link".to_string());
        let request = self.request_builder.build();

        self.send_request(&request);
    }
    /// Get the current signald version
    pub fn version(&mut self) {
        self.request_builder.flush();
        self.request_builder.set_type("version".to_string());
        let request = self.request_builder.build();

        self.send_request(&request);
    }
    /// Query all the user's contacts
    pub async fn list_contacts(&mut self, username: String) -> String {
        self.request_builder.flush();
        self.request_builder.set_type("list_contacts".to_string());
        self.request_builder.set_username(username);
        self.request_builder.set_id(self.message_count.to_string());
        let request = self.request_builder.build();

        let id = self.message_count.to_string();

        self.send_request(&request);

        let mut rx = self.socket.get_rx();
        for l in rx.iter() {
            let v: Value = serde_json::from_str(l.as_str()).expect("Couldn't parse message");
            match v["id"].as_str() {
                Some(s) => {
                   if s == id {
                       return l;
                   }
                },
                None => {}
            }
        }

        return "".parse().unwrap();
    }
    /// Send a contact sync request to the other devices on this account
    pub fn sync_contacts(&mut self, username: String) {
        self.request_builder.flush();
        self.request_builder.set_type("sync_contacts".to_string());
        self.request_builder.set_username(username);
        self.request_builder.set_id(self.message_count.to_string());
        let request = self.request_builder.build();

        self.send_request(&request);
    }
}
