// ----- Iterator for adjcent cells in a matrix (Vec<Vec<T>>) -----
const N_DIRS: usize = 4;
const DI: [i32; N_DIRS] = [-1, 1, 0, 0];
const DJ: [i32; N_DIRS] = [0, 0, -1, 1];

#[derive(Debug, Copy, Clone)]
pub struct MatrixAdjIter {
    i: usize,
    j: usize,
    h: usize,
    w: usize,
    iter_index: usize,
}

impl Iterator for MatrixAdjIter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        while self.iter_index < N_DIRS {
            let d = self.iter_index;
            self.iter_index += 1;
            let p = ((self.i as i32) + DI[d], (self.j as i32) + DJ[d]);
            if p.0 >= 0 && p.1 >= 0 {
                let p = (p.0 as usize, p.1 as usize);
                if p.0 < self.h && p.1 < self.w {
                    return Some(p);
                }
            }
        }
        None
    }
}

pub trait VecAdjIterExt<T> {
    fn adj_iter(&self, i: usize, j: usize) -> MatrixAdjIter;
}

impl<T> VecAdjIterExt<T> for Vec<Vec<T>> {
    fn adj_iter(&self, i: usize, j: usize) -> MatrixAdjIter {
        MatrixAdjIter {
            i,
            j,
            h: self.len(),
            w: self[0].len(),
            iter_index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_adj_iter() {
        let matrix = vec![vec![0; 5]; 10];
        let h = matrix.len();
        let w = matrix[0].len();

        // point in the middle
        let iter = matrix.adj_iter(2, 3);
        let mut adj: Vec<(usize, usize)> = iter.into_iter().collect();
        assert_eq!(adj.len(), 4);
        adj.sort();
        assert_eq!(adj, vec![(1, 3), (2, 2), (2, 4), (3, 3)]);

        // try 4 corners
        let iter = matrix.adj_iter(h - 1, w - 1);
        assert_eq!(iter.count(), 2);

        let iter = matrix.adj_iter(0, w - 1);
        assert_eq!(iter.count(), 2);

        let iter = matrix.adj_iter(h - 1, 0);
        assert_eq!(iter.count(), 2);

        let iter = matrix.adj_iter(0, 0);
        assert_eq!(iter.count(), 2);

        // point in the side
        let iter = matrix.adj_iter(2, w - 1);
        assert_eq!(iter.count(), 3);

        let iter = matrix.adj_iter(0, 2);
        assert_eq!(iter.count(), 3);

        let iter = matrix.adj_iter(h - 1, 2);
        assert_eq!(iter.count(), 3);

        let iter = matrix.adj_iter(2, 0);
        assert_eq!(iter.count(), 3);
    }
}
