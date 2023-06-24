impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut merged = vec![];
        for i in 0..start_time.len(){
            merged.push((start_time[i],end_time[i],profit[i]));
        }
        merged.sort_unstable_by_key(|tuple|tuple.0);
        let n = merged.len();
        let mut dp = vec![-1;n];
        solve2(&merged,&mut dp, 0)
    }
}
fn solve2(schedule:&Vec<(i32,i32,i32)>, memo:&mut Vec<i32>, idx:usize)->i32{
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