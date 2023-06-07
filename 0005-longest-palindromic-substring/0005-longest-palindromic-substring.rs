impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars:Vec<_> = s.chars().collect();
        let n= chars.len();
        let mut dp = vec![vec![false;n];n];
        for i in 0..n{
            dp[i][i] = true;
        }
        let mut ans = vec![0,0];
        for i in 0..n-1{
            if chars[i]==chars[i+1]{
                dp[i][i+1]=true;
                ans[0] = i;
                ans[1]=i+1;
            }
        }
        for len in 2..n{
            for i in 0..n-len{
                let j = i+len;
                if chars[i]==chars[j]&&dp[i+1][j-1]{
                    dp[i][j]=true;
                    ans[0]=i;
                    ans[1]=j;
                }
            }
        }
        String::from(&s[ans[0]..=ans[1]])
    }
}