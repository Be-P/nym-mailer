use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct SendMailMessage<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub subject: &'a str,
    pub body: &'a str,
}

impl<'a> SendMailMessage<'a> {
    pub fn new(from: &'a str, to: &'a str, subject: &'a str, body: &'a str) -> Self {
        SendMailMessage {
            from,
            to,
            subject,
            body,
        }
    }
}
