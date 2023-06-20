impl Solution {
    pub fn max_a(n: i32) -> i32 {
        let mut dp = vec![0;n as usize+1];
        dp[0]=0;
        dp[1]=1;
        if n==1{
            return dp[1]
        }
        dp[2]=2;

        for i in 3..=n as usize{
            let mut ans= dp[i-1]+1;
            for j in (2..=i-3).rev(){
                ans = std::cmp::max(ans,dp[j]*(i-j-1) as i32);
            }
            dp[i]=ans;
        }
        dp[n as usize]
    }
}