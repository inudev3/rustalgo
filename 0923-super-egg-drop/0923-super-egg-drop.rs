use std::collections::HashMap;

fn solve(memo: &mut HashMap<(usize, usize), i32>, floor: usize, rem: usize) -> i32 {
    if let Some(&val) = memo.get(&(floor, rem)) {
        return val;
    }
    
    let ans = if floor == 0 {
        0
    } else if rem == 1 {
        floor as i32
    } else {
        let mut lo = 1;
        let mut hi = floor;
        while lo + 1 < hi {
            let mid = lo + (hi - lo) / 2;
            let t1 = solve(memo, mid - 1, rem - 1);
            let t2 = solve(memo, floor - mid, rem);
            if t1 < t2 {
                lo = mid;
            } else if t1 > t2 {
                hi = mid;
            } else {
                lo = mid;
                hi=mid;
            }
        }
        1 + std::cmp::min(
            std::cmp::max(solve(memo, lo - 1, rem - 1), solve(memo, floor - lo, rem)),
            std::cmp::max(solve(memo, hi - 1, rem - 1), solve(memo, floor - hi, rem)),
        )
    };

    memo.insert((floor, rem), ans);
    ans
}

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        solve(&mut memo, n as usize, k as usize)
    }
}
