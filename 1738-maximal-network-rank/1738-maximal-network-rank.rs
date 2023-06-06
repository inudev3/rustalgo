impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let n = n as usize;
        let mut adj = vec![vec![0;n];n];
        let mut indegree = vec![0;n];
        for road in roads.iter(){
            let from = road[0] as usize;
            let to = road[1] as usize;
            indegree[from]+=1;
            indegree[to]+=1;
            adj[from][to] =1;
            adj[to][from]=1;
        }
        
        for i in 0..n{
            for j in i+1..n{
                ans = std::cmp::max(indegree[i]+indegree[j]-adj[i][j], ans);
            }
        }
        ans
    }
}