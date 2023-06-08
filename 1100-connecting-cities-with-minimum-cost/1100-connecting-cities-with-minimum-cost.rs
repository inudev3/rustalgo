pub fn union(parent:&mut Vec<usize>,rank:&mut Vec<usize>,x:usize,y:usize)->bool{
    let rootx = find(parent,x);
    let rooty = find(parent,y);
    if rootx!=rooty{
        if rank[rootx]>rank[rooty]{
            parent[rootx] =rooty;
        }else if rank[rooty]<rank[rootx] {
            parent[rooty] =rootx;
        }else{
            parent[rooty]=rootx;
            rank[rootx]+=1;
        }
        return true
    }
    false
}
pub fn find(parent:&mut Vec<usize>,x:usize)->usize{
    if x==parent[x]{
        x
    }else{
        parent[x] =find(parent,parent[x]);
        parent[x]
    }
}
impl Solution {
    pub fn minimum_cost(n: i32,mut connections: Vec<Vec<i32>>) -> i32 {
        connections.sort_by_key(|edge|edge[2]);
        let n = n as usize;
        let mut parent = (0..n+1).collect();
        let mut rank = vec![1;n+1];
        let mut res=0;
        let mut cnt=0;
        for conn in &connections{
            if !union(&mut parent,&mut rank, conn[0] as usize,conn[1] as usize){
                continue;
            }
            res+=conn[2];
            cnt+=1;
        }
        if cnt==n-1{
            res
        }else{
            -1
        }
    }
}