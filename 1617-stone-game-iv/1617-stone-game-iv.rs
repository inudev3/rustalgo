impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut dp = vec![false;n as usize+1];
        for i in 1..=n as usize{
            let ceiling = (i as f32).sqrt().floor() as usize;
            for j in 1..=ceiling{
                if !dp[i-j*j]{
                    dp[i]=true;
                    break;
                }
            }
        }
        dp[n as usize]
    }
}

