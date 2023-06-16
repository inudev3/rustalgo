impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let mut dp = vec![0f64;n as usize+1];
        dp[0]=1.0;
        let mut s = if k>0{1.0}else{0.0};
        for i in 1..=n as usize{
            dp[i] = s/max_pts as f64;
            if i<k as usize{
                s+=dp[i];
            }
            if i as i32-max_pts>=0 && i as i32-max_pts<k{
                s -= dp[i-max_pts as usize];
            }
        }
        dp[k as usize..].iter().sum()
    }
}