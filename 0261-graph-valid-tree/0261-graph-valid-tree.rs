use std::collections::{HashSet,HashMap,VecDeque};
impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        if edges.len()!=n as usize-1{
            return false
        }
        let mut graph = vec![vec![];n as usize];
        for edge in edges.iter(){
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
        let mut seen = HashSet::new();
        seen.insert(0);
        let mut queue = VecDeque::new();
        queue.push_back(0);
        while let Some(node) = queue.pop_front(){
            for &next in graph[node as usize].iter(){
                if seen.contains(&next){
                    continue
                }
                seen.insert(next);
                queue.push_back(next)
            }
        }
        return seen.len()==n as usize
    }
}
