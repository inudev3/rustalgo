const MOD:i32= 1_000_000_007;
impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let sum:i32 = profit.iter().map(|x|*x).sum();
        let mut dp = vec![vec![vec![-1;sum as usize+1];n as usize+1];group.len()];
        solve(min_profit,&profit,&group,&mut dp, n as usize,0,0)
    }
}
//profit[i], group[i]
fn solve(min_profit:i32,profit:&Vec<i32>,group:&Vec<i32>, memo:&mut Vec<Vec<Vec<i32>>>, rem:usize,idx:usize,sum:usize)->i32{
     if idx>= profit.len(){
        return if sum as i32>=min_profit{1} else {0}
    }
    if memo[idx][rem][sum]!=-1{
        return memo[idx][rem][sum]
    }

   
    let mut ans = solve(min_profit,profit,group,memo,rem,idx+1,sum)%MOD;
    if rem >=group[idx] as usize{
        ans = (ans+solve(min_profit,profit,group,memo,rem-group[idx] as usize,idx+1,sum+profit[idx] as usize)%MOD)%MOD;
    }
    memo[idx][rem][sum]=ans;
    ans
}