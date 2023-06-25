use std::collections::HashMap;
use std::cmp::{min,max};
fn solve(memo: &mut Vec<Vec<i32>>, n: usize, k: usize) -> i32 {
    if n == 0 || n == 1 {
        return n as i32;
    }
    if k == 1 {
        return n as i32;
    }
    if memo[k][n] != -1 {
        return memo[k][n];
    }
    let mut lo = 1;
    let mut hi = n;
    while lo + 1 < hi {
        let mid = lo + (hi - lo) / 2;
        let t1 = solve(memo, mid - 1, k - 1);
        let t2 = solve(memo, n - mid, k);
        if t1 < t2 {
            lo = mid;
        } else if t1 > t2 {
            hi = mid;
        } else {
            lo = mid;
            hi=mid;
        }
    }
    memo[k][n]=1 + min(
        max(solve(memo, lo - 1, k - 1), solve(memo, n - lo, k)),
        max(solve(memo, hi - 1, k - 1), solve(memo, n - hi, k)),
    );
    memo[k][n]
}

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut memo = vec![vec![-1;n as usize+1];k as usize+1];
        solve(&mut memo, n as usize, k as usize)
    }
}