impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut LIS = vec![];
        let n = nums.len();
        for &num in nums.iter(){
            if LIS.is_empty(){
                LIS.push(num);
            }
            else{
                let idx = LIS.partition_point(|&x|x<num);
                if idx==LIS.len(){
                    LIS.push(num);
                }
                else {
                    LIS[idx] = num;
                }
            }
        }
        return LIS.len() as i32
    }
}