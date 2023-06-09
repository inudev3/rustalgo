const MOD:i64 = 1_000_000_007;
impl Solution {
    pub fn number_of_ways(num_people: i32) -> i32 {
        let n = (num_people/2) as usize;
        let mut dp = vec![0i64;n+1];
        dp[0]=1;
        for i in 1..=n{
            for j in 0..i{
                dp[i] = (dp[i] + (dp[j]*dp[i-j-1])%MOD)%MOD;
            }
        }
        dp[n] as i32
    }
}