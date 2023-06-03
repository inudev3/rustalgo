impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lo=0;
        let mut hi = n-1;
        if n==1||nums[0]<nums[hi]{
            return nums[0]
        }
        while lo<=hi{
            let mid = lo+(hi-lo)/2;
            if nums[mid]>nums[mid+1]{
                return nums[mid+1]
            }else if nums[mid]<nums[mid-1]{
                return nums[mid]
            } else{
                if nums[mid]>nums[lo]{
                    lo=mid+1;
                }else{
                    hi=mid-1;
                }
            }
        }
         -1
    }
}