impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        let mut dp = vec![vec![-1;n];n];
        solve(0,n-1,0,&piles,&mut dp)>0
    }
}
fn solve(l:usize,r:usize,turn:usize,piles:&Vec<i32>, memo:&mut Vec<Vec<i32>>)->i32{
    if l>=r{
        return 0
    }
    if memo[l][r]!=-1{
        return memo[l][r]
    }
    memo[l][r] = if turn%2==0{
        std::cmp::max(
            solve(l+1,r,turn+1,piles,memo)+piles[l],
            solve(l,r-1,turn+1,piles,memo)+piles[r]
        )
    }else{
        std::cmp::min(
            solve(l+1,r,turn+1,piles,memo)-piles[l],
            solve(l,r-1,turn+1, piles,memo)-piles[r]
        )
    };
    memo[l][r]
}