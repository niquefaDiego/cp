// Iterator for adjcent cells in a matrix
const N_DIRS: usize = 4;
const DI: [i32; N_DIRS] = [-1, 1, 0, 0 ];
const DJ: [i32; N_DIRS] = [ 0, 0, -1, 1 ];

pub struct MatrixAdjIter { i: usize, j: usize, h: usize, w: usize, iter: usize }

pub fn matrix_adj<T>(a: &Vec<Vec<T>>, i: usize, j: usize) -> MatrixAdjIter {
    MatrixAdjIter { i, j, h: a.len(), w: a[0].len(), iter: 0 }
}

impl Iterator for MatrixAdjIter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        while self.iter < N_DIRS {
            let d = self.iter;
            self.iter += 1;
            let p = ((self.i as i32) + DI[d], (self.j as i32) + DJ[d]);
            if p.0 < 0 || p.1 < 0 { continue; }
            let p = (p.0 as usize, p.1 as usize);
            if p.0 >= self.h || p.1 >= self.w { continue; }
            return Some(p);
        }
        return None;
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
        let iter = matrix_adj(&matrix, 2, 3);
        let mut adj: Vec<(usize,usize)> = iter.collect();
        assert_eq!(adj.len(), 4);
        adj.sort();
        assert_eq!(adj, vec![(1, 3), (2,2), (2,4), (3,3)]);

        // try 4 corners
        let iter = matrix_adj(&matrix, h-1, w-1);
        assert_eq!(iter.count(), 2);

        let iter = matrix_adj(&matrix, 0, w-1);
        assert_eq!(iter.count(), 2);

        let iter = matrix_adj(&matrix, h-1, 0);
        assert_eq!(iter.count(), 2);

        let iter = matrix_adj(&matrix, 0, 0);
        assert_eq!(iter.count(), 2);

        // point in the side
        let iter = matrix_adj(&matrix, 2, w-1);
        assert_eq!(iter.count(), 3);

        let iter = matrix_adj(&matrix, 0, 2);
        assert_eq!(iter.count(), 3);

        let iter = matrix_adj(&matrix, h-1, 2);
        assert_eq!(iter.count(), 3);

        let iter = matrix_adj(&matrix, 2, 0);
        assert_eq!(iter.count(), 3);
    }
}
