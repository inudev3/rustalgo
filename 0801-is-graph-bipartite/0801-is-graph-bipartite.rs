impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut colors = vec![-1;n];
        for i in 0..n{
            
            if colors[i]==-1 && !dfs(&graph, &mut colors, i, 0){
                return false
            }
        }
        true
    }
}
fn dfs(graph:&Vec<Vec<i32>>, colors:&mut Vec<i32>, node:usize, color:i32)->bool{
    if colors[node]!=-1{
        return colors[node]==color
    }
    colors[node]=color;
    for &nei in graph[node].iter(){
        if !dfs(graph,colors,nei as usize, color^1){
            return false
        }
    }
    true
}