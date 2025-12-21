#[derive(Copy, Clone, Debug)]
pub struct Zn<const M: u32>(u32);

/// Return (d, x, y) such that: a*x + b*y = d.
/// And d is the greatest common divisor between a and b.
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (d, y, x) = extended_gcd(b, a%b);
    (d, x, y - ((a/b))*x)
}

impl<const M: u32> Zn<M> {
    pub fn new(x: impl Into<i64>) -> Zn<M> {
        let x: i64 = x.into();
        let m = M as i64;
        Zn((((x%m)+m)%m) as u32)
    }

    pub fn inverse(self) -> Zn<M> {
        let (d, x, _) = extended_gcd(self.0 as i64, M as i64);
        debug_assert!(d == 1);
        Zn::new(x)
    }
}

impl<const M: u32> std::ops::Add<Zn<M>> for Zn<M> {
    type Output = Zn<M>;
    fn add(self, other: Zn<M>) -> Zn<M> { Zn((self.0+other.0)%M) }
}

impl<const M: u32> std::ops::Sub<Zn<M>> for Zn<M> {
    type Output = Zn<M>;
    fn sub(self, other: Zn<M>) -> Zn<M> { Zn((self.0+M-other.0)%M) }
}

impl<const M: u32> std::ops::Mul<Zn<M>> for Zn<M> {
    type Output = Zn<M>;
    fn mul(self, other: Zn<M>) -> Zn<M> {
        Zn(((self.0 as u64)*(other.0 as u64)%(M as u64)) as u32)
    }
}

impl<const M: u32> std::ops::Div<Zn<M>> for Zn<M> {
    type Output = Zn<M>;
    fn div(self, other: Zn<M>) -> Zn<M> { self * other.inverse() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended_gcd() {
        assert_eq!(extended_gcd(0, 3), (3, 0, 1));
        assert_eq!(extended_gcd(7, 0), (7, 1, 0));
        assert_eq!(extended_gcd(10, 20), (10, 1, 0));
        assert_eq!(extended_gcd(18, 30), (6, 2, -1));
    }

    #[test]
    fn test_zn() {
        const MOD: u32 = 1000000007;
        type Z = Zn<MOD>;
        let n10 = Z::new(-10);
        let p12 = Z::new(12);
        let p20 = Z::new(20);
        let zero = Z::new(2*MOD);
        let inv_20 = Z::new(850000006); // 1/20

        //constructor 
        assert_eq!(n10.0, 999999997);
        assert_eq!(p12.0, 12);
        assert_eq!(zero.0, 0);

        // addition
        assert_eq!((n10+p12).0, 2);

        // subtraction
        assert_eq!((n10-p12).0, MOD-22);

        // multiplication
        assert_eq!((n10*p12).0, MOD-120);
        assert_eq!((p20*inv_20).0, 1);

        // division
        assert_eq!((p20/p20).0, 1);
        assert_eq!((p20/n10).0, MOD-2);
        assert_eq!((Z::new(120)/Z::new(6)).0, 20);
    }
}
