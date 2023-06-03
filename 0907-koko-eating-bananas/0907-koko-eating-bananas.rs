impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let  mut lo=1;
        let  mut hi= *piles.iter().max().unwrap_or(&100_000_000);
        //속도가 올라가면 시간이 줄고
        //속도가 느려지면 시간이 늠
        while lo<hi{
            let mid = lo+(hi-lo)/2;
            let total:i32 = piles.iter().map(|&pile|(pile+mid-1)/mid).sum();
            if total<=h{
                hi=mid;
            }else{
                lo=mid+1;
            }
        }
        hi
    }
}