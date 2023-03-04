// s -> sign bit      1
// e -> exponent bits 8
// m -> mantissa bits 23
// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
// seeeeeee emmmmmmm mmmmmmmm mmmmmmmm

pub fn main() {
    let n: f32 = -42.42;

    // should show 00000000000000000000000000000001
    // since n is negative
    isolate_sign_bit(n);

    isolate_exponent_bits(n);
}

fn isolate_sign_bit(n: f32) {
    // Interpret the bits of the f32 as a u32 to allow for bit manipulation
    let n_bits: u32 = n.to_bits();

    // Shift the bits within n 31 places to the right
    // This puts the sign bit in the least significant position
    let sign_bit = n_bits >> 31;

    println!("{:032b}", sign_bit);
}

fn isolate_exponent_bits(n: f32) {
    // Interpret the bits of the f32 as a u32 to allow for bit manipulation
    let n_bits: u32 = n.to_bits();

    // Exponent bits are not right-aligned
    // Shift them 8 bits to the right, overwriting the mantissa
    let exponent = n_bits >> 23;

    // Filter the sign bit using an AND mask
    // only the 8 rightmost bits can pass through
    let exponent = exponent & 0xff;

    // Interpret the remaining bits as a signed integer and subtract the bias as defined by the standard
    let exponent = (exponent as i32) - 127;

    println!("{:08b}", exponent);
}

fn isolate_mantissa_bits(n: f32) {
    let n_bits: u32 = n.to_bits();

    // Mutable f32 init to 2 ^ -0 : this represents the weight of the implicit 24th bit
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        // Bit mask with the iteration number as the bit allowed ot pass through and assign the result to mask
        let mask = 1 << i;

        // use mask as a filter against the bits from the original number stored as n_bits.
        // when original number's bit at pos i is non-zero, one_at_bit_i will be assigned a non-0 val
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            // Calculate the weight of bit at position i -> 2 * i ^ -23
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
}
