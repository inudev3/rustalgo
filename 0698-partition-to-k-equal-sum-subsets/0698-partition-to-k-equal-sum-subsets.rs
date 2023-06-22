use std::collections::HashMap;
impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        let total:i32 = nums.iter().map(|x|*x).sum();
           if total%k!=0{
            return false
        }
        let n = nums.len();
        let subsum = total/k;
        let mut dp = vec![-1;1<<n];
        dp[0]=0;
        for mask in 0..1<<n{
            if dp[mask]==-1{
                continue
            }
            for i in 0..n{
                if (mask>>i)&1==0 && dp[mask]+nums[i]<=subsum{
                    dp[mask|(1<<i)] = (dp[mask]+nums[i])%subsum;
                }
            }
            if dp[(1<<n)-1]==0{
                return true
            }
        }
        dp[(1<<n)-1]==0
    }
}
