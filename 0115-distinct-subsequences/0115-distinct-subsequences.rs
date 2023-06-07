impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
       let s = s.as_bytes();
       let t = t.as_bytes();
       let m = s.len();
       let n = t.len();
        let mut memo = vec![vec![-1;n+1];m+1];
        solve(s,t,0,0,&mut memo)
    }

}
fn solve(s1:&[u8],s2:&[u8],i:usize,j:usize,memo:&mut Vec<Vec<i32>>)->i32{
    if j==s2.len(){
        return 1
    }
    if i==s1.len(){
        return 0
    }
    if memo[i][j]!=-1{
        return memo[i][j]
    }
    let mut ans = solve(s1,s2,i+1,j,memo);
    if s1[i]==s2[j]{
        ans+=solve(s1,s2,i+1,j+1,memo)
    }
    memo[i][j]=ans;
    ans
}
