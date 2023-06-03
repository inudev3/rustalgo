use std::cmp::min;
use std::cmp::Ordering::*;
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let h = h as i64;
        let mut left = 1;
        let mut right = piles.iter().max().unwrap().clone();
        let mut res = right;
        //속도가 낮으면 시간이 높고
        //속도가 높으면 시간이 낮음
        while left <= right {
            let k = (right + left) / 2;
            let mut hours: i64 = 0;
            for x in piles.iter() {
                hours += (x / k) as i64;
                if x % k > 0 {
                    hours += 1;
                }
            }
            match hours.cmp(&h) {
                Less | Equal => {
                    res = min(res, k);
                    right = k - 1;
                }
                Greater => {
                    left = k + 1;
                }
            }
        }
        res
    }
}