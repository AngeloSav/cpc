use std::{
    cmp,
    collections::{BTreeSet, HashSet},
};

pub struct MinMaxSegmentTree {
    data: Vec<i64>,
    lazy: Vec<Option<i64>>,
    n_leaves: usize,
}

impl MinMaxSegmentTree {
    pub fn new(v: Vec<i64>) -> Self {
        let mut data = vec![i64::MIN; 4 * v.len()];
        let lazy = vec![None; 4 * v.len()];
        Self::build_internal(&v, &mut data, 0, 0, v.len() - 1);

        Self {
            data,
            lazy,
            n_leaves: v.len(),
        }
    }

    fn build_internal(v: &[i64], data: &mut [i64], idx: usize, rs: usize, re: usize) {
        if rs == re {
            data[idx] = v[rs];
        } else {
            let mid = (rs + re) / 2;
            Self::build_internal(v, data, left(idx), rs, mid);
            Self::build_internal(v, data, right(idx), mid + 1, re);
            data[idx] = cmp::max(data[left(idx)], data[right(idx)]);
        }
    }

    pub fn query(&mut self, l: usize, r: usize) -> i64 {
        if l > r {
            panic!("invalid range (l > n)")
        }
        if l > self.n_leaves - 1 || r > self.n_leaves - 1 {
            panic!("Range is out of bounds!")
        }

        self.query_internal(0, l, r, 0, self.n_leaves - 1)
    }

    fn query_internal(&mut self, i: usize, l: usize, r: usize, rs: usize, re: usize) -> i64 {
        if l > r {
            return i64::MIN;
        }

        if l == rs && r == re {
            return self.data[i];
        }

        self.propagate_lazy(i);
        let mid = (rs + re) / 2;
        let left_res = self.query_internal(left(i), l, r.min(mid), rs, mid);
        let right_res = self.query_internal(right(i), l.max(mid + 1), r, mid + 1, re);
        cmp::max(left_res, right_res)
    }

    pub fn update_range(&mut self, l: usize, r: usize, val: i64) {
        if l > r {
            panic!("invalid range (l > n)")
        }
        if l > self.n_leaves - 1 || r > self.n_leaves - 1 {
            panic!("Range is out of bounds!")
        }
        self.update_internal(0, l, r, val, 0, self.n_leaves - 1);
    }

    pub fn update_internal(
        &mut self,
        i: usize,
        l: usize,
        r: usize,
        val: i64,
        rs: usize,
        re: usize,
    ) {
        if l > r {
            return;
        }

        if l == rs && r == re {
            self.data[i] = self.data[i].min(val);
            self.lazy[i] = Some(val.min(self.lazy[i].unwrap_or(i64::MAX)));
            return;
        }

        self.propagate_lazy(i);
        let mid = (rs + re) / 2;
        self.update_internal(left(i), l, r.min(mid), val, rs, mid);
        self.update_internal(right(i), l.max(mid + 1), r, val, mid + 1, re);
        self.data[i] = cmp::max(self.data[left(i)], self.data[right(i)])
    }

    fn propagate_lazy(&mut self, i: usize) {
        if let Some(upd) = self.lazy[i] {
            self.data[left(i)] = self.data[left(i)].min(upd);
            self.lazy[left(i)] = Some(upd.min(self.lazy[left(i)].unwrap_or(i64::MAX)));

            self.data[right(i)] = self.data[right(i)].min(upd);
            self.lazy[right(i)] = Some(upd.min(self.lazy[right(i)].unwrap_or(i64::MAX)));

            self.lazy[i] = None;
        }
    }
}
pub struct IsThereSegmentTree {
    data: Vec<HashSet<usize>>,
    n_leaves: usize,
}

impl IsThereSegmentTree {
    pub fn new(v: Vec<usize>) -> Self {
        let mut data = vec![HashSet::new(); 4 * v.len()];
        Self::build_internal(&v, &mut data, 0, 0, v.len() - 1);

        Self {
            data,
            n_leaves: v.len(),
        }
    }

    fn build_internal(v: &[usize], data: &mut [HashSet<usize>], idx: usize, rs: usize, re: usize) {
        if rs == re {
            data[idx].insert(v[rs]);
        } else {
            let mid = (rs + re) / 2;
            Self::build_internal(v, data, left(idx), rs, mid);
            Self::build_internal(v, data, right(idx), mid + 1, re);
            data[idx].extend(data[left(idx)].clone());
            data[idx].extend(data[right(idx)].clone());
        }
    }

    pub fn query(&self, l: usize, r: usize, k: usize) -> bool {
        if l > r {
            panic!("invalid range (l > n)")
        }
        if l > self.n_leaves - 1 || r > self.n_leaves - 1 {
            panic!("Range is out of bounds!")
        }
        self.query_internal(0, l, r, k, 0, self.n_leaves - 1)
    }

    fn query_internal(&self, i: usize, l: usize, r: usize, k: usize, rs: usize, re: usize) -> bool {
        if l > r {
            return false;
        }

        if l == rs && r == re {
            return self.data[i].contains(&k);
        }

        let mid = (rs + re) / 2;
        let left_res = self.query_internal(left(i), l, r.min(mid), k, rs, mid);
        let right_res = self.query_internal(right(i), l.max(mid + 1), r, k, mid + 1, re);
        left_res || right_res
    }
}

fn left(node: usize) -> usize {
    (node * 2) + 1
}

fn right(node: usize) -> usize {
    (node * 2) + 2
}

#[cfg(test)]
mod tests;
