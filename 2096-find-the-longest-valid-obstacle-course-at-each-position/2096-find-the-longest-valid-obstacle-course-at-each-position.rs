impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let n = obstacles.len();
        let mut dp = vec![1;n];
        let mut LIS = vec![];
        for i in 0..n{
            let idx = LIS.partition_point(|&x|x<=obstacles[i]);
            if idx==LIS.len(){
                LIS.push(obstacles[i]);
            }else{
                LIS[idx] = obstacles[i];
            }
            dp[i] = (idx+1) as i32;
        }
        dp
    }
}