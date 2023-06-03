impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut lo=1 ;
        let mut hi:i32 = *nums.iter().max().unwrap()+1;
        while lo<hi{
            let mid = lo+(hi-lo)/2;
            let total:i64=nums.iter().map(|&x|(x+mid-1) as i64/mid as i64).sum();
            if total<=threshold as i64{
                hi=mid;
            }else{
                lo=mid+1;
            }
        } 
        return hi as i32
    }
}