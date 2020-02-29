use std::{thread, time};
use crate::signald::signaldrequest::SignaldRequestBuilder;
use common::SOCKET_PATH;
use signald::signald::Signald;
use async_std::prelude::*;
use crate::signald::signaldsocket::SignaldEvents;

mod common;
mod signald;

struct Logger {

}
impl SignaldEvents for Logger {
    fn on_connect(&self, path: &str) {
        println!("Connected to {}", path);
    }
    fn on_message(&self, mesg: &str) {
        println!("received msg {}", mesg);
    }
}

/**
 * Main is currently used for debugging purposes
 * The library itself only consists of the signal/ folder
 */
fn main() {
    let mut signald = Signald::new(SOCKET_PATH.to_string());
    let logger = Logger {};
    signald.add_event_hook(logger);
    signald.connect();

    let mut messagebuilder = SignaldRequestBuilder::new();
    messagebuilder.set_type("send".to_string());
    messagebuilder.set_username("+32472271852".to_string());
    messagebuilder.set_recipient_number("+32472271852".to_string());
    messagebuilder.set_message_body("Heeey jarne".to_string());
    let req = messagebuilder.build();

    signald.subscribe("+32472271852".to_string());

    // signald.subscribe("+32472271852".to_string());
    signald.send_request(&req);
    
    // signald.link();
    // signald.version();
    // signald.list_contacts("+32472271852".to_string());
    // signald.list_contacts("+32472271852".to_string());
    signald.list_contacts("+32472271852".to_string());
//    signald.read_requests();

    loop {
        signald.sync();
    }

}

