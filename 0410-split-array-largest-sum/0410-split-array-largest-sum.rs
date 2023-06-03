impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut lo = *nums.iter().max().unwrap_or(&1);
        let mut hi = nums.iter().sum();
        //최대가 커지면 개수가 줄고
        //최대가 작아지면 개수가 늘어남
        while lo<hi{
            let mid = lo+(hi-lo)/2;
            let (cnt, _) =nums.iter().fold((1,0),|(cnt,sum), &x|{
                if sum+x>mid{
                    (cnt+1, x)
                }else{
                    (cnt, sum+x)
                }
            });
            if cnt<=k{
                hi=mid;
            }else{
                lo=mid+1;
            }
        }
        return hi
    }
}