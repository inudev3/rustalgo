use std::collections::HashMap;
impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        let m = arr1.len();
        let n = arr2.len();
        arr2.sort();
        let mut dp = HashMap::new();
        match solve(&mut dp, 0, -1, &arr1, &arr2){
            2001=>-1,
            x => x
        }
    }
}
fn solve(memo:&mut HashMap<(i32,i32),i32>, idx:usize, prev:i32, arr1:& Vec<i32>, arr2:&Vec<i32> )->i32{
    if idx==arr1.len(){
        return 0
    }
    if memo.contains_key(&(idx as i32, prev)){
        return *memo.get(&(idx as i32, prev)).unwrap()
    }
    let mut ans = 2001;
    if arr1[idx]>prev{       
        ans = solve(memo,idx+1,arr1[idx],arr1,arr2);
    }
    let replace = arr2.partition_point(|&x|x<=prev);
    if replace<arr2.len(){
        let res = solve(memo,idx+1,arr2[replace],arr1,arr2);
        if res!=2001{
            ans = ans.min(1+res);
        }
    }
    memo.insert((idx as i32, prev), ans);
    ans
}