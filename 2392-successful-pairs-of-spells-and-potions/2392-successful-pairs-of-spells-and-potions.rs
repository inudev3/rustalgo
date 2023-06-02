impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let n = spells.len();
        let m = potions.len();
        let mut res :Vec<i32> = vec![0;n];
        let mut sorted : Vec<i64> = potions.iter().map(|&x| x as i64).collect();
        sorted.sort();
        for i in 0..n{
            let bound = (success + (spells[i] as i64 - 1)) / (spells[i] as i64);
            let idx = lower_bound(&sorted, bound);
            res[i] = (m - idx) as i32;
        }
        return res
    }
}

fn lower_bound(arr: &Vec<i64>, target: i64) -> usize {
    let mut lo = 0;
    let mut hi = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] >= target {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    hi
}
