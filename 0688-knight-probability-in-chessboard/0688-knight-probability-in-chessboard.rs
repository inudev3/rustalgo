const dx:[i32;8] = [-2,-1,1,2,2,1,-1,-2];
const dy:[i32;8] = [1,2,2,1,-1,-2,-2,-1];
impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let n = n as usize;
        let mut dp = vec![vec![vec![0f64;n];n];k as usize+1];
        dp[0][row as usize][column as usize] = 1f64;
        for m in 1..=k as usize{
            for i in 0..n{
                for j in 0..n{
                    for d in 0..8 as usize{
                        let nx = i as i32+dx[d];
                        let ny = j as i32+dy[d];
                        if nx>=0 && nx<n as i32 && ny>=0 && ny<n as i32{ 
                            let nx = nx as usize;
                            let ny = ny as usize;
                            dp[m][i][j] += dp[m-1][nx][ny]/8f64;
                        }
                    }
                }
            }
        }
        let mut ans = 0f64;
        for i in 0..n{
            for j in 0..n{
                ans+=dp[k as usize][i][j];
            }
        }
        ans
    }
}
