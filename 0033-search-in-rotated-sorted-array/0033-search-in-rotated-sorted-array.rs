impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut lo=0;
        let mut hi = n-1;
        while lo<=hi{
            let mid = lo+(hi-lo)/2;
            if nums[mid]==target{
                return mid as i32
            }else if nums[mid]>=nums[lo]{
                if nums[lo]<=target && target<nums[mid]{
                    hi=mid-1;
                }else {
                    lo=mid+1;
                }
            }else {
                if target<=nums[hi] && target>nums[mid]{
                    lo=mid+1;
                }else{
                    hi=mid-1;
                }
            }
        }
        -1
    }

}