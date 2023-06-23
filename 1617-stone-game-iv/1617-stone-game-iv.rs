impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut dp = vec![None;n as usize+1];
        solve(n,&mut dp)
    }
}
fn solve(rem:i32,  memo:&mut Vec<Option<bool>>)->bool{
    if rem<=0{
        return false
    }
    if memo[rem as usize].is_some(){
        return memo[rem as usize].unwrap()
    }
    let mut ans=false;
    for i in 1..=(rem as f32).sqrt().floor() as i32{
        ans  = ans || !solve(rem -i*i, memo);
        if ans{
            break;
        }
    }
    memo[rem as usize]=Some(ans);
    ans
}
