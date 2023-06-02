impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut cnt=0;
        let mut sorted = nums.clone();
        sorted.sort();
        let n = nums.len();
        if n<3{
            return 0
        }
        for i in 0..n-2{
            if sorted[i]==0{continue;}
            let mut k=i+2;
            for j in i+1..n-1 {
                while k<n && sorted[i]+sorted[j]>sorted[k]{k+=1;}
                cnt+=k-j-1;
            }
        }
        cnt as i32
    }
}