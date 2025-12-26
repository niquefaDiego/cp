pub struct GenericSieve<T, const N: usize> {
    pub is_p: Vec<bool>,
    pub primes: Vec<T>,
    pub div: Vec<T> // divisor[0] = divisor[1] = 0, otherwise divisor[i] > 0
}

impl<T, const N: usize> GenericSieve<T, N> where T :
Copy + Ord + std::fmt::Debug
+ TryFrom<usize> + TryInto<usize>
+ std::ops::Div<T, Output=T> 
+ std::ops::Rem<T, Output=T> 
+ std::ops::Mul<T, Output=T>,
<T as TryFrom<usize>>::Error: std::fmt::Debug,
<T as TryInto<usize>>::Error: std::fmt::Debug
{
    pub fn new() -> GenericSieve<T,N> {
        let mut is_p = vec![true; N];
        let mut primes = vec![];
        let mut div = vec![0.try_into().unwrap(); N];
        is_p[0] = false; is_p[1] = false;
        let mut i: usize = 2;
        while i*i < N {
            if is_p[i] {
                for j in (i*i..N).step_by(i) {
                    is_p[j] = false;
                    if div[j] == 0.try_into().unwrap() { div[j] = i.try_into().unwrap(); }
                }
            }
            i += 1;
        }
        for i in 0..N {
            if is_p[i] {
                primes.push(i.try_into().unwrap());
                div[i] = i.try_into().unwrap();
            }
        }
        GenericSieve { is_p, primes, div }
    }

    fn factorize_small_n(&self, mut n: T, mut pf: Vec<T>) -> Vec<T> {
        debug_assert!(n < (N.try_into().unwrap()));
        while n > 1.try_into().unwrap() {
            let d: T = self.div[n.try_into().unwrap()];
            pf.push(d);
            n = n / d;
        }
        pf
    }

    /// factorize(120) = [2, 2, 2, 3, 5]
    /// Result is sorted
    pub fn factorize(&self, mut n: T) -> Vec<T> {
        let mut pf: Vec<T> = Vec::new();
        if n < N.try_into().unwrap() { return self.factorize_small_n(n, pf); }
        for p in &self.primes {
            if p.clone()*p.clone() > n { break; }
            while n % p.clone() == 0.try_into().unwrap() {
                pf.push(p.clone());
                n = n / p.clone();
                if n < N.try_into().unwrap() { return self.factorize_small_n(n, pf); }
            }
        }
        if n > 1.try_into().unwrap() { pf.push(n); }
        pf
    }

    fn divisor_rec(&self, i: usize, mut x: T, pf: &Vec<T>, d: &mut Vec<T>) {
        if i >= pf.len() { d.push(x); return; }
        let mut j = i+1;
        while j < pf.len() && pf[i] == pf[j] { j += 1 };
        self.divisor_rec(j, x, pf, d);
        for _ in 1..=(j-i) {
            x = x * pf[i];
            self.divisor_rec(j, x, pf, d);
        }
    }

    /// divisors(n) = [1, 5, 3, 15, 2, 10, 6, 30, 4, 20, 12, 60, 8, 40, 24, 120]
    pub fn divisors(&self, n: T) -> Vec<T> {
        let pf = self.factorize(n);
        let mut divs = vec![];
        self.divisor_rec(0, 1.try_into().unwrap(), &pf, &mut divs);
        divs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sieve() {
        let sieve: GenericSieve<u32, 100> = GenericSieve::new();
        let expected_is_p = [false, false, true, true, false, true, false, true, false, false];
        let expected_primes = vec![2, 3, 5, 7, 11, 13, 17];
        let expected_div = [0, 0, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2, 3];
        assert_eq!(expected_is_p, sieve.is_p[0..expected_is_p.len()]);
        assert_eq!(expected_primes, sieve.primes[0..expected_primes.len()]);
        assert_eq!(expected_div, sieve.div[0..expected_div.len()]);
    }
}

