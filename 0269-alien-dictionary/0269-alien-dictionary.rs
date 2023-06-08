use std::collections::{HashMap, HashSet, VecDeque};
impl Solution{
    pub fn alien_order(words: Vec<String>) -> String {
        let mut adj: HashMap<char, Vec<char>> = HashMap::new();
        let mut indegree: HashMap<char, usize> = HashMap::new();
        let mut set: HashSet<char> = HashSet::new();
        let words: Vec<Vec<char>> = words.into_iter().map(|s| s.chars().collect()).collect();
        for word in &words {
            for c in word {
                set.insert(*c);
            }
        }

        for &ch in &set {
            adj.insert(ch, vec![]);
            indegree.insert(ch, 0);
        }

        for i in 0..words.len()-1 {
            let word1 = &words[i];
            let word2 = &words[i+1];
            if word1.len() > word2.len() && word1.starts_with(word2) {
                return String::new();
            }

            for j in 0..std::cmp::min(word1.len(), word2.len()) {
                if word1[j] != word2[j] {
                    indegree.entry(word2[j]).and_modify(|e|  *e += 1 );
                    adj.entry(word1[j]).and_modify(|e| e.push(word2[j]));
                    break;
                }
            }
        }

        let mut queue: VecDeque<char> = VecDeque::new();
        for (&k, &v) in &indegree {
            if v == 0 {
                queue.push_back(k);
            }
        }

        let mut result = String::new();
        while let Some(curr)= queue.pop_front() {
            result.push(curr);
            if let Some(next_chars) = adj.get(&curr) {
                for &next in next_chars {
                    if let Some(count) = indegree.get_mut(&next) {
                        *count -= 1;
                        if *count == 0 {
                            queue.push_back(next);
                        }
                    }
                }
            }
        }

        if result.len() < indegree.len() {
            String::new()
        } else {
            result
        }
    }
}
