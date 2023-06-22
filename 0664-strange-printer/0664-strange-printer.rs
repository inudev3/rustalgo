impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![-1;n];n];
        solve(s,0,n-1,&mut dp)+1
    }
}
fn solve(s:&[u8], l:usize,r:usize, memo:&mut Vec<Vec<i32>>)->i32{
    if memo[l][r]!=-1{
        return memo[l][r]
    }
    let mut found= -1;
    let mut res = s.len() as i32;
    for i in l..r{
        if s[i]!=s[r]&& found==-1{
            found=i as i32;
        }
        if found!=-1{
            res = std::cmp::min(res,1+solve(s,found as usize,i,memo)+solve(s,i+1,r,memo));            
        }
    }
    if found==-1{
        res=0;
    }
    memo[l][r]=res;
    res
}