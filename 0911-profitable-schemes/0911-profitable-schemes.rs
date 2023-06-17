impl Solution {
    pub fn profitable_schemes(n: i32, mip: i32, g: Vec<i32>, p: Vec<i32>) -> i32 {
        let (mut dp, mut mask, n, mip) = ([[0u128;101]; 101], [0u128; 101], n as usize, mip as usize);
        dp[0][0]=1; mask[0]=1;
        for (&g, &p) in g.iter().zip(p.iter()) {
            for i in (0..(n as i32-g+1).max(0) as usize).rev() {
                let x = mask[i];
                for j in (0..=mip).filter(|&j| (1<<j)&x>0) {
                    let jj = (j+p as usize).min(mip);
                    mask[i+g as usize]|=1<<jj;
                    dp[i+g as usize][jj]+=dp[i][j]
                }
            }
        }
        (dp.iter().map(|x| x[mip]).sum::<u128>() % 1000000007) as i32
    }
}