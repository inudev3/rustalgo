use std::collections::{HashSet,HashMap,VecDeque};


pub fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize)->bool {
    let root_x = find(root, x);
    let root_y = find(root, y);
    if root_x != root_y {
        if rank[root_x] > rank[root_y] {
            root[root_y] = root_x;
        } else if rank[root_x] < rank[root_y] {
            root[root_x] = root_y;
        } else {
            root[root_y] = root_x;
            rank[root_x] += 1;
        }
        return true
    }
    false
}

pub fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if x == root[x] {
        x
    } else {
        root[x] = find(root, root[x]);
        root[x]
    }
}

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        if edges.len()!=n -1{
            return false
        }
        let mut root:Vec<usize> = (0..n).collect();
        let mut rank = vec![1;n];
        for edge in edges.iter(){
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            if !union(&mut root, &mut rank,x,y){
                return false
            }
        }
        true
    }
}
