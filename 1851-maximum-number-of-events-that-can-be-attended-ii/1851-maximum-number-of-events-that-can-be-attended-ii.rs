impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = events.len();
        events.sort_by_key(|event|event[0]);
        let mut dp = vec![vec![-1;n];k as usize+1];
        solve1(k as usize,0,&events,&mut dp)
    }
}
fn solve1(rem:usize,idx:usize, events:&Vec<Vec<i32>>, memo:&mut Vec<Vec<i32>>)->i32{
    if rem==0||idx==events.len(){
        return 0
    }
    if memo[rem][idx]!=-1{
        return memo[rem][idx]
    }
    let mut ans = solve1(rem,idx+1,events,memo);
    let next_idx=events[(idx+1)..].partition_point(|arr|arr[0]<=events[idx][1])+idx+1;
    ans = ans.max(solve1(rem-1,next_idx, events,memo)+events[idx][2]);
    memo[rem][idx]=ans;
    ans
}