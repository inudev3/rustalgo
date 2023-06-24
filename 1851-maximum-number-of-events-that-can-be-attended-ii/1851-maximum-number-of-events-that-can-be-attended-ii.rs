impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = events.len();
        events.sort_by_key(|event|event[0]);
        let mut dp = vec![vec![0;k as usize+1];n+1];
        for i in (0..n).rev(){
            let next_idx =events[(i+1)..].partition_point(|arr|arr[0]<=events[i][1])+i+1;
            for j in 1..=k as usize{
                dp[i][j] = std::cmp::max(dp[i+1][j],events[i][2]+dp[next_idx][j-1]);
            }
        }
        dp[0][k as usize]
    }
}