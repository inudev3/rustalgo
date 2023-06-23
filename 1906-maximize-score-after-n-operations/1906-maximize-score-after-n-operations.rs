impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let n =nums.len()/2;
        //(i+1)*gcd(x,y) / remove x,y
        let total = 1<<(2*n);
        let mut dp = vec![-1;total];
        solve(&nums,0, &mut dp)
    }
}
fn solve(nums:&Vec<i32>, mask:usize, memo:&mut Vec<i32> )->i32{
    let one_count = mask.count_ones() as usize;
    if one_count==nums.len(){
        return 0
    }
    if memo[mask]!=-1{
        return memo[mask]
    }
    let mut ans =0;
    for i in 0..nums.len(){
        if (mask>>i)&1>0{
            continue
        }
        for j in i+1..nums.len(){
            if (mask>>j)&1>0{
                continue
            }
            let newmask = mask| (1<<j) | (1<<i);
            let score = gcd(nums[i],nums[j]) *(one_count/2+1) as i32;
            ans = std::cmp::max(solve(nums,newmask, memo)+score,ans);
        }
    }
    memo[mask]=ans;
    ans
}
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}