impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let n = s1.len();
        let mut dp = vec![vec![vec![false;n];n];n+1];
        for i in 0..n{
            for j in 0..n{
                dp[1][i][j] = s1[i]==s2[j];
            }
        }
        for len in 2..=n{
            for i in 0..=n-len{
                for j in 0..=n-len{
                    for newlen in 1..len{
                        dp[len][i][j] |= dp[newlen][i][j] && dp[len-newlen][i+newlen][j+newlen];
                        dp[len][i][j] |= dp[newlen][i][j+len-newlen] && dp[len-newlen][i+newlen][j];
                    }
                }
            }
        }
        dp[n][0][0]
    }
}
