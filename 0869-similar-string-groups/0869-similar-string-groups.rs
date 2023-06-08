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
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        
        let n = strs.len();
        let mut parent = (0..n).collect();
        let mut rank = vec![1;n];
        let mut cnt = n;
        for i in 0..n{
            for j in i+1..n{
                if isSim(strs[i].as_bytes(), strs[j].as_bytes()){
                    if union(&mut parent,&mut rank, i,j){
                        cnt-=1;
                    }
                }
            }
        }
        cnt as i32
    }
}
fn isSim(a:&[u8],b:&[u8])->bool{
    let n = a.len();
    let mut cnt=0;
    for i in 0..n{
        if a[i]!=b[i]{
            cnt+=1;
        }
    }
    cnt==0||cnt==2
}