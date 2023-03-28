/// Q Format is a fixed-point number format that uses a single byte
///
/// Q7 indicates that there are 7 bits available plus 1 sign bit
///
/// We disguise the decimal nature of the type by hiding the 7 bits within an i8
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Q7(i8);

impl From<F64> for Q7 {
    fn from(n: f64) -> Self {
        // assert!(n >= -1.0);
        // assert!(n <= 1.0);
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> f64 {
        (n.0 as f64) * 2_f64.powf(-7.0)
    }
}
