const MOD:i32 = 1_000_000_007;
const DX:[i32;4] = [1,-1,0,0];
const DY:[i32;4] = [0,0,1,-1];
impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![-1;n];m];
        let mut ans=0;
        for i in 0..m{
            for j in 0..n{
                ans = (ans+solve(i,j,&grid,&mut dp))%MOD as i64;
            }
        }
        ans as i32
    }
}
fn solve(x:usize,y:usize,grid:&Vec<Vec<i32>>,memo:&mut Vec<Vec<i32>>)->i64{
    if memo[x][y]!=-1{
        return memo[x][y] as i64;
    }
    let mut res = 1i64;
    for k in 0..4{
        let nx = x+DX[k] as usize;
        let ny = y+DY[k] as usize;
        if 0<=nx &&nx<memo.len() && 0<=ny && ny<memo[0].len(){
            if grid[nx][ny]>grid[x][y]{
                res = (res+solve(nx,ny,grid,memo)%MOD as i64);
            }
        }
    }
    memo[x][y] = res as i32;
    res
}