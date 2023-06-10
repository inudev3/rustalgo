impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();
        let mut dp = vec![vec![-1;n];n];
        solve(&stones, &mut dp, 0,0)==1
    }
}
fn solve(stones:&Vec<i32>,memo:&mut Vec<Vec<i32>>, idx:usize, prevjump:usize)->i32{
    if memo[idx][prevjump]!=-1{
        return memo[idx][prevjump]
    }
    for i in -1..=1  as i32{
        let next_jump = prevjump as i32 + i;
        if next_jump > 0 {
            let next_stone = stones[idx] + next_jump;
            match stones[(idx+1)..].binary_search(&next_stone) {
                Ok(i) => {
                    if solve(stones, memo, idx + i + 1, next_jump as usize) == 1 {
                        memo[idx][prevjump] = 1;
                        return 1
                    }
                },
                _ => continue,
            }
        }
    }
    memo[idx][prevjump] = if idx==stones.len()-1{1}else{0};
    return memo[idx][prevjump]
}