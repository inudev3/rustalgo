use std::cmp::min;
impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let mut memo =vec![vec![-1;s2.len()];s1.len()];
        solve(s1.as_bytes(), s2.as_bytes(),0,0,&mut memo)
    }
}
fn solve(s1:&[u8],s2:&[u8],i:usize,j:usize,memo:&mut Vec<Vec<i32>>)->i32{
    //
    if i==s1.len(){
        return s2[j..].iter().map(|&x|x as i32).sum()
    }
    if j==s2.len(){
        return s1[i..].iter().map(|&x|x as i32).sum()
    }
    if memo[i][j]!=-1{
        return memo[i][j]
    }
    memo[i][j]= if s1[i]==s2[j]{
        solve(s1,s2,i+1,j+1,memo)
    }else{
        min(solve(s1,s2,i+1,j,memo)+s1[i] as i32, solve(s1,s2,i,j+1,memo)+s2[j] as i32)
    };
    memo[i][j]
}
