impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let n = dist.len();
        let mut lo = 1;
        let mut hi = 1_000_000_0;
        let mut ans = -1;
        //속도를 늘리면 시간이 줄고
        while lo<=hi{
            let mid = lo+(hi-lo)/2;
            let mut sum = dist[0..n-1].iter().fold(0,|acc,&x|{
                acc+((x+mid-1)/mid)
            });
            let sum =  dist[n-1] as f64/mid as f64 + sum as f64;
            if sum<=hour{
                ans=mid;
                hi=mid-1;
            }else{
                lo=mid+1;
            }
        }
        return ans
    }
}