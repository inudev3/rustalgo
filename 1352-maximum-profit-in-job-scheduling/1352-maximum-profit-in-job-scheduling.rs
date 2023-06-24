impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut merged :Vec<(i32,i32,i32)>=start_time.into_iter().zip(end_time.into_iter()).zip(profit.into_iter()).map(|((a,b),c)|(a,b,c)).collect();
        merged.sort_unstable_by_key(|k|k.0);
        let n = merged.len();
        let mut dp = vec![-1;n];
        solve2(&merged,&mut dp, 0)
    }
}
fn solve2(schedule:&[(i32,i32,i32)], memo:&mut [i32], idx:usize)->i32{
    if idx==schedule.len(){
        return 0
    }
    if memo[idx]!=-1{
        return memo[idx]
    }
    let mut ans = solve2(schedule,memo,idx+1);
    let next_idx = schedule[(idx+1)..].partition_point(|tup|tup.0<schedule[idx].1)+idx+1;
    ans = ans.max(solve2(schedule,memo,next_idx)+schedule[idx].2);
    memo[idx]=ans;
    ans
}
