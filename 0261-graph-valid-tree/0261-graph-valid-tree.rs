use std::collections::{HashSet,HashMap,VecDeque};
pub struct UnionFind {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            root: (0..size).collect(),
            rank: vec![1; size],
        }
    }

    pub fn union(&mut self, x: usize, y: usize)->bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                self.root[root_y] = root_x;
            } else if self.rank[root_x] < self.rank[root_y] {
                self.root[root_x] = root_y;
            } else {
                self.root[root_y] = root_x;
                self.rank[root_x] += 1;
            }
            return true
        }
        false
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x == self.root[x] {
            x
        } else {
            self.root[x] = self.find(self.root[x]);
            self.root[x]
        }
    }
}

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        if edges.len()!=n as usize -1{
            return false
        }
        let mut uf = UnionFind::new(n as usize);
        for edge in edges.iter(){
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            if !uf.union(x,y){
                return false
            }
        }
        true
    }
}
