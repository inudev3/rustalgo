impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n  = stone_value.len();
        let mut dp = vec![-MAX;n];        
        let res = solve(0, &stone_value, &mut dp);
        let res = match res{
            i if i>0=> "Alice",
            i if i<0=> "Bob",
            _=> "Tie"
        };
        res.to_string()
    }
}
const MAX:i32 = 50000001;
fn solve(idx:usize, stones:&Vec<i32>, memo:&mut Vec<i32>)->i32{
    if idx>= stones.len(){        
        return 0
    }
    if memo[idx]!=-MAX{
        return memo[idx]
    }
    let mut ans = 0;
    for j in idx..std::cmp::min(idx+3, stones.len()){
        ans+=stones[j];
        memo[idx]= std::cmp::max(memo[idx],ans-solve(j+1,stones,memo));
    }
    memo[idx]
}