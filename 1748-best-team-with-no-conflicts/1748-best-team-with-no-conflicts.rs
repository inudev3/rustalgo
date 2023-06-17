impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut pair:Vec<_> = scores.iter().zip(ages.iter()).collect();
        pair.sort_by(|a,b|{
            let primary = a.1.cmp(b.1);
            let secondary = a.0.cmp(b.0);
            primary.then(secondary)
        });
        let mut memo = vec![vec![-1;scores.len()];scores.len()];
        solve(0,-1,&pair,&mut memo)
    }
}
fn solve(idx:usize, prev:i32, pair:&Vec<(&i32,&i32)>,memo:&mut Vec<Vec<i32>>)->i32{
    if idx>=pair.len(){
        return 0
    }
    if memo[(prev+1) as usize][idx]!=-1{
        return memo[(prev+1) as usize][idx]
    }
    let mut ans = solve(idx+1,prev,pair,memo);
    if prev==-1 || pair[idx].0 >=pair[prev as usize].0{
        ans= std::cmp::max(ans, solve(idx+1,idx as i32,pair,memo)+pair[idx].0);
    }
    memo[(prev+1) as usize][idx]=ans;
    ans
}