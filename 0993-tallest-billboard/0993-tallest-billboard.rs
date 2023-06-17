use std::collections::HashMap;
impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let mut dp = HashMap::new();
        dp.insert(0,0);
        for &rod in &rods{
            let mut newdp = dp.clone();
            for (&diff,&taller) in &dp{
                let shorter = taller-diff;
                let newTall = *newdp.get(&(diff+rod)).unwrap_or(&0);
                newdp.insert(diff+rod,std::cmp::max(newTall,taller+rod));
                let diff2 = (shorter-taller+rod).abs();
                let newTall2 = std::cmp::max(shorter+rod,taller);
                newdp.insert(diff2,std::cmp::max(newTall2,*newdp.get(&diff2).unwrap_or(&0)));
            }
            dp = newdp;
        }
        *dp.get(&0).unwrap_or(&0)
    }
}
