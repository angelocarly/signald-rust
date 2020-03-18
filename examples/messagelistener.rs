use signald_rust::signald::signald::Signald;
use signald_rust::signald::signaldresponse::ResponseType;

#[tokio::main]
async fn main() {
    let mut signald = Signald::connect();
    println!("Connected to socket");

    // Edit "my_number" to your linked signald phone number
    let my_number =  "my_number".to_string();

    match signald.subscribe(my_number.clone()).await.unwrap().data {
        ResponseType::Subscribed => {
            println!("Subscribed to {}", my_number.clone());
        }
        _ => panic!("Failed to subscribe to {}", my_number.clone())
    }

    // Print each sent/received message, note: messages sent via signald will not be received
    signald.get_rx().iter().for_each(|x| {
        match x.data {
            ResponseType::Message(message) => {
                if message.sync_message.is_some() {
                    let sync = message.sync_message.unwrap();
                    if sync.sent.is_some() {
                        let mesg = sync.sent.unwrap().message.message;
                        println!("SENT: {}", mesg);
                    }
                }
                if message.data_message.is_some() {
                    let mesg = message.data_message.unwrap().message;
                    println!("RECEIVED: {}", mesg);
                }
            }
            _ => {}
        }
    })

}

