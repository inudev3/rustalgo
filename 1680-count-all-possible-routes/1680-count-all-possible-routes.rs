impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        //fuel: abs(locations diff)
        let n = locations.len();
        let mut dp = vec![vec![-1;fuel as usize+1];n];
        solve(&locations,start as usize,finish as usize,fuel,&mut dp)
    }
}
const MOD:i32 = 1_000_000_007;
fn solve(loc:&Vec<i32>,i:usize,j:usize,rem:i32,memo:&mut Vec<Vec<i32>>)->i32{
    if rem<0{
        return 0
    }
    if memo[i][rem as usize]!=-1{
        return memo[i][rem as usize]
    }
    let mut ans = if i==j{1}else{0};
    for l in 0..loc.len(){
        if l!=i{
            let used = (loc[l]-loc[i]).abs();
            ans = (ans+solve(loc,l,j,rem-used,memo))%MOD;
        }
    }
    memo[i][rem as usize]=ans;
    ans
}