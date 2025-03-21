#![allow(unused_variables)]
#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64, // Mhz
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl Copy for CubeSat {}
impl Copy for StatusMessage {}

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        CubeSat { id: self.id }
    }
}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

// fn main() {
//     let mut mail = Mailbox { messages: vec![] };
//     let base = GroundStation {};

//     let sat_ids = fetch_sat_ids();

//     for sat_id in sat_ids {
//         let sat = base.connect(sat_id);
//         let msg = Message {
//             to: sat_id,
//             content: String::from("hello"),
//         };
//         base.send(&mut mail, msg);
//     }

//     let sat_ids = fetch_sat_ids();

//     for sat_id in sat_ids {
//         let sat = base.connect(sat_id);

//         let msg = sat.recv(&mut mail);
//         println!("{:?}: {:?}", sat, msg);
//     }
// }

// fn main() {
//     let sat_a = CubeSat { id: 0 };
//     let a_status = check_status(sat_a.clone());
//     println!("a: {:?}", a_status);
//
//     let a_status = check_status(sat_a);
//     println!("a: {:?}", a_status);
// }

fn main() {
    let base: Rc<RefCell<GroundStation>> =
        Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));

    println!("{:?}", base);

    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
    }

    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}
