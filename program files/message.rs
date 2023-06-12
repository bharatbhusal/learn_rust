// Create a module named messaging and define an enum Message inside it.
// The enum should have two variants: Email and SMS.
// The Email variant should contain fields for sender (String), subject (String), and body (String).
// The SMS variant should contain fields for sender (String) and content (String).
// Implement a method send for the Message enum that prints out the details of the message.

mod messaging {
    #[derive(Debug)]
    pub enum Message {
        Sms {
            sender: String,
            content: String,
        },
        Email {
            sender: String,
            subject: String,
            body: String,
        },
    }
    impl Message {
        pub fn send(&self) {
            match self {
                Message::Sms { sender, content } => {
                    println!("Sender: {}\nContent: {}", sender, content)
                }
                Message::Email {
                    sender,
                    subject,
                    body,
                } => println!("Sender: {}\nSubject: {}\nBody: {}", sender, subject, body),
            }
        }
        pub fn get_content(&self) -> &str {
            match self {
                Message::Sms { content, .. } => content,
                Message::Email { body, .. } => body,
            }
        }
        pub fn get_sender(&self) -> Option<&str> {
            match self {
                Message::Sms { sender, .. } => Some(sender),
                Message::Email { sender, .. } => Some(sender),
            }
        }
        pub fn get_subject(&self) -> Option<&str> {
            match self {
                Message::Email { subject, .. } => Some(subject),
                _ => None,
            }
        }
    }
}

use messaging::Message;

fn main() {
    let mysms = Message::Sms {
        sender: String::from("Bharat"),
        content: String::from("Hi there"),
    };
    let mymail = Message::Email {
        sender: String::from("Pabitra"),
        subject: String::from("Leave"),
        body: String::from("I want leave"),
    };
    println!("Sender of sms: {:?}", mysms.get_sender());
    println!("Content of sms: {:?}", mysms.get_content());
    println!("Subject of sms: {:?}", mysms.get_subject());

    println!("Sender of mail: {:?}", mymail.get_sender());
    println!("Subject of mail: {:?}", mymail.get_subject());
    println!("Content of mail: {:?}", mymail.get_content());

    println!("SMS: ");
    mysms.send();
    println!("Mail: ");
    mymail.send();
}
