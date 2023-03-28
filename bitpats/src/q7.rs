/// Q Format is a fixed-point number format that uses a single byte
///
/// Q7 indicates that there are 7 bits available plus 1 sign bit
///
/// We disguise the decimal nature of the type by hiding the 7 bits within an i8
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
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

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        // it's safe to convert from f32 to f64
        // A number that can be represented in 32 bits can also be represented in 64 bits
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> Self {
        // Generally converting from f64 to f32 risks loss of precision
        // But here we only have numbers between -1 and 1 to convert from
        f64::from(n) as f32
    }
}

#[cfg(test)]
mod tests {
    // Brings the parent module within the submodule’s local scope. Items that are marked as pub are accessible here.
    use super::*;
    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7::from(1.));
        assert_eq!(Q7::from(-10.), Q7::from(-1.));
    }

    #[test]
    fn f32_to_q7() {
        let n1: f32 = 0.7;
        let q1 = Q7::from(n1);
        let n2 = -0.4;
        let q2 = Q7::from(n2);
        let n3 = 123.0;
        let q3 = Q7::from(n3);
        assert_eq!(q1, Q7(89));
        assert_eq!(q2, Q7(-51));
        assert_eq!(q3, Q7(127));
    }

    #[test]
    fn q7_to_f32() {
        let q1 = Q7::from(0.7);
        let n1 = f32::from(q1);
        assert_eq!(n1, 0.6953125);
        let q2 = Q7::from(n1);
        let n2 = f32::from(q2);
        assert_eq!(n1, n2);
    }
}
