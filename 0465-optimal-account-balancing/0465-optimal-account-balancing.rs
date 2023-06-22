use std::collections::HashMap;
impl Solution {
    pub fn min_transfers(transactions: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![-1;1<<12];
        let mut balance = HashMap::new();
        for tr in &transactions{
            let (from, to, amount) = (tr[0], tr[1], tr[2]);
            *balance.entry(from).or_insert(0)+=amount;
            *balance.entry(to).or_insert(0)-=amount;
        }
        let mut dept_arr = balance.values().filter(|&&x| x != 0).cloned().collect::<Vec<_>>();
        fn backtrack (start:usize, nums:&mut Vec<i32>) ->i32{
            let mut s = start;
            while s<nums.len()&& nums[s]==0{
                s+=1;
            }
            if s==nums.len(){
                return 0
            }
            let mut res = i32::MAX;
            let mut prev=0;
            for i in s+1..nums.len(){
                if nums[i]!=prev && nums[i]*nums[s]<0{
                    nums[i]+=nums[s];
                    res = res.min(1+backtrack(s+1,nums));
                    nums[i]-=nums[s];
                    prev = nums[i];
                }
            }
            if res!=i32::MAX{
                res
            }else{
                0
            }
        }
        backtrack(0,&mut dept_arr)
    }
}