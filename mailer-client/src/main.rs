use nym_sdk::mixnet::{self, Recipient};


#[tokio::main]
async fn main() {
    let mut client = mixnet::MixnetClient::connect_new().await.unwrap();

    // Be able to get our client address
    let our_address = client.nym_address();
    println!("Our Service Provider Nym address is: {our_address}");

    let destination_address = "21KMZoMNfdtUf5fk5jy7QemedcXG1rCq49BDPE9oJewC.FNHoGcUXiyjAFKR58BiyyZ9eU3gQtMoE2DnrdxJ67jxX@E3mvZTHQCdBvhfr178Swx9g4QG3kkRUun7YnToLMcMbM";
    let recipient = Recipient::try_from_base58_string(destination_address).unwrap();

    let send_mail_msg =
        mailer_common::SendMailMessage::new("gabrio.tognozzi", "gabrio.tognozzi@fluus.com", "Subject", "test");

    let json_msg = serde_json::to_string(&send_mail_msg).unwrap();

    println!("Sending {json_msg} to {recipient}");

    client.send_str(recipient, json_msg.as_str()).await;

    // TODO Calling disconnect() before waiting for a message, would panic(), probably a bug
    client.send_str(*our_address, json_msg.as_str()).await;
    let msg = client.wait_for_messages().await.unwrap();
    println!("Received message {:?}!",msg);

    println!("Disconnecting client ...");
    client.disconnect().await;
}
