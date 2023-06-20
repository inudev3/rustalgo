impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0;n];2];
        //dp[0][i] - 처음 감소하는 seq i번째 인덱스에서 길이
        dp[0][0] = 1;
        dp[1][0]=1;
        for i in 1..n{
            dp[1][i]=if nums[i]>nums[i-1]{
                dp[0][i-1]+1                
            }else{               
                dp[1][i-1]
            };
             dp[0][i] = if nums[i]<nums[i-1]{
                dp[1][i-1]+1
             }else{
                dp[0][i-1]
             };
        }
        std::cmp::max(dp[1][n-1],dp[0][n-1])
    }
}