use crate::signaldrequest::SignaldRequest;
use crate::signaldresponse::SignaldResponse;
use bus::BusReader;

pub mod signaldsocket;
pub mod test;

pub trait Socket {
    fn send_request(&mut self, request: &SignaldRequest);
    fn get_rx(&mut self) -> BusReader<SignaldResponse>;
}
