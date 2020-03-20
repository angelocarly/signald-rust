use crate::signaldrequest::SignaldRequestBuilder;
use crate::signaldrequest::SignaldRequest;
use crate::signaldsocket::{SignaldSocket};
use tokio::time::*;
use std::time::Duration;
use bus::{BusReader};
use std::sync::mpsc::RecvTimeoutError::Timeout;
use std::sync::mpsc::RecvTimeoutError;
use crate::signaldresponse::{SignaldResponse};
use crate::signald::FilterType::{Id};
use crate::signaldresponse::ResponseType::BusUpdate;
pub static SOCKET_PATH: &'static str = "/var/run/signald/signald.sock";

pub enum FilterType {
    Id,
    Type
}

pub struct Signald {
    // The signald socket
    socket: SignaldSocket,
    // A request builder which is reused to limit memory allocation
    request_builder: SignaldRequestBuilder,
    // A count of all the sent messages on this socket
    message_count: u32,
}
impl Signald {

    /// Connect the default Signald socket
    pub fn connect() -> Signald {
        Signald::connect_path(SOCKET_PATH)
    }
    /// Connect to a custom Signald socket
    pub fn connect_path(socket_path: &str) -> Signald {
        Signald {
            socket: SignaldSocket::connect(socket_path.to_string(), 100),
            request_builder: SignaldRequestBuilder::new(),
            message_count: 0,
        }
    }
    /// Send a signald request on the socket
    pub fn send_request(&mut self, request: &SignaldRequest) {
        self.message_count += 1;
        self.socket.send_request(&request);
    }

    // Signald messages
    // Todo: add attachments, etc
    /// Send a message to the socket
    pub async fn send(&mut self, username: String, recipient_number: String, message_body: Option<String>) {
        self.request_builder.flush();
        self.request_builder.set_type("send".to_string());
        self.request_builder.set_username(username);
        self.request_builder.set_recipient_number(recipient_number);
        if let Some(i) = message_body {
            self.request_builder.set_message_body(i);
        }

        let request = self.request_builder.build();
        self.send_request(&request);
    }

    /// Enable receiving user events such as received messages
    pub async fn subscribe(&mut self, username: String) -> Result<SignaldResponse, RecvTimeoutError> {
        let id = self.message_count.to_string();

        self.request_builder.flush();
        self.request_builder.set_type("subscribe".to_string());
        self.request_builder.set_username(username);
        self.request_builder.set_id(id.clone());
        let request = self.request_builder.build();

        self.send_request(&request);
        self.wait_for_request(Id, id).await
    }
    /// Disable receiving user events such as received messages
    pub async fn unsubscribe(&mut self, username: String) -> Result<SignaldResponse, RecvTimeoutError> {
        let id = self.message_count.to_string();
        self.request_builder.flush();
        self.request_builder.set_type("unsubscribe".to_string());
        self.request_builder.set_username(username);
        self.request_builder.set_id(id.clone());
        let request = self.request_builder.build();

        self.send_request(&request);
        self.wait_for_request(Id, id).await
    }
    /// Link an existing signal account
    pub async fn link(&mut self) -> Result<SignaldResponse, RecvTimeoutError> {
        let id = self.message_count.to_string();
        self.request_builder.flush();
        self.request_builder.set_type("link".to_string());
        self.request_builder.set_id(id.clone());
        let request = self.request_builder.build();

        self.send_request(&request);
        self.wait_for_request(Id, id).await
    }
    // Get the current signald version
    // pub async fn version(&mut self) -> Result<SignaldResponse, RecvTimeoutError> {
    //     let id = self.message_count.to_string();
    //
    //     self.request_builder.flush();
    //     self.request_builder.set_type("version".to_string());
    //     self.request_builder.set_id(id.clone());
    //     let request = self.request_builder.build();
    //
    //     self.send_request(&request);
    //     self.wait_for_request(Type, ResponseType::Version()).await
    // }
    /// Query all the user's contacts
    pub async fn list_contacts(&mut self, username: String) -> Result<SignaldResponse, RecvTimeoutError> {
        let id = self.message_count.to_string();

        self.request_builder.flush();
        self.request_builder.set_type("list_contacts".to_string());
        self.request_builder.set_username(username);
        self.request_builder.set_id(id.clone());
        let request = self.request_builder.build();

        self.send_request(&request);
        self.wait_for_request(Id, id).await
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
    /// Get a response stream that returns every received message on the socket
    pub fn get_rx(&mut self) -> BusReader<SignaldResponse> {
        self.socket.get_rx()
    }
    // Todo: make it possible to filter on type
    /// Get a response from the bus with a matching id
    /// Returns a RecvTimeoutError if the message took more than 3 seconds to return
    async fn wait_for_request(&mut self, typ: FilterType, val: String) -> Result<SignaldResponse, RecvTimeoutError> {
        // The max possible time to receive a message
        let end = Instant::now() + Duration::from_millis(3000);
        let mut rx = self.socket.get_rx();

        let result = rx.iter()
            // Stop the receiver once the time is over, this keeps updating thanks to the update messages in systemdsocket
            .take_while(|_| Instant::now() < end )
            .find(|response| {
                // The systemdsocket sends an 'update' message each second, don't parse this
                if let BusUpdate = response.data { return false; }

                match typ {
                    Id=> {
                        match &response.id {
                            Some(s) => {
                                return s == val.as_str();
                            },
                            None => {
                                false
                            }
                        }
                    }
                    // Type => {
                    //     return match &response._data {
                    //         val => {
                    //             true
                    //         }
                    //     };
                    // }
                    _ => panic!("Wrong singald filter"),
                }
            });

        // When no results are found within the time limit, an error is returned
        match result {
            Some(x) => {
                Ok(x)
            },
            None => {
                Err(Timeout)
            }
        }

    }
}
