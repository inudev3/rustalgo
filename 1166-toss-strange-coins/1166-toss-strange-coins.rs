impl Solution {
    pub fn probability_of_heads(prob: Vec<f64>, target: i32) -> f64 {
        let n = prob.len();
        let mut dp = vec![vec![-1f64;n];target as usize+1];
    
        solve(target,0,&mut dp,  &prob)
    }   
}
fn solve(rem:i32,idx:usize,memo:&mut Vec<Vec<f64>>, prob:&Vec<f64>)->f64{
    if rem<0{
        return 0f64
    }
    let rem = rem as usize;
    if idx==prob.len(){
        return if rem>0{0f64}else{1f64}
    }
    
    if memo[rem][idx]!=-1f64{
        return memo[rem][idx]
    }
    memo[rem][idx]=solve(rem as i32-1,idx+1,memo,prob)*prob[idx]+solve(rem as i32,idx+1,memo,prob)*(1 as f64-prob[idx]);
    memo[rem][idx]
}