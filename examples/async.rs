use signald_rust::signald::Signald;
use signald_rust::signaldresponse::ResponseType;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // Edit "my_number" to your linked signald phone number
    let my_number =  "+32472271852".to_string();

    let mut signald = Arc::new(Mutex::new(Signald::connect()));
    println!("Connected to socket");

    let mut sig = signald.clone().lock().unwrap();
    tokio::spawn(async move {
        sig.list_contacts(my_number).await;
    });


    match signald.lock().unwrap().subscribe(my_number.clone()).await.unwrap().data {
        ResponseType::Subscribed => {
            println!("Subscribed to {}", my_number.clone());
        }
        _ => panic!("Failed to subscribe to {}", my_number.clone())
    }


    loop {

    }

}

