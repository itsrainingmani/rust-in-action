// s -> sign bit      1
// e -> exponent bits 8
// m -> mantissa bits 23
// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
// seeeeeee emmmmmmm mmmmmmmm mmmmmmmm

const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

pub fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff; // retains only the 23 least sig bits via AND mask

    (sign, exponent, fraction)
}

pub fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

pub fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    // Mutable f32 init to 2 ^ -0 : this represents the weight of the implicit 24th bit
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        // Bit mask with the iteration number as the bit allowed ot pass through and assign the result to mask
        let mask = 1 << i;

        // use mask as a filter against the bits from the original number stored as n_bits.
        // when original number's bit at pos i is non-zero, one_at_bit_i will be assigned a non-0 val
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            // Calculate the weight of bit at position i -> 2 * i ^ -23
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (signed_1, exponent, mantissa)
}
