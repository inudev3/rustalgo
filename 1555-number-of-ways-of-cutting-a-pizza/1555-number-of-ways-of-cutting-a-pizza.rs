const MOD:i32 = 1_000_000_007;
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let pizza:Vec<_> = pizza.iter().map(|str|str.as_bytes()).collect();
        let m = pizza.len();
        let n = pizza[0].len();
        let mut dp = vec![vec![vec![0;n];m];k as usize];
        let mut apples = vec![vec![0;n+1];m+1];
        for i in (0..m).rev(){
            for j in (0..n).rev(){
                apples[i][j] = if pizza[i][j]==b'A'{1}else {0}+apples[i+1][j]+apples[i][j+1]-apples[i+1][j+1];  
                dp[0][i][j]=if apples[i][j]>0{1}else{0};      
            }
        }
        for rem in 1..k as usize{
            for i in 0..m{
                for j in 0..n{
                    for next_row in i+1..m{
                        if apples[i][j]-apples[next_row][j]>0{
                            dp[rem][i][j] = (dp[rem][i][j]+dp[rem-1][next_row][j]%MOD)%MOD;
                        }
                    }
                    for next_col in j+1..n{
                        if apples[i][j]-apples[i][next_col]>0{
                            dp[rem][i][j] = (dp[rem][i][j]+dp[rem-1][i][next_col]%MOD)%MOD;
                        }
                    }
                }
            }
        }
        return dp[k as usize-1][0][0]
    }
}