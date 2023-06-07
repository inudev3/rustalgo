use std::cmp::min;
impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let m = s1.len();
        let n = s2.len();
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut memo =vec![vec![0;n+1];m+1];
        for i in (0..m).rev(){
            memo[i][n] = memo[i+1][n]+s1[i] as i32;
        }
        for i in (0..n).rev(){
            memo[m][i] = memo[m][i+1]+s2[i] as i32;;
        }
        solve(s1,s2,0,0,&mut memo)
    }
}
fn solve(s1:&[u8],s2:&[u8],i:usize,j:usize,memo:&mut Vec<Vec<i32>>)->i32{
    if i == s1.len() || j == s2.len() {
        return memo[i][j]
    }
    if memo[i][j]!=0{
        return memo[i][j]
    }
    memo[i][j]= if s1[i]==s2[j]{
        solve(s1,s2,i+1,j+1,memo)
    }else{
        min(solve(s1,s2,i+1,j,memo)+s1[i] as i32, solve(s1,s2,i,j+1,memo)+s2[j] as i32)
    };
    memo[i][j]
}
