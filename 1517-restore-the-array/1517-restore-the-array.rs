const MOD:i32 = 1_000_000_007;
impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
       let n = s.len();
       let mut dp = vec![-1;n+1];
        solve(&mut dp, 0, k, s.as_bytes())
    }
}
fn solve(memo:&mut Vec<i32>, idx:usize,k:i32,s:&[u8])->i32{
    
    if memo[idx]!=-1{
        return memo[idx]
    }
    if idx==s.len(){
        return 1
    }

    if s[idx]==b'0'{
        memo[idx]=0;
        return 0
    }
    let mut ans=0;
    for i in idx..s.len(){
        let slice = std::str::from_utf8(&s[idx..i+1]).unwrap();
        if slice.parse::<i64>().unwrap()>k as i64{
            break;
        }
        ans = (ans+solve(memo,i+1,k,s)%MOD)%MOD;
    }
    memo[idx]=ans;
    ans
}
