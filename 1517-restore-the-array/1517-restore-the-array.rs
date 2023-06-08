const MOD:i32 = 1_000_000_007;
impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![-1;n+1];
        solve(0,s,&mut dp, k)
    }
}
fn solve(idx:usize,s:&[u8],memo:&mut Vec<i32>, k:i32)->i32{
    if memo[idx]!=-1{
        return memo[idx]
    }
    if idx==s.len(){
        return 1
    }
    if s[idx]==b'0'{
        return 0
    }
    let mut cnt=0;
    for i in idx..s.len(){
        let slice = std::str::from_utf8(&s[idx..=i]).unwrap();
        if slice.parse::<i64>().unwrap()>k as i64{
            break;
        }
        cnt = (cnt+solve(i+1,s,memo,k))%MOD;
    }
    memo[idx]=cnt;
    cnt
}