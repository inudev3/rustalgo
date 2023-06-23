impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let mut dp = vec![false;n +1];
        for i in 0..=n{
            if dp[i]{
                continue;
            }
            for j in 1..{
                if i+j*j>n{
                    break;
                }
                dp[i+j*j]=true;
            }
        }
        dp[n]
    }
}
