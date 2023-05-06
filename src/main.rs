use nym_sdk::mixnet::{self, ReconstructedMessage};
use tokio::spawn;
use dotenv::dotenv;

mod sendgrid;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut client = mixnet::MixnetClient::connect_new().await.unwrap();

    // Be able to get our client address
    let our_address = client.nym_address();
    println!("Our Service Provider Nym address is: {our_address}");

    let send_mail_msg =
        sendgrid::SendMailMessage::new("gabrio.tognozzi", "gabrio.tognozzi@cyberleap.it", "Subject", "test");

    let json_msg = serde_json::to_string(&send_mail_msg).unwrap();

    // Send a message throught the mixnet to ourselves
    client.send_str(*our_address, json_msg.as_str()).await;

    println!("Waiting for message (ctrl-c to exit)");

    client
        .on_messages(|msg| {
            spawn(async move {
                // Call your async function here
                process_message_async(msg).await;
            });
        })
        .await;
}

async fn process_message_async(msg: ReconstructedMessage) {
    let cow = String::from_utf8_lossy(&msg.message);
    let send_mail_msg: sendgrid::SendMailMessage = serde_json::from_str(cow.as_ref()).unwrap();
    sendgrid::send_mail(send_mail_msg).await;
}