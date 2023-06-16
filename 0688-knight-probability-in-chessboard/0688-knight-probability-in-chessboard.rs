const dx:[i32;8] = [-2,-1,1,2,2,1,-1,-2];
const dy:[i32;8] = [1,2,2,1,-1,-2,-2,-1];
impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let n = n as usize;
        let mut dp = vec![vec![vec![-1f64;n];n];k as usize+1];
        let (k,row,column) = (k as usize,row as usize,column as usize);
        dp[0][row][column]=1f64;
        let mut ans=0f64;
        for i in 0..n{
            for j in 0..n{
                ans+=solve(k,i,j,&mut dp)
            }
        }
        ans
    }
}
fn solve(rem:usize,i:usize,j:usize,memo:&mut Vec<Vec<Vec<f64>>>)->f64{
    if memo[rem][i][j]!=-1f64{
        return memo[rem][i][j]
    }
    if rem==0{
        return 0f64
    }
    memo[rem][i][j]=0.0;
    let mut ans =0.0;
    for d in 0..8 as usize{
        let nx = i as i32 +dx[d];
        let ny = j as i32 + dy[d];
        if nx>=0 && nx<memo[0].len() as i32 && ny>=0 && ny<memo[0].len() as i32{
            let nx = nx as usize;
            let ny = ny as usize;
            ans += solve(rem-1,nx,ny,memo)/8f64;
        }
    }
    memo[rem][i][j]=ans;
    ans
}
