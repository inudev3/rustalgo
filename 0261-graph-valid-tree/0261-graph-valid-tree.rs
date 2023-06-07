use std::collections::{HashSet,HashMap,VecDeque};
impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![vec![];n as usize];
        for edge in edges.iter(){
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
        let mut parent = HashMap::new();
        parent.insert(0,-1);
        let mut queue = VecDeque::new();
        queue.push_back(0);
        while let Some(node) = queue.pop_front(){
            for &next in graph[node as usize].iter(){
                if *parent.get(&node).unwrap_or(&-1) == next{
                    continue
                }
                if parent.contains_key(&next){
                    return false
                }
                queue.push_back(next);
                parent.insert(next,node);
            }
        }
        return parent.len()==n as usize
    }
}
