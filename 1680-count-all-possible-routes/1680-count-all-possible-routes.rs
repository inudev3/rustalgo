impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        //fuel: abs(locations diff)
        let n = locations.len();
        let mut dp = vec![vec![-1;fuel as usize+1];n];
        solve(&locations,start,finish,fuel,&mut dp)
    }
}
const MOD:i32 = 1_000_000_007;
fn solve(loc:&Vec<i32>,i:i32,j:i32,rem:i32,memo:&mut Vec<Vec<i32>>)->i32{
  
    if rem<0{
        return 0
    }
    if memo[i as usize][rem as usize]!=-1{
        return memo[i as usize][rem as usize]
    }
    let mut ans= if i==j{1}else {0};
    for k in 0..loc.len(){

        let diff = (loc[k]-loc[i as usize]);
        if i!=(k as i32){
            ans = (ans+solve(loc, k as i32,j,rem-diff.abs(), memo)%MOD)%MOD;
        }
    }
    memo[i as usize][rem as usize]=ans;
    ans
}