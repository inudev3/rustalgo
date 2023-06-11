impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        let n = n as usize;            
        cuts.insert(0,0);
        cuts.push(n as i32);
        cuts.sort();
        let mut dp = vec![vec![-1;cuts.len()];cuts.len()];
        solve(&cuts, &mut dp, 0, cuts.len()-1)
    }
}
fn solve(cuts:&Vec<i32>, memo:&mut Vec<Vec<i32>>, l:usize,r:usize)->i32{
    if r-l<=1{
        return 0
    }
    if memo[l][r]!=-1{
        return memo[l][r]
    }
    let mut res = i32::MAX;
    for i in l+1..r{
        res = std::cmp::min(res, cuts[r]-cuts[l]+solve(cuts,memo,l,i)+solve(cuts,memo,i,r));
    }
    memo[l][r]=res;
    res
}