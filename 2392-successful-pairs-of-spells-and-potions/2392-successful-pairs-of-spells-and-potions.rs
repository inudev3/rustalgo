impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let n =spells.len();
        let m = potions.len();
        let mut res = vec![0;n];
        potions.sort();
        let sorted:Vec<_> = potions.iter().map(|&x| x as i64).collect();
        
        for i in 0..n{
            let bound = (success+(spells[i]-1) as i64)/(spells[i] as i64);
            let idx = sorted.partition_point(|&x|
                x<bound
            );
            res[i] = (m-idx) as i32;
        }
        return res
    }
}