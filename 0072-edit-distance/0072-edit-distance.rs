
use std::cmp::min;
fn solve(i:usize,j:usize,word1:&[u8], word2:&[u8], memo:&mut Vec<Vec<i32>>)->i32{
    if i==word1.len(){
        return (word2.len()-j) as i32
    }else if j==word2.len(){
        return (word1.len()-i) as i32
    }
    if memo[i][j]!=-1{
        return memo[i][j]
    }
    let ans = if word1[i]==word2[j]{
        solve(i+1,j+1,word1,word2,memo)
    }else{
        min(
            min(solve(i+1,j,word1,word2,memo)/*deleting*/,
        solve(i,j+1,word1,word2,memo)/*inserting*/),
        solve(i+1,j+1,word1,word2,memo)
        )+1/*replacing*/
    };
    memo[i][j]=ans;
    return ans
}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let mut memo = vec![vec![-1;n];m];
        solve(0,0,word1.as_bytes(), word2.as_bytes(), &mut memo)
    }
}


