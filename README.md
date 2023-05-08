# Nym Mailer

This is the POC of a Nym Service Provider that receives emails through the Nym Network and relays them to the internet.

## Mailer Server

The Server crate, that receives messages and sends them through SMTP to the specified destination.
Right now it just forwards every email request to the destination requested by the client.

## Mailer Client

The POC of a client that sends to the Mailer Server a "SendMailMessage" request. 

## Getting Started

First, start the server.

```bash
cd mailer-server

# Configure Sendgrid API and Domain
cp example.env .env

cargo run
```

Copy and paste the server Nym address into the client `destination_address` variable, and then run the client

```bash
cd mailer-client

cargo run
```

## Future Plans

This repository only provides a POC of the Nym Mailer Service, the next steps follow:
 - Remove the Sendgrid dependency, and implement a native SMTP relay
 - Implement a Web UI and a Native UI client interface 
 - Allow Clients tu buy email accounts, and use "nym credentials" to ensure the user has right to use an email account
 - Allow Clients to receive back email responses implementing a mailbox
