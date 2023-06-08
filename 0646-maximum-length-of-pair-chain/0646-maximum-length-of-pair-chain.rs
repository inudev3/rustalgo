impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_by_key(|pair|pair[1]);
        let mut cur = i32::MIN;
        let mut ans=0;
        for pair in &pairs{
            if cur<pair[0]{
                cur= pair[1];
                ans+=1;
            }
        }
        ans
    }
}