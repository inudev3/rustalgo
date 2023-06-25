use std::cmp::{max, min};

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut dp = vec![vec![-1; n as usize + 1]; k as usize + 1];
        solve(k as usize, n as usize, &mut dp)
    }
}
fn solve(k: usize, n: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
    if n == 0 || n == 1 {
        return n as i32;
    }
    if k == 1 {
        return n as i32;
    }
    if dp[k][n] != -1 {
        return dp[k][n];
    }

    let mut ans = i32::MAX;
    let mut l = 1;
    let mut r = n;
    let mut temp = 0;

    while l < r {
        let mid = l + (r - l + 1) / 2;
        let left =solve(k - 1, mid - 1, dp);
        let right =solve(k, n - mid, dp);
        temp = 1 + max(left, right);

        if left <= right {
            l = mid;
        } else {
            r = mid - 1;
        }

        ans = min(ans, temp);
    }

    dp[k][n] = ans;
    ans
}