use std::collections::HashMap;
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut dp = HashMap::new();
        let mut ans = 1;
        for &num in &arr{
            let cnt= *dp.get(&(num-difference)).unwrap_or(&0);
            dp.insert(num,cnt+1);
            ans = std::cmp::max(ans,cnt+1);
        }
        ans
    }
}
