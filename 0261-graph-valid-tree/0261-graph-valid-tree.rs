use std::collections::HashSet;
impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![vec![];n as usize];
        for edge in edges.iter(){
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
        let mut seen = HashSet::new();
        return dfs(0,-1,&mut seen, &graph) && seen.len()==n as usize
    }
}
fn dfs(node:i32, parent:i32, seen:&mut HashSet<i32>, adj:&Vec<Vec<i32>>)->bool{
    if seen.contains(&node){
        return false
    }
    seen.insert(node);
    for &next in adj[node as usize].iter(){
        if parent==next{
            continue
        }
        if !dfs(next,node, seen,adj){
            return false
        }
    }
    return true
}