impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut lo=*weights.iter().max().unwrap_or(&1);
        let mut hi = 25*1000000;
        while lo<hi{
            let mid = lo+(hi-lo)/2;
            
            let (cnt, _)=weights.iter().fold((1,0),|(cnt,sum),&x|{
               if sum+x>mid{
                   (cnt+1,x)
               }else{
                   (cnt,sum+x)
               }
            });
            //용량을 줄이면 날짜가 늘고
            //용량을 늘리면 날짜가 줌
            if cnt<=days{
                hi=mid;
            }else{
                lo=mid+1;
            }
        }
        return hi
    }
}