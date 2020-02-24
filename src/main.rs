use crate::signald::signaldrequest::SignaldRequestBuilder;
use common::SOCKET_PATH;
use signald::signald::Signald;

mod common;
mod signald;

/**
 * Main is currently used for debugging purposes
 * The library itself only consists of the signal/ folder
 */
fn main() {
    let mut signald = Signald::connect(SOCKET_PATH.to_string());

    let mut messagebuilder = SignaldRequestBuilder::new();
    messagebuilder.set_type("send".to_string());
    messagebuilder.set_username("+32472271852".to_string());
    messagebuilder.set_recipient_number("+32472271852".to_string());
    messagebuilder.set_message_body("TReess1".to_string());
    let req = messagebuilder.build();

    // signald.subscribe("+32472271852".to_string());
    // signald.send_request(&req);
    // signald.link();
    // signald.version();
    signald.list_contacts("+32472271852".to_string());
    signald.list_contacts("+32472271852".to_string());
    signald.list_contacts("+32472271852".to_string());

    signald.read_requests();
    // Read all incoming messages
    // let stream = BufReader::new(stream);
    // for line in stream.lines() {
    // println!("{}", line.unwrap());
    // }
}
