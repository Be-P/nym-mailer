use std::env;

use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Personalization {
    to: Vec<Recipient>,
    subject: String,
}

#[derive(Serialize)]
struct Recipient {
    email: String,
}

#[derive(Serialize)]
struct Email {
    personalizations: Vec<Personalization>,
    from: Recipient,
    content: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    r#type: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SendMailMessage<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    body: &'a str,
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

pub(crate) async fn send_mail(msg: SendMailMessage<'_>) {
    let sendgrid_api = env::var("SENDGRID_API_KEY").unwrap();
    let header = format!("Bearer {}", sendgrid_api);
    let client = Client::new();
    let url = "https://api.sendgrid.com/v3/mail/send";

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(header.as_str()).unwrap(),
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let email = Email {
        personalizations: vec![Personalization {
            to: vec![Recipient {
                email: msg.to.to_string(),
            }],
            subject: msg.subject.to_string(),
        }],
        from: Recipient {
            email: msg.from.to_string(),
        },
        content: vec![Content {
            r#type: "text/plain".to_string(),
            value: msg.body.to_string(),
        }],
    };
    let response = client.post(url).headers(headers).json(&email).send().await;
    println!("{:?}", response);
}
