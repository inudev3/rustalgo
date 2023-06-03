impl Solution {
    pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
        let mut lo=1;
        let mut hi= 1e9 as i32;
        //최소 sweetness의 최대
        while lo<hi{
            let mid = lo+(hi-lo+1)/2;
            let (cnt,_)=sweetness.iter().fold((0,0),|(cnt,sum),&x|{
                if sum+x>=mid{
                    (cnt+1,0)
                }else{
                    (cnt,sum+x)
                }
            });
            if cnt<k+1{
                hi=mid-1;
            }else{
                lo=mid;
            }
        }
        lo
    }
}