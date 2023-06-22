impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        //하루 difficulty = 개중 최대
        if (n as i32) < d{
            return -1
        }
        //dp[i][j] = i번째 날에 j번째 job까지 수행할 경우 최소 diff
        let mut dp = vec![vec![-1;n];d as usize+1];
        solve(0,d as usize, &job_difficulty, &mut dp)
    }
}
const MAX_DIFF:i32 = 1001;
fn solve(idx:usize, rem:usize,diff:&Vec<i32>, memo:&mut Vec<Vec<i32>>)->i32{
    if memo[rem][idx]!=-1{
        return memo[rem][idx]
    }
    if rem==1{
        let mut ans=0;
        for i in idx..diff.len(){
            ans = std::cmp::max(ans,diff[i]);
        }
        memo[rem][idx]=ans;
        return ans
    }
    let mut maxDiff=0;
    let mut ans=i32::MAX;
    for j in idx..=diff.len()-rem{
        maxDiff = std::cmp::max(maxDiff, diff[j]);
        ans = std::cmp::min(ans, maxDiff+solve(j+1,rem-1,diff,memo));
    }
    memo[rem][idx]=ans;
    ans
}