impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();
        let mut dp = vec![vec![None;n];n];
        solve(&stones, &mut dp, 0, 0)
    }
}
fn solve(stones:&Vec<i32>, memo:&mut Vec<Vec<Option<bool>>>, idx:usize, prevjump:usize) -> bool {
    if memo[idx][prevjump].is_some() {
        return memo[idx][prevjump].unwrap();
    }
    for i in -1..=1 {
        let next_jump = prevjump as i32 + i;
        if next_jump > 0 {
            let next_stone = stones[idx] + next_jump;
            match stones[(idx+1)..].binary_search(&next_stone) {
                Ok(i) => {
                    if solve(stones, memo, idx + i + 1, next_jump as usize) {
                        memo[idx][prevjump] = Some(true);
                        return true
                    }
                },
                _ => continue,
            }
        }
    }
    memo[idx][prevjump] = Some(idx == stones.len() - 1);
    memo[idx][prevjump].unwrap()
}
