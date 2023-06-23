impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![vec![-1;n];n];
        solve(0,n-1,&mut dp, &nums)>=0
    }
}
fn solve(l:usize,r:usize,memo:&mut Vec<Vec<i32>>, nums:&Vec<i32>)->i32{
      if memo[l][r]!=-1{
        return memo[l][r]
    }
    if l>=r{
        return nums[l]
    }
  
    memo[l][r]=std::cmp::max(
        nums[l]-solve(l+1,r,memo,nums),
        nums[r]-solve(l,r-1,memo,nums)
    );
    memo[l][r]
}