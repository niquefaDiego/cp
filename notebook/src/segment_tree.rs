use std::ops::{Add};

pub struct SegTree<T> {
    fr: usize,
    to: usize,
    value: T,
    left: Option<Box<SegTree<T>>>,
    right: Option<Box<SegTree<T>>>
}

impl<T> SegTree<T>
where T: Clone + Add<Output=T> + Copy + Default
{
    pub fn new(fr: usize, to: usize, initial_value: T) -> SegTree<T> {
        debug_assert!(fr <= to);
        if fr == to {
            return SegTree {fr, to, value: initial_value, left: None, right: None};
        }
        let mid = fr+(to-fr)/2;
        let left = SegTree::new(fr, mid, initial_value);
        let right = SegTree::new(mid+1, to, initial_value);
        let value = left.value + right.value;
        let left = Some(Box::new(left));
        let right = Some(Box::new(right));
        SegTree{fr, to, value, left, right}
    }

    pub fn query(&self, a: usize, b: usize) -> T {
        debug_assert!(self.fr <= a && a <= b && b <= self.to);
        if a == self.fr && b == self.to { return self.value; }
        let left = self.left.as_ref().unwrap();
        let right = self.right.as_ref().unwrap();
        let mut ans = T::default();
        if left.to >= a { ans = ans + left.query(a, left.to); }
        if right.fr <= b { ans = ans + right.query(right.fr, b); }
        ans
    }

    pub fn update(&mut self, i: usize, v: T) {
        debug_assert!(self.fr <= i && i <= self.to); 
        if self.fr == self.to {
            self.value = v;
            return;
        }
        let left = self.left.as_mut().unwrap();
        let right = self.right.as_mut().unwrap();
        if left.to >= i { left.update(i, v); }
        else { right.update(i, v); }
        self.value = left.value + right.value;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seg_tree() {
        let mut stree = SegTree::new(0, 9, 0);
        assert_eq!(0, stree.query(0,9));
        assert_eq!(0, stree.query(4,4));
        assert_eq!(0, stree.query(9,9));
        stree.update(4, 10);
        stree.update(9, 4);
        assert_eq!(14, stree.query(0,9));
        assert_eq!(10, stree.query(4,4));
        assert_eq!(4, stree.query(9,9));
        stree.update(4, 15); // 0 0 0 0 15 0 0 0 0 4
        stree.update(5, 1); // 0 0 0 0 15 1 0 0 0 4
        assert_eq!(20, stree.query(1,9));
        assert_eq!(16, stree.query(2,8));
    }
}
