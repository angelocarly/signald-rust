use crate::signaldrequest::SignaldRequestBuilder;
use crate::signaldrequest::SignaldRequest;
use tokio::time::*;
use std::time::Duration;
use bus::{BusReader};
use std::sync::mpsc::RecvTimeoutError::Timeout;
use std::sync::mpsc::RecvTimeoutError;
use crate::signaldresponse::{SignaldResponse, ResponseType, VersionData};
use crate::signald::FilterType::{Id, Type};
use crate::signaldresponse::ResponseType::BusUpdate;
use crate::socket::Socket;
use crate::socket::signaldsocket::SignaldSocket;

pub static SOCKET_PATH: &'static str = "/var/run/signald/signald.sock";

pub enum FilterType {
    Id(String),
    Type(ResponseType)
}

pub struct Signald {
    // The signald socket
    socket: Box<dyn Socket>,
    // A count of all the sent messages on this socket
    message_count: u32,
}
impl Signald {

    /// Connect the default Signald socket
    pub fn connect() -> Signald {
        Signald::connect_path(&SOCKET_PATH)
    }
    /// Connect to a custom Signald socket
    pub fn connect_path(socket_path: &str) -> Self {
        let socket: SignaldSocket = SignaldSocket::connect(socket_path.to_string(), 100);

        Self {
            socket: Box::new(socket),
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
        let mut request_builder = SignaldRequestBuilder::new();
        request_builder.set_type("send".to_string());
        request_builder.set_username(username);
        request_builder.set_recipient_number(recipient_number);
        if let Some(i) = message_body {
            request_builder.set_message_body(i);
        }

        let request = request_builder.build();
        self.send_request(&request);
    }

    /// Enable receiving user events such as received messages
    pub async fn subscribe(&mut self, username: String) -> Result<SignaldResponse, RecvTimeoutError> {
        let id = self.message_count.to_string();

        let mut request_builder = SignaldRequestBuilder::new();
        request_builder.set_type("subscribe".to_string());
        request_builder.set_username(username);
        request_builder.set_id(id.clone());
        let request = request_builder.build();

        self.send_request(&request);
        self.wait_for_request(Id(id)).await
    }
    /// Disable receiving user events such as received messages
    pub async fn unsubscribe(&mut self, username: String) -> Result<SignaldResponse, RecvTimeoutError> {
        let id = self.message_count.to_string();

        let mut request_builder = SignaldRequestBuilder::new();
        request_builder.set_type("unsubscribe".to_string());
        request_builder.set_username(username);
        request_builder.set_id(id.clone());
        let request = request_builder.build();

        self.send_request(&request);
        self.wait_for_request(Id(id)).await
    }
    /// Link an existing signal account
    pub async fn link(&mut self) -> Result<SignaldResponse, RecvTimeoutError> {
        let id = self.message_count.to_string();

        let mut request_builder = SignaldRequestBuilder::new();
        request_builder.set_type("link".to_string());
        request_builder.set_id(id.clone());
        let request = request_builder.build();

        self.send_request(&request);
        self.wait_for_request(Id(id)).await
    }
    /// Get the current signald version
    pub async fn version(&mut self) -> Result<SignaldResponse, RecvTimeoutError> {
        let id = self.message_count.to_string();

        let mut request_builder = SignaldRequestBuilder::new();
        request_builder.set_type("version".to_string());
        request_builder.set_id(id.clone());
        let request = request_builder.build();

        self.send_request(&request);
        self.wait_for_request(Type(ResponseType::Version(None))).await
    }
    /// Query all the user's contacts
    pub async fn list_contacts(&mut self, username: String) -> Result<SignaldResponse, RecvTimeoutError> {
        let id = self.message_count.to_string();

        let mut request_builder = SignaldRequestBuilder::new();
        request_builder.set_type("list_contacts".to_string());
        request_builder.set_username(username);
        request_builder.set_id(id.clone());
        let request = request_builder.build();

        self.send_request(&request);
        self.wait_for_request(Id(id)).await
    }
    /// Send a contact sync request to the other devices on this account
    pub fn sync_contacts(&mut self, username: String) {
        let mut request_builder = SignaldRequestBuilder::new();
        request_builder.set_type("sync_contacts".to_string());
        request_builder.set_username(username);
        request_builder.set_id(self.message_count.to_string());
        let request = request_builder.build();

        self.send_request(&request);
    }
    /// Get a response stream that returns every received message on the socket
    pub fn get_rx(&mut self) -> BusReader<SignaldResponse> {
        self.socket.get_rx()
    }

    /// Get a response from the bus with a matching id or type
    /// Returns a RecvTimeoutError if the message took more than 3 seconds to return
    async fn wait_for_request(&mut self, filter: FilterType) -> Result<SignaldResponse, RecvTimeoutError> {
        // The max possible time to receive a message
        let end = Instant::now() + Duration::from_millis(3000);
        let mut rx = self.socket.get_rx();

        let result = rx.iter()
            // Stop the receiver once the time is over, this keeps updating thanks to the update messages in systemdsocket
            .take_while(|_| Instant::now() < end )
            .find(|response| {
                // The systemdsocket sends an 'update' message each second, don't parse this
                if let BusUpdate = response.data { return false; }

                Signald::filter_request(&filter, &response)
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

    fn filter_request(filter: &FilterType, message: &SignaldResponse) -> bool {
        match filter {
            // Filter on id
            Id(req_id) => {
                match &message.id {
                    Some(s) => {
                        return s == req_id.as_str();
                    },
                    None => {
                        false
                    }
                }
            }
            // Filter on response type
            Type(req_type) => {
                let disc1 = std::mem::discriminant(req_type);
                let disc2 = std::mem::discriminant(&message.data);
                return disc1 == disc2;
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_request_id_success() {
        let id = "test".to_string();

        let message = SignaldResponse {
            id: Some(id.clone()),
            data: ResponseType::Subscribed
        };

        assert!(Signald::filter_request(&Id(id), &message));
    }

    #[test]
    fn test_filter_request_id_wrong() {
        let id = "test".to_string();

        let message = SignaldResponse {
            id: Some(id.clone()),
            data: ResponseType::Subscribed
        };

        assert!(!Signald::filter_request(&Id("INCORRECT_ID".to_string()), &message));
    }

    #[test]
    fn test_filter_request_type_correct() {
        let message = SignaldResponse {
            id: None,
            data: ResponseType::Subscribed
        };

        assert!(Signald::filter_request(&Type(ResponseType::Subscribed), &message));
    }

    #[test]
    fn test_filter_request_type_wrong() {
        let message = SignaldResponse {
            id: None,
            data: ResponseType::Subscribed
        };

        assert!(!Signald::filter_request(&Type(ResponseType::Unsubscribed), &message));
    }
}
