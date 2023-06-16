impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let n = s1.len();
        let mut dp = vec![vec![vec![Option::None;n];n];n+1];
        helper(s1, s2, n, 0, 0, &mut dp)
    }
}
fn helper(s1: &[u8], s2: &[u8], len: usize, i: usize, j: usize, dp: &mut Vec<Vec<Vec<Option<bool>>>>) -> bool {
        if let Some(result) = dp[len][i][j] {
            return result;
        }
        if &s1[i..i+len] == &s2[j..j+len] {
            dp[len][i][j] = Some(true);
            return true;
        }
        let mut cnt = vec![0; 26];
        for k in 0..len {
            cnt[(s1[i+k] - b'a') as usize] += 1;
            cnt[(s2[j+k] - b'a') as usize] -= 1;
        }
        for ch in 0..26 {
            if cnt[ch] != 0 {
                dp[len][i][j] = Some(false);
                return false;
            }
        }
        for newlen in 1..len {
            if (helper(s1, s2, newlen, i, j, dp) && helper(s1, s2, len - newlen, i + newlen, j + newlen, dp)) ||
               (helper(s1, s2, newlen, i, j + len - newlen, dp) && helper(s1, s2, len - newlen, i + newlen, j, dp)) {
                dp[len][i][j] = Some(true);
                return true;
            }
        }
        dp[len][i][j] = Some(false);
        false
    }