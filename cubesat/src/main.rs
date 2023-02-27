#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

type Message = String;

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

struct GroundStation;

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

fn main() {
    println!("CubeSat");

    let base = GroundStation {};

    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };
    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, Message::from("hello there!"));
    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);
}
