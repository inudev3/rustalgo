impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start =nums.partition_point(|&x|x<target);
        let end = nums.partition_point(|&x|x<=target);
        if start==end{
             vec![-1,-1]
        }else{
            vec![start as i32,(end -1) as i32 ]
        }
    }
}