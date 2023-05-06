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

    println!("Listening for incoming messages ... (ctrl-c to exit)");
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
    let send_mail_msg: mailer_common::SendMailMessage = serde_json::from_str(cow.as_ref()).unwrap();
    println!("Received message {:?}", send_mail_msg);
    sendgrid::send_mail(send_mail_msg).await;
}