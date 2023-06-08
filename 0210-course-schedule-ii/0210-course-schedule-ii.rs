use std::collections::{ VecDeque };
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses;
        let n = n as usize;
        let mut graph = vec![vec![];n];
        let mut indegree = vec![0;n];
        for req in &prerequisites{
            graph[req[1] as usize].push(req[0]);
            indegree[req[0] as usize]+=1;
        }
        let mut res =vec![];
        let mut queue = VecDeque::new();
        for i in 0..n{
            if indegree[i]==0{
                queue.push_back(i);
            }
        }
        while let Some(curr) = queue.pop_front(){
            res.push(curr as i32);
            for &next in &graph[curr]{
                indegree[next as usize]-=1;
                if indegree[next as usize]==0{
                    queue.push_back(next as usize);
                }
            }
        }
        if res.len()==n{res} else {vec![]}
    }
}