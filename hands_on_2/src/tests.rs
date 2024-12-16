use std::cmp::max;

use crate::MinMaxSegmentTree;

#[test]
fn playground() {
    //prefix sums
    let arr = vec![1, 0, 1, 1, 0, -1, 0, -1, 0];
    let p = arr
        .iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<_>>();

    println!("{:?}", p);
}

#[test]
fn playground2() {
    //prefix sums

    println!("{:?}", None.min(Some(3)));
}
