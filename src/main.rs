use nym_sdk::mixnet::{self, ReconstructedMessage};
use serde::{Serialize, Deserialize};
use tokio::spawn;


#[derive(Serialize, Deserialize, Debug)]
struct SendMailMessage<'a> {
    from: &'a str,
    to: &'a str,
    body: &'a str,
}

#[tokio::main]
async fn main() {
    //nym_bin_common::logging::setup_logging();
    // Passing no config makes the client fire up an ephemeral session and figure shit out on its own
    let mut client = mixnet::MixnetClient::connect_new().await.unwrap();
    // Be able to get our client address
    let our_address = client.nym_address();
    println!("Our client nym address is: {our_address}");

    let send_mail_msg = SendMailMessage {
        from: "nym",
        to: "gabrio.tognozzi@gmail.com",
        body:"test",
    };

    let json_msg = serde_json::to_string(&send_mail_msg).unwrap();

    // Send a message throught the mixnet to ourselves
    client.send_str(*our_address, json_msg.as_str()).await;

    println!("Waiting for message (ctrl-c to exit)");

    client
        .on_messages(|msg| {spawn(async move {
            // Call your async function here
            process_message_async(msg).await;
        });})
        .await;

}

async fn process_message_async(msg: ReconstructedMessage){
    let cow = String::from_utf8_lossy(&msg.message);
    let send_mail_msg: SendMailMessage =  serde_json::from_str(cow.as_ref()).unwrap();
    println!("Received: {:?}", send_mail_msg);

}


#[cfg(test)]
mod tests {
 
    #[test]
    fn example_dh() {

        assert_eq!(1,1);
    }

}


