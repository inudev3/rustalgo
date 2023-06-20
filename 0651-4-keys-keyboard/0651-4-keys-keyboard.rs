impl Solution {
    pub fn max_a(n: i32) -> i32 {
        let mut dp = vec![-1;n as usize+1];
        solve(n as usize,&mut dp)
    }
}
fn solve(n:usize, memo:&mut Vec<i32>)->i32{
    if n<=2{
        return n as i32
    }
    if memo[n]!=-1{
        return memo[n]
    }
    let mut ans = solve(n-1,memo)+1;
    for i in (2..=n-3).rev(){
        ans = std::cmp::max(ans,solve(i,memo)*(n-i-1) as i32);
    }
    memo[n]=ans;
    ans
}
