use std::collections::HashMap;
use std::cmp::max;
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut score = HashMap::new();
        let mut maxnum=0;
        for &num in &nums {
            *score.entry(num).or_insert(0) += num;
            maxnum = max(maxnum,num);
        }
        let mut memo = vec![-1;(maxnum+1) as usize];
        solve(&mut memo, &nums, &score, maxnum as usize)
    }
}
fn solve(memo:&mut Vec<i32>, nums:&Vec<i32>,score:&HashMap<i32,i32>,num:usize)->i32{
    if num==0{
        return 0
    }
    if num==1{
        return *score.get(&1).unwrap_or(&0)
    }
    if memo[num]!=-1{
        return memo[num]
    }
    let mut ans=0;
    let gain = score.get(&(num as i32)).unwrap_or(&0);
    ans = max(*gain+solve(memo,nums,score,num-2), solve(memo,nums,score,num-1));
    memo[num]=ans;
    ans
}