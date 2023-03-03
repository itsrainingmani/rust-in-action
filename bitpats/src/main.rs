fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    // Same bit pattern but different numbers
    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);

    let a: f32 = 42.42;
    let frankentype: u32 = unsafe {
        // Shreds type safety
        std::mem::transmute(a)
    };

    println!("{}", frankentype);

    // Invokes the std::fmt::Binary trait
    println!("{:032b}", frankentype); // left pad with 32 0s

    let b: f32 = unsafe { std::mem::transmute(frankentype) };

    println!("{}", b);
    assert_eq!(a, b);

    // int_overflow();
    // u16_bit_patterns();

    // in order to get 200 + 200 = 144
    // you have to compile with the -O flag
    // arith_overflow();

    // Depending on the hardware this will most probably display -
    // -573785174 vs -1430532899
    // or
    // -1430532899 vs -573785174
    endianness();
}

fn endianness() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { std::mem::transmute(big_endian) };
    let b: i32 = unsafe { std::mem::transmute(little_endian) };

    println!("{} vs {}", a, b);
}

#[allow(arithmetic_overflow)]
fn arith_overflow() {
    let (a, b) = (200, 200);

    // without the type dec, Rust won't assume that you're trying to create an impossible situation
    let c: u8 = a + b;
    println!("200 + 200 = {}", c);
}

fn u16_bit_patterns() {
    let zero: u16 = 0b0000_0000_0000_0000;
    let one: u16 = 0b0000_0000_0000_0001;
    let two: u16 = 0b0000_0000_0000_0010;

    let six_533: u16 = 0b1111_1111_1111_1101;
    let six_534: u16 = 0b1111_1111_1111_1110;
    let six_535: u16 = 0b1111_1111_1111_1111;

    print!("{}, {}, {}, ..., ", zero, one, two);
    println!("{}, {}, {}", six_533, six_534, six_535);
}

fn int_overflow() {
    let mut i: u16 = 0;

    print!("{}..", i);

    loop {
        i += 1000;
        print!("{}..", i);
        if i % 10000 == 0 {
            print!("\n");
        }
    }
}
