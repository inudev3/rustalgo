use std::collections::HashMap;
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut dp = vec![HashMap::new();nums.len()];
        let n = nums.len();
        let mut ans = 0;
        for right in 0..n{
            for left in 0..right{
                let diff=nums[right]-nums[left];
                let count = dp[left].get(&diff).unwrap_or(&1)+1;
                dp[right].insert(diff, count);
                ans = std::cmp::max(ans,count);
            }
        }
        ans
    }
}