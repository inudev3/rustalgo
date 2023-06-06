impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut indegree= vec![0;n as usize];
        for edge in &edges{
            indegree[edge[1] as usize]+=1;
        }
        let mut res =vec![];
        for i in 0..n as usize{
            if indegree[i]==0{
                res.push(i as i32);
            }
        }
        res
    }
}