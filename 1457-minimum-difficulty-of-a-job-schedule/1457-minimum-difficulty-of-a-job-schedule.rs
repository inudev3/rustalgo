impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        //하루 difficulty = 개중 최대
        if (n as i32) < d{
            return -1
        }
        let mut dp = vec![vec![-1;d as usize+1];n];
         solve(0,d as usize,&job_difficulty,&mut dp)
    }
}
const MAX_DIFF:i32 = 1001;
fn solve(idx:usize, rem:usize,diff:&Vec<i32>, memo:&mut Vec<Vec<i32>>)->i32{
 
    if memo[idx][rem]!=-1{
        return memo[idx][rem]
    }
    if rem==1{
        let mut res =0;
        for j in idx..diff.len(){
            res = std::cmp::max(res,diff[j]);
        }
        memo[idx][rem]=res;
        return res
    }

    let mut res = i32::MAX;
    let mut maxDiff=0;
    for j in idx..=diff.len()-rem{
        maxDiff = std::cmp::max(maxDiff, diff[j]);
        res = std::cmp::min(res,maxDiff+solve(j+1,rem-1,diff,memo));
    }
    memo[idx][rem]=res;
    res
}