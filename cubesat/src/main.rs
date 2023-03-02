#![allow(unused_variables)]

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64, // Mhz
}

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

    // An object that has interior mutability presents an immutable facade while internal values are being modified
    let base: Rc<RefCell<GroundStation>> =
        Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));

    println!("base: {:?}", base);

    // A new scope where base can be mutably borrowed
    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
    }

    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {:?}", base); // value: <borrowed> indicates that base is mutably borrowed in this scope and is no longer generally accessible
    println!("base_3: {:?}", base_3);

    // let mut mail = Mailbox { messages: vec![] };

    // let sat_ids = fetch_sat_ids();

    // for sat_id in sat_ids {
    //     let mut sat = base.connect(sat_id);
    //     let msg = Message {
    //         to: sat_id,
    //         content: String::from("Hello"),
    //     };
    //     base.send(&mut mail, msg);

    //     // base.send(&mut sat, Message::from("hello."));
    // }

    // let sat_ids = fetch_sat_ids();

    // for sat_id in sat_ids {
    //     let sat = base.connect(sat_id);
    //     let msg = sat.recv(&mut mail);

    //     println!("{:?}: {:?}", sat, msg);
    // }
}
