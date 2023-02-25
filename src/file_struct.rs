#![allow(unused_variables)]
#![allow(dead_code)]
// type File = String;
use rand::prelude::*;

fn one_in(denom: u32) -> bool {
    thread_rng().gen_ratio(1, denom)
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

// fn open(f: File) -> Result<File, String> {
//     if one_in(10000) {
//         let err_msg = String::from("Permission denied.");
//         return Err(err_msg);
//     }

//     Ok(f)
// }

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

// fn close(f: File) -> Result<File, String> {
//     if one_in(10000) {
//         let err_msg = String::from("Permission denied.");
//         return Err(err_msg);
//     }

//     Ok(f)
// }

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

pub fn main() {
    // let mut f1 = File::from("f1.txt");
    let f5_data = vec![114, 117, 115, 116, 33];
    let mut f5 = File::new_with_data("2.txt", &f5_data);

    let mut buffer: Vec<u8> = vec![];

    if f5.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f5 = open(f5).unwrap();
    let f5_length = f5.read(&mut buffer).unwrap();
    f5 = close(f5).unwrap();

    //Converts Vec<u8> to String. Any bytes that aren't valid UTF-8 are replaced with �
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f5);
    println!("{} is {} bytes long", &f5.name, f5_length);
    println!("{}", text);

    // open(&mut f1);
    //read(f1, vec![]);
    // close(&mut f1);
}
