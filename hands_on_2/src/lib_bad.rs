use std::{default, fmt::Debug, vec};

pub trait MyData: Default + Debug + Copy {}
impl<T> MyData for T where T: Default + Debug + Copy {}

#[derive(Debug, Default)]
pub struct SegmentTree<T, M>
where
    T: MyData,
    M: Fn(&T, &T) -> T,
{
    data: Vec<T>,
    lazy: Vec<Option<T>>,
    merge_fun: M,
    update_fun: M,
    n_leaves: usize,
}

impl<T, M> SegmentTree<T, M>
where
    T: MyData,
    M: Fn(&T, &T) -> T,
{
    pub fn new(v: Vec<T>, merge_fun: M, update_fun: M) -> Self {
        let mut data = vec![T::default(); 4 * v.len()];
        let lazy = vec![None; 4 * v.len()];
        Self::build_internal(&v, &mut data, 0, 0, v.len() - 1, &merge_fun);

        Self {
            data,
            lazy,
            merge_fun,
            update_fun,
            n_leaves: v.len(),
        }
    }

    fn build_internal(v: &[T], data: &mut [T], idx: usize, rs: usize, re: usize, merge_fun: &M) {
        if rs == re {
            data[idx] = v[rs];
        } else {
            let mid = (rs + re) / 2;
            Self::build_internal(&v, data, Self::left(idx), rs, mid, merge_fun);
            Self::build_internal(&v, data, Self::right(idx), mid + 1, re, merge_fun);
            data[idx] = merge_fun(&data[Self::left(idx)], &data[Self::right(idx)]);
        }
    }

    /// Returns the answer for the query in the range [l, r]
    pub fn query(&self, l: usize, r: usize) -> Option<T> {
        assert!(l <= r, "Invalid range! (l > r)");
        assert!(r < self.n_leaves, "Range out of bounds!");
        self.query_internal(0, l, r, 0, self.n_leaves - 1)
    }

    fn query_internal(
        &self,
        i: usize,
        l: usize,
        r: usize,
        node_start_range: usize,
        node_end_range: usize,
    ) -> Option<T> {
        dbg!(
            i,
            node_start_range,
            node_end_range,
            self.data[i],
            "--------------"
        );

        //no overlap
        if node_start_range > r || node_end_range < l {
            return None;
        }

        //total overlap
        if node_start_range >= l && node_end_range <= r {
            return Some(self.data[i]);
        }

        //partial overlap
        let mid = (node_start_range + node_end_range) / 2;
        let l_sol = self.query_internal(Self::left(i), l, r, node_start_range, mid);
        let r_sol = self.query_internal(Self::right(i), l, r, mid + 1, node_end_range);

        match (l_sol, r_sol) {
            (None, None) => None,
            (None, Some(x)) => Some(x),
            (Some(x), None) => Some(x),
            (Some(x), Some(y)) => Some((self.merge_fun)(&x, &y)),
        }
    }

    pub fn update_range(&mut self, l: usize, r: usize, x: T) {}

    pub fn update_range_internal(
        &mut self,
        i: usize,
        l: usize,
        r: usize,
        rs: usize,
        re: usize,
        x: T,
    ) {
        if let Some(upd) = self.lazy[i] {
            self.data[i] = (self.update_fun)(&self.data[i], &upd);
            self.lazy[Self::left(i)] = Some(upd);
            self.lazy[Self::right(i)] = Some(upd);
        }
        // if l > r {
        //     return;
        // }

        // if l == rs && r == re {
        //     self.data[i] = (self.update_fun)(&self.data[i], &x);
        //     self.lazy[i] = Some(x);
        // } else {
        // }
    }

    fn update_lazy_idx(&mut self, idx: usize, upd_to_add: T) {
        if let Some(lazy) = self.lazy[idx] {
            self.lazy[idx] = Some((self.merge_fun)(&lazy, &upd_to_add));
        } else {
            self.lazy[idx] = Some(upd_to_add);
        }
    }

    fn left(node: usize) -> usize {
        (node * 2) + 1
    }

    fn right(node: usize) -> usize {
        (node * 2) + 2
    }
}

/// Returns floor(log_2(x))
fn log2(x: u64) -> u64 {
    x & 1 << (64 - x.leading_zeros() - 1)
}

#[cfg(test)]
mod tests;
