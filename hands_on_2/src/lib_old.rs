use std::{default, fmt::Debug, vec};

pub trait MyData: Default + Debug + Copy {}
impl<T> MyData for T where T: Default + Debug + Copy {}

#[derive(Debug, Default)]
pub struct SegmentTree<T, C>
where
    T: MyData,
    C: Fn(&T, &T) -> T,
{
    data: Vec<T>,
    combine: C,
    n_leaves: usize,
}

impl<T, C> SegmentTree<T, C>
where
    T: MyData,
    C: Fn(&T, &T) -> T,
{
    fn new(v: Vec<T>, combine: C) -> Self {
        let data_len = 2 * v.len() - 1;
        let mut data = vec![T::default(); data_len];
        let log_len = log2(v.len() as u64);
        let split_level = (v.len() % (log_len as usize)) * 2;

        // dbg!(data_len);
        // dbg!(v.len());
        // dbg!(split_level);
        // println!("first part in [{}, {})", (data_len - split_level), "end");
        // println!(
        //     "second part in [{}, {})",
        //     (data_len - split_level - (v.len() - split_level)),
        //     (data_len - split_level)
        // );

        data[(data_len - split_level)..].copy_from_slice(&v[0..split_level]);
        data[(data_len - split_level - (v.len() - split_level))..(data_len - split_level)]
            .copy_from_slice(&v[split_level..]);

        for i in (0..(data_len - split_level - (v.len() - split_level))).rev() {
            data[i] = combine(&data[i * 2 + 1], &data[i * 2 + 2]);
        }

        Self {
            data,
            combine,
            n_leaves: v.len(),
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
            (Some(x), Some(y)) => Some((self.combine)(&x, &y)),
        }
    }

    fn left(node: usize) -> usize {
        node * 2 + 1
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
