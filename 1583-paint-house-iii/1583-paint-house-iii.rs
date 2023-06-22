const MAX_COST: i32 = 1000001;

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let mut dp = vec![vec![vec![-1; n as usize+1]; target as usize+1]; m as usize];
        let res = solve(&mut dp, &houses, &cost, 0, 0, 0);
        if res == MAX_COST {
            return -1
        }
        res
    }
}

fn solve(memo: &mut Vec<Vec<Vec<i32>>>, houses: &Vec<i32>, cost: &Vec<Vec<i32>>, i: usize, prevcolor: i32, cnt: usize) -> i32 {

    if i >= houses.len() {
        if cnt == memo[0].len() - 1 {
            return 0
        } else {
            return MAX_COST
        }
    }
    if cnt >= memo[0].len() {
        return MAX_COST
    }
    if memo[i][cnt][prevcolor as usize] != -1 {
        return memo[i][cnt][prevcolor as usize]
    }

    let mut min = MAX_COST;
     if houses[i]!=0{
        let newcnt = if houses[i]!=prevcolor as i32{1} else {0} + cnt;
        min = solve(memo,houses,cost,i+1,houses[i], newcnt);
    }else{
        for color in 1..=cost[0].len(){
            let newcnt=if color!=prevcolor as usize{1}else{0}+cnt;
            min = std::cmp::min(min,cost[i][color-1]+solve(memo,houses,cost,i+1,color as i32,newcnt));
        }
    }
    memo[i][cnt][prevcolor as usize] = min;
    min
}
