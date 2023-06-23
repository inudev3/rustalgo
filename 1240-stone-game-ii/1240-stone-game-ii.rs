impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut dp = vec![vec![-MAX; n/2+1]; n];
        let mut suffix = vec![0; n+1];
        
        for i in (0..n).rev() {
            suffix[i] = suffix[i+1] + piles[i];
        }
        
        solve(0, &suffix, &piles, 1, &mut dp)
    }
}

const MAX :i32 = 1000001;

fn solve(idx: usize, suffix: &Vec<i32>, piles: &Vec<i32>, M: usize, memo: &mut Vec<Vec<i32>>) -> i32{
    if idx + 2*M >= piles.len() {
        return suffix[idx];
    }
    if memo[idx][M] != -MAX {
        return memo[idx][M];
    }
    let mut max_score = -MAX;
    let mut take = 0;
    for j in idx..std::cmp::min(piles.len(), idx + 2*M) {
        take += piles[j];
        let next_score = suffix[j+1] - solve(j+1, suffix, piles, std::cmp::max(M, j - idx + 1), memo);
        max_score = std::cmp::max(max_score, take + next_score);
    }
    memo[idx][M] = max_score;
    max_score
}
