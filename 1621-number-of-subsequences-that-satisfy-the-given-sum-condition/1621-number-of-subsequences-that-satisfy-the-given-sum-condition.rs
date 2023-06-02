const MOD :i32 = 1_000_000_007;
impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut cnt=0;
        nums.sort();
        let mut power = vec![0;nums.len()];
        power[0]=1;
        for i in 1..nums.len(){
            power[i] = (power[i-1]*2)%MOD
        }
        for left in 0..nums.len(){
            let right = nums.partition_point(|&x|x<=target-nums[left]) as i32 -1;
            if right<0{
                continue;
            }
            let right = right as usize;
            if left<=right{
                cnt= (cnt + power[right-left])%MOD;
            }
        }
        cnt
    }
}