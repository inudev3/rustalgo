impl Solution {
    pub fn probability_of_heads(prob: Vec<f64>, target: i32) -> f64 {
        let n = prob.len();
        let mut dp = vec![vec![-1f64;n];target as usize+1];
        let mut suffix = vec![1.0;n+1];
        for i in (0..n).rev(){
            suffix[i] = suffix[i+1]*(1f64-prob[i]);
        }
        solve(target as usize,0,&mut dp,  &prob, &suffix)
    }   
}
fn solve(rem:usize,idx:usize,memo:&mut Vec<Vec<f64>>, prob:&Vec<f64>, suffix:&Vec<f64>)->f64{
    
    if idx==prob.len(){
        return if rem>0{0f64}else{1f64}
    }
    if rem==0{
        return suffix[idx]
    }
    if memo[rem][idx]!=-1f64{
        return memo[rem][idx]
    }
    memo[rem][idx]=solve(rem-1,idx+1,memo,prob, suffix)*prob[idx]+solve(rem,idx+1,memo,prob,suffix)*(1 as f64-prob[idx]);
    memo[rem][idx]
}