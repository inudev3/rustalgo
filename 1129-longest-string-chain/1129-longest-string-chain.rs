use std::collections::{HashMap, HashSet};

impl Solution{
     fn longest_str_chain(words: Vec<String>) -> i32 {
    let (mut set, mut memo): (HashSet<_>, HashMap<_, _>) = words.iter().map(|x| {
        (x.clone(), (x.clone(), -1))
    }).unzip();
    let mut ans = 0;
    for word in &words {
        ans = std::cmp::max(ans, solve(&mut memo, &set, word));
    }
    ans
}
}
fn solve(memo: &mut HashMap<String, i32>, wordset: &HashSet<String>, curr: &str) -> i32 {
    if let Some(&result) = memo.get(curr) {
        if result != -1 {
            return result;
        }
    }
    let mut res = 1;
    for i in 0..curr.len() {
        let new_word = format!("{}{}", &curr[..i], &curr[i+1..]);
        if wordset.contains(&new_word) {
            let currlen = 1 + solve(memo, wordset, &new_word);
            res = std::cmp::max(res, currlen);
        }
    }
    memo.insert(curr.to_string(), res);
    res
}
