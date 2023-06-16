const MOD:i32 = 1_000_000_007;
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let pizza:Vec<_> = pizza.iter().map(|str|str.as_bytes()).collect();
        let m = pizza.len();
        let n = pizza[0].len();
        let mut dp = vec![vec![vec![-1;n];m];k as usize];
        let mut prefix = vec![vec![0;n+1];m+1];
        for i in (0..m).rev(){
            for j in (0..n).rev(){
                prefix[i][j] = if pizza[i][j]==b'A'{1}else {0}+prefix[i+1][j]+prefix[i][j+1]-prefix[i+1][j+1];     
            }
        }
        return solve(&mut dp,&prefix,0,0,k as usize-1)
    }
}
fn solve(memo:&mut Vec<Vec<Vec<i32>>>, prefix:&Vec<Vec<i32>>, i:usize,j:usize,rem:usize) ->i32{
    if prefix[i][j]==0{
        return 0
    }
    if rem==0{
        return 1
    }
    if memo[rem][i][j]!=-1{
        return memo[rem][i][j]
    }
    let mut ans=0;
    for nr in i+1..prefix.len()-1{
        if prefix[i][j]-prefix[nr][j]>0{
            ans = (ans+solve(memo,prefix,nr,j,rem-1)%MOD)%MOD;
        }        
    }
    for nc in j+1..prefix[0].len()-1{
        if prefix[i][j]-prefix[i][nc]>0{
            ans = (ans+solve(memo,prefix,i,nc,rem-1)%MOD)%MOD;
        }
    }
    memo[rem][i][j]=ans;
    return ans
}