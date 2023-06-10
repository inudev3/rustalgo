impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();
        let mut dp = vec![vec![-1;m];m];
        solve(&mut dp, &nums,&multipliers,0,0)
        
    }
}
fn solve(memo:&mut Vec<Vec<i32>>, nums:&Vec<i32>, multipliers:&Vec<i32>, nidx:usize,midx:usize)->i32{
    if midx>=multipliers.len(){
        return 0
    }
    if memo[midx][nidx]!=-1{
        return memo[midx][nidx]
    }
    
    memo[midx][nidx] = std::cmp::max(
        solve(memo, nums,multipliers,nidx+1, midx+1)+nums[nidx]*multipliers[midx],
        solve(memo,nums,multipliers,nidx,midx+1)+nums[nums.len()-1-(midx-nidx)]*multipliers[midx]
    );
    memo[midx][nidx]
}