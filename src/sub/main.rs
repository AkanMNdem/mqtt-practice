use std::{env, process, thread, time::Duration};

extern crate paho_mqtt as mqtt;

const DEFAULT_BROKER:&str="tcp://broker.empx.io:1883";
const DEFAULT_CLIENT:&str = "rust_publish";
const DEFAULT_TOPICS:&[&str] = &["rust/mqtt", "rust/test"];
const DEFAULT_QOS:&[i32]=&[0,1];

fn try_reconnect(client: &mqtt::Client) -> bool {
    println!("Connection lost. Reconnection is being attempted");
    for _ in 0..12{
        thread::sleep(Duration::from_millis(5000));
        if client.reconnect().is_ok(){
            println!("Sucessfully reconnected");
            return true;
        }

    }
    println!("Unable to reconnect.");
    false
}

fn subscribe_to_topics(client: &mqtt::Client){
    if let Err(e) = client.subscribe_many(DEFAULT_TOPICS,DEFAULT_QOS){
        println!("Error subricibing topics: {:?}", e);
        process::exit(1);
    }
}
fn main(){
    let host = env::args().nth(1).unwrap_or_else(||DEFAULT_BROKER.to_string());

    let create_options = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(DEFAULT_CLIENT.to_string())
        .finalize();

    let client = mqtt::Client::new(create_options).unwrap_or_else(|err|{
        println!("Error creating a client: {:?}", err);
        process::exit(1);
    });

    let rx = client.start_consuming();

    let lwt = mqtt::MessageBuilder::new()
        .topic("test")
        .payload("Consumer lost connection")
        .finalize();

    let connection_options = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .will_message(lwt)
        .finalize();

    if let Err(e) = client.connect(connection_options){
        println!("Unable to connect: \n\t{:?}", e);
        process::exit(1)
    }

    subscribe_to_topics(&client);
    println!("Processing requests...");
    for message in rx.iter(){
        if let Some(message)= message{
            println!{"{}", message};
        }
        else if !client.is_connected(){
            if try_reconnect(&client){
                println!("Resubscribing topics...");
                subscribe_to_topics(&client)
            } else{
                break;
            }
        }
    }

    if client.is_connected(){
        println!("Disconnecting");
        client.unsubscribe_many(DEFAULT_TOPICS).unwrap();
        client.disconnect(None).unwrap();
    }
    println!("Exiting");

}