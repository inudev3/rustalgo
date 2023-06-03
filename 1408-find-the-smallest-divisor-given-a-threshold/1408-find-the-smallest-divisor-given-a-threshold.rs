impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut lo = 1;
        let mut hi: i32 = *nums.iter().max().unwrap_or(&1000000);

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            let total: i64 = nums.iter().fold(0, |acc, &x| acc + (x as i64 + mid as i64 - 1) / mid as i64);

            if total <= threshold as i64 {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        } 
        return lo;
    }
}
