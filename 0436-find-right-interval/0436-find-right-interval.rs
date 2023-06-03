impl Solution {
    pub fn find_right_interval(mut intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut indexed:Vec<_> = intervals.iter().map(|x|x[0]).enumerate().collect();
        indexed.sort_by_key(|&x|x.1);
        let n = intervals.len();
        let mut res= vec![];
        // start로 정렬한 뒤에 
        // end에 대한 lower bound 찾기
        for i in 0..n{
            let idx = indexed.partition_point(|&x|x.1<intervals[i][1]);
            let mut ans= -1;
            if idx!=n{
                ans = indexed[idx].0 as i32;
            }
            res.push(ans);
        }
        res
    }
}