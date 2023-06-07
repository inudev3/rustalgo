use std::cmp::max;
fn solve( s: &[u8], i: usize, j: usize, memo:&mut Vec<Vec<i32>>) -> i32 {
    if i>j{
        return 0
    }
    if i==j{
        return 1
    }
    if memo[i][j]!=-1{
        return memo[i][j]
    }
    
    memo[i][j]=if s[i]==s[j]{
        solve(s,i+1,j-1,memo)+2
    }else{
        max(solve(s,i+1,j,memo), solve(s,i,j-1,memo))
    };
    memo[i][j]
}
impl Solution {

    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut memo = vec![vec![-1;n];n];
        solve(bytes, 0,n-1,&mut memo)
    }


}