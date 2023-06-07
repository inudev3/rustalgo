use std::cmp;
use std::collections::HashSet;
impl Solution{
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_set: HashSet<String> = word_dict.iter().cloned().collect();
        let n = s.len();
        let mut dp = vec![false; n + 1];
        let longest = word_dict.iter().map(|s| s.len()).max().unwrap_or(0);
        dp[0]=true;
        for i in 1..=n{
            for j in (cmp::max(i as i32-longest as i32, 0) as usize..i).rev(){
                //i-1 downto max(i-longest,0)
                if dp[j]{
                    let substr=&s[j..i];
                    if word_set.contains(substr){
                        dp[i]=true;
                        break;
                    }
                }
            }
        }
        dp[n]
    }
}
