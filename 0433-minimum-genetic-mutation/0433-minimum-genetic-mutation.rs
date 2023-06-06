use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let mut set:HashSet<&String> = bank.iter().collect();
        if !set.contains(&end_gene) {
            return -1
        }
        let mut queue = VecDeque::new();
        set.clear();
        queue.push_back((&start_gene,0));
        set.insert(&start_gene);
        while let Some((curr,step)) = queue.pop_front(){
            if *curr==end_gene{
                return step
            }
            for next in &bank{
                if !set.contains(next){
                    if isValid(next,curr){
                        set.insert(next);
                        queue.push_back((next,step+1));
                    }
                }
            }
        }
        -1
    }
}
fn isValid(w1:&String, w2:&String)->bool{
    let n = w1.len();
    let mut cnt=0;
    for i in 0..n{
        if w1.chars().nth(i)!= w2.chars().nth(i){
            cnt+=1;
        }
    }
    return cnt==1
}