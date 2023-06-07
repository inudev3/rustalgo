use std::cmp::max;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n= nums.len();
        let mut dp = vec![0;n];
        dp[0] = nums[0];
        if n==1{
            return dp[0]
        }
        dp[1] = max(nums[1], dp[0]);
        for i in 2..n{
            dp[i] = max(dp[i-2]+nums[i], dp[i-1]);
        }
        dp[n-1]
    }
}