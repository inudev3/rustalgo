use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(sz: usize) -> Self {
        UnionFind {
            parent: (0..sz).collect(),
            rank: vec![1; sz],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root != y_root {
            if self.rank[x_root] < self.rank[y_root] {
                self.parent[x_root] = y_root;
            } else if self.rank[x_root] > self.rank[y_root] {
                self.parent[y_root] = x_root;
            } else {
                self.parent[y_root] = x_root;
                self.rank[x_root] += 1;
            }
        }
    }
}
impl Solution{
fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let n = accounts.len();
    let mut uf = UnionFind::new(n);
    let mut email_to_id = HashMap::new();
    let mut email_to_name = HashMap::new();

    for i in 0..n {
        for j in 1..accounts[i].len() {
            let email = &accounts[i][j];
            if !email_to_id.contains_key(email) {
                email_to_id.insert(email.clone(), i);
            }  
            uf.union(i, email_to_id[email]);            
            email_to_name.insert(email.clone(), accounts[i][0].clone());
        }
    }

    let mut lists = vec![vec![]; n];
    for (email, &id) in email_to_id.iter() {
        let index = uf.find(id);
        lists[index].push(email.clone());
    }

    for list in &mut lists {
        if !list.is_empty() {
            list.sort();
            list.insert(0, email_to_name[&list[0]].clone());
        }
    }

    lists.into_iter().filter(|v| !v.is_empty()).collect()
}
}
