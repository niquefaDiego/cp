// ----- Union-Find / Disjoint Set Union (DSU) -----
pub struct DSet {
    pub p: Vec<usize>,
    pub r: Vec<usize>,
}

impl DSet {
    pub fn new(n: usize) -> DSet {
        DSet {
            p: (0..n).collect(),
            r: vec![0; n],
        }
    }

    pub fn root(&mut self, i: usize) -> usize {
        if self.p[i] != i {
            self.p[i] = self.root(self.p[i]);
        }
        return self.p[i];
    }

    pub fn merge(&mut self, a: usize, b: usize) -> bool {
        let a = self.root(a);
        let b = self.root(b);
        if a == b {
            return false;
        }
        if self.r[a] > self.r[b] {
            self.p[b] = a;
        } else {
            self.p[a] = b;
            if self.r[a] == self.r[b] {
                self.r[b] += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disjoint_set() {
        let mut ds = DSet::new(10);
        ds.merge(0, 2);
        ds.merge(6, 8);
        ds.merge(2, 4);
        ds.merge(4, 6);
        assert_eq!(ds.root(0), ds.root(8));
        assert_eq!(ds.root(0), ds.root(6));
        assert_eq!(ds.root(0), ds.root(4));
        assert_eq!(ds.root(0), ds.root(2));
        assert_eq!(1, ds.root(1));
        assert_eq!(3, ds.root(3));
        assert_eq!(5, ds.root(5));
        assert_eq!(7, ds.root(7));
        assert_eq!(9, ds.root(9));
        let root0 = ds.root(0);
        assert_eq!(ds.r[root0], 2);
    }
}
