use std::collections::HashMap;
use std::cmp::max;
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut score = HashMap::new();
        let mut maxnum=0;
        nums.iter().for_each(|x| {
            score.insert(*x, *score.get(x).unwrap_or(&0)+ *x);
            maxnum = max(maxnum, *x);
        });
        let mut dp = vec![0;(maxnum+1) as usize];
        dp[1] = *score.get(&1).unwrap_or(&0);
        for i in 2..maxnum as usize +1{
            dp[i] = max(dp[i-2]+ *score.get(&(i as i32)).unwrap_or(&0), dp[i-1]);
        }
        dp[maxnum as usize]
    }
}
