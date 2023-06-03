impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by(|a, b| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => b[1].cmp(&a[1]),
            other => other,
        });
        let mut LIS = vec![];
        for env in envelopes.iter(){
            if LIS.is_empty() {
                LIS.push(env[1]);
            }else{
                let idx = LIS.partition_point(|&x|x<env[1]);             
                if idx==LIS.len(){
                    LIS.push(env[1]);
                }else{
                    LIS[idx]= env[1];                
                }
            }
        }
        return LIS.len() as i32
    }
}