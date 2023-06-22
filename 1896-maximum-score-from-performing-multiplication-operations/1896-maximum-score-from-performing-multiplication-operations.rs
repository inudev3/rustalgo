impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();
        let mut dp = vec![vec![-1;n];m];
        //dp[i][j] = i번째 operation을 front가 j번째 인 경우에 했을 때의 최대 점수
        solve(0,0,&mut dp, &nums,&multipliers)
    }
}
fn solve(front:usize, idx:usize, memo:&mut Vec<Vec<i32>>, nums:&Vec<i32>, multipliers:&Vec<i32>)->i32{
     if idx==multipliers.len(){
        return 0
    }
    if memo[idx][front]!=-1{
        return memo[idx][front]
    }
    
    memo[idx][front]=std::cmp::max(
        multipliers[idx]*nums[front]+solve(front+1,idx+1,memo,nums,multipliers),
        multipliers[idx]*nums[nums.len()-1-(idx-front)]+solve(front,idx+1,memo,nums,multipliers)
    );
    memo[idx][front]
}