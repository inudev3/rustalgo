use std::collections::{HashMap, HashSet, VecDeque};
impl Solution{
    pub fn alien_order(words: Vec<String>) -> String {
        let mut adj = vec![vec![];26];
        let mut indegree = vec![0;26];
        let mut set: HashSet<u8> = HashSet::new();
        let words: Vec<Vec<u8>> = words.into_iter().map(|s| s.into_bytes()).collect();
        for word in &words {
            for &c in word {
                set.insert(c-b'a');
            }
        }

        for i in 0..words.len()-1 {
            let word1 = &words[i];
            let word2 = &words[i+1];
            if word1.len() > word2.len() && word1.starts_with(word2) {
                return String::new();
            }

            for j in 0..std::cmp::min(word1.len(), word2.len()) {
                if word1[j] != word2[j] {
                    let a =(word1[j]-b'a') as usize;
                    let b = (word2[j]-b'a') as usize;
                    indegree[b]+=1;
                    adj[a].push(word2[j]);
                    break;
                }
            }
        }

        let mut queue: VecDeque<u8> = VecDeque::new();
        for i in 0..26 as u8{
            if set.contains(&i)&&indegree[i as usize] == 0 {
                queue.push_back(i as u8);
            }
        }

        let mut result = vec![];
        while let Some(curr)= queue.pop_front() {
            result.push(curr+b'a');
            for &next in &adj[curr as usize] {     
                let a= (next-b'a') as usize;            
               indegree[a] -= 1;
                if indegree[a] == 0 {
                    queue.push_back(a as u8);
                }             
            }
        }

        if result.len() < set.len() {
            String::new()
        } else {
            String::from_utf8(result).unwrap()
        }
    }
}
