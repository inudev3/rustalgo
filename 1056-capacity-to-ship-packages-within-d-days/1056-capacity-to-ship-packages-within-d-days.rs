impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut lo= *weights.iter().max().unwrap_or(&1);
        let mut hi= 25000000;
        while lo<hi {
            let mid = lo+(hi-lo)/2;
            let mut sum = 0;
            let mut cnt = 1;
            for weight in &weights {
                sum += *weight;
                if sum > mid {
                    sum = *weight;
                    cnt += 1;
                }
            }
            if cnt<=days{
                hi=mid;
            }else{
                lo=mid+1;
            }
        }
        hi
    }
}