pub mod decode;
pub mod scanmem;
pub mod virtualmem;

use std::mem::size_of;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

// Reference - Signal that the Rust compiler will provide safety guarantees
// Pointer - Refer to something more primitive. Implication that we are responsible for safety
// Raw Pointer - used for types where it's important to make their unsafe nature explicit

fn main() {
    let a: usize = 42;

    let b: &[u8; 10] = &B;

    // Boxed byte slice. Placing a value inside the box moves ownership of the value to the owner of the box
    let c: Box<[u8]> = Box::new(C);

    // println!("a: {}, b: {:p}, c: {:p}", a, b, c);
    println!("a (an unsigned integer):");
    println!("  location: {:p}", &a);
    println!("  size:     {:?} bytes", size_of::<usize>());
    println!("  value:    {:?}", a);
    println!();
    println!("b (a reference to B):");
    println!("  location:  {:p}", &b);
    println!("  size:      {:?} bytes", size_of::<&[u8; 10]>());
    println!("  points to: {:p}", b);
    println!();
    println!("c (a \"box\" for C):");
    println!("  location:  {:p}", &c);
    println!("  size:      {:?} bytes", size_of::<Box<[u8]>>());
    println!("  points to: {:p}", c);
    println!();
    println!("B (an array of 10 bytes):");
    println!(" location: {:p}", &B);
    println!(" size: {:?} bytes", size_of::<[u8; 10]>());
    println!(" value: {:?}", B);
    println!();
    println!("C (an array of 11 bytes):");
    println!(" location: {:p}", &C);
    println!(" size: {:?} bytes", size_of::<[u8; 11]>());
    println!(" value: {:?}", C);

    // decode::main();

    // let z: i64 = 42;
    // let z_ptr = &z as *const i64; // Rust's pointer types always point to the starting byte of T
    // let z_addr: usize = unsafe { std::mem::transmute(z_ptr) };

    // println!("z: {} ({:p}...0x{:x})", z, z_ptr, z_addr + 7);

    virtualmem::main();
    scanmem::main();
}
