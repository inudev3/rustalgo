impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut memo = vec![vec![vec![-1;n];n];m];
        solve(0,0,n as i32-1,&mut memo, &grid)
    }
}
fn solve(i:usize,j1:i32, j2:i32,memo:&mut Vec<Vec<Vec<i32>>>,grid:&Vec<Vec<i32>>)->i32{
    if i==grid.len(){
        return 0
    }
    if memo[i][j1 as usize][j2 as usize]!=-1{
        return memo[i][j1 as usize][j2 as usize]
    }
    let mut ans = grid[i][j1 as usize];
    if j1!=j2{
        ans+=grid[i][j2 as usize];
    }
    let n = grid[0].len();
    let mut Max=0;
    for n1 in i32::max(j1-1,0)..=i32::min(n as i32 -1,j1+1){
        for n2 in i32::max(j2-1,0)..=i32::min(n as i32 -1,j2+1){
            Max = i32::max(Max,  solve(i+1,n1,n2,memo,grid));
        }
    }
    ans+=Max;
    memo[i][j1 as usize][j2 as usize]=ans;
    ans
}
