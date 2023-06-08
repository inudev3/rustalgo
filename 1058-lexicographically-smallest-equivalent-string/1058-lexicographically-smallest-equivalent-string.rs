pub fn union(parent:&mut Vec<usize>,rank:&mut Vec<usize>,x:usize,y:usize)->bool{
    let rootx = find(parent,x);
    let rooty = find(parent,y);
    if rootx!=rooty{
        if rank[rootx]>rank[rooty]{
            parent[rootx] =rooty;
        }else {
            parent[rooty] =rootx;
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
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let n = s1.len();
        let mut parent:Vec<_> = (0..26).collect();
        let mut rank:Vec<_> =(0..26).collect();
        let s1 = s1.as_bytes();
        let s2= s2.as_bytes();
        for i in 0..n{
            union(&mut parent,&mut rank, (s1[i]-b'a') as usize, (s2[i]-b'a') as usize);
        }
        let mut base_str = base_str.into_bytes();
        for i in 0..base_str.len(){
            let root =find(&mut parent, (base_str[i] - b'a') as usize);
            base_str[i] = b'a'+root as u8;
        }
        String::from_utf8(base_str).unwrap()
    }
}