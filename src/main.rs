#![allow(unused_imports)]
extern crate ria;

use ria::{add_traits, file_struct};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Rust in Action");

    // add_traits::main();
    file_struct::main();

    Ok(())
}
