use std::{env, process, time::Duration};

extern crate paho_mqtt as mqtt;

const DEFAULT_BROKER:&str="tcp://broker.empx.io:1883";
const DEFAULT_CLIENT:&str = "rust_publish";
const DEFAULT_TOPICS:&[&str] = &["rust/mqtt", "rust/test"];
const QOS:i32 =1;

fn main() {
    let host = env::args().nth(1).unwrap_or_else(||DEFAULT_BROKER.to_string());

    let create_options = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(DEFAULT_CLIENT.to_string())
        .finalize();

    let client = mqtt::Client::new(create_options).unwrap_or_else(|err|{
        println!("Error creating a client: {:?}", err);
        process::exit(1);
    });

    let connection_options = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    if let Err(e)=client.connect(connection_options){
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    for num in 0..5{
        let value = "Hello world!".to_string()+&num.to_string();
        let mut message = mqtt::Message:: new(DEFAULT_TOPICS[0], value.clone(),QOS);
        if num % 2 == 0 {
            println!("Publishing messages on the {:?} topic", DEFAULT_TOPICS[1]);
            message = mqtt::Message::new(DEFAULT_TOPICS[1], value.clone(),QOS);
        } else {
            println!("Publishing messages on the {:?} topic", DEFAULT_TOPICS[0]);
        }
        let token = client.publish(message);

        if let Err(e)=token{
            println!("Error sending message: {:?}", e);
            break;
        }
    }
    let token = client.disconnect(None);
    println!("Disconnect from the broker");
    token.unwrap();

}