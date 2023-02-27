#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

struct GroundStation;

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

impl CubeSat {
    /// Calls Mailbox.deliver() to receive Messages, gaining ownership of a Message
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl Mailbox {
    /// Requires mutable access to itself and ownership over a Message
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    /// Requires a shared reference to a Cubesat to pull out its id field
    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);

                // When we find a message, return early with the Message wrapped in Some
                return Some(msg);
            }
        }

        None
    }
}

impl GroundStation {
    /// Calls Mailbox.post() to send messages, yielding ownership of a Message
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
            mailbox: Mailbox { messages: vec![] },
        }
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    println!("CubeSat");

    let mut mail = Mailbox { messages: vec![] };
    let base = GroundStation {};

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        let msg = Message {
            to: sat_id,
            content: String::from("Hello"),
        };
        base.send(&mut mail, msg);

        // base.send(&mut sat, Message::from("hello."));
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);

        println!("{:?}: {:?}", sat, msg);
    }
}
