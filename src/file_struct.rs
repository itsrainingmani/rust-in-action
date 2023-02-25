//! Simulating files one step at a time

#![allow(unused_variables)] // Crate level attributes
#![allow(dead_code)]
use std::fmt;
use std::fmt::Display;

use rand::prelude::*;

fn one_in(denom: u32) -> bool {
    thread_rng().gen_ratio(1, denom)
}

// Traits enable the compiler and others to know that multiple types are attempting to perform the same task. Allowing multiple types to implement the a Read trait enables code re-use and allows the Rust compiler to perform its zero cost abstraction wizardry
trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

/// Represents a "file",
/// which probably lives on a file system
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    /// New files are assumed to be empty, but a name is required
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    /// Returns the file's length in bytes
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the file's name
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Read for File {
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

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
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
    println!("{}", f5); // This will use the impl of Display that we just wrote
    println!("{} is {} bytes long", &f5.name, f5_length);
    println!("{}", text);

    // open(&mut f1);
    //read(f1, vec![]);
    // close(&mut f1);
}
