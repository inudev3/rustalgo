impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s =s.as_bytes();
        let n = s.len();
        let mut dp = vec![0;n];
        let mut ans=0;
        for i in 1..n{
            if s[i]==b')'{
                if s[i-1]==b'('{
                    dp[i] = if i>=2{dp[i-2]}else{0}+2;
                }else if i as i32>dp[i-1] && s[i-dp[i-1] as usize-1]==b'('{
                    dp[i] = dp[i-1]+if i as i32>=dp[i-1]+2{dp[i-dp[i-1] as usize-2]}else{0}+2;
                }
                ans=std::cmp::max(ans,dp[i]);
            }
        }
        ans
    }
}