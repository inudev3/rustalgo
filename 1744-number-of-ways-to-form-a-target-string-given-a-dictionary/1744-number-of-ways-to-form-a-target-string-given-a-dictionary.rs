const MOD:i32 = 1_000_000_007;
impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let n = words.len();
        let words :Vec<_>=words.iter().map(|word|word.as_bytes()).collect();
        let tLen = target.len();
        let wLen = words[0].len();
        let mut freq = vec![vec![0;wLen];26]; 
        for i in 0..wLen{
            for word in &words{
                freq[(word[i]-b'a') as usize][i]+=1;
            }
        }
        let mut dp = vec![vec![-1;wLen+1];tLen+1];
        return solve(0,0,target.as_bytes(),&freq,&mut dp)
    }
}
fn solve(col:usize, len:usize,target:&[u8],freq:&Vec<Vec<i32>>, memo:&mut Vec<Vec<i32>>)->i32{
    if col==freq[0].len(){
        return if len==target.len(){1}else{0}
    }
    if memo[len][col]!=-1{
        return memo[len][col]
    }
    let mut res = (solve(col+1,len,target,freq,memo)%MOD) as i64;
    if len<target.len(){
        res = (res
            + freq[(target[len] - b'a') as usize][col] as i64
                * solve(col + 1, len + 1, target, freq, memo) as i64
                % MOD as i64)
            % MOD as i64;
    }

    memo[len][col]=res as i32;
    return res as i32
}