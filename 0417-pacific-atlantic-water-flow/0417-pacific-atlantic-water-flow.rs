use std::collections::VecDeque;
const dx:[i32;4] = [1,-1,0,0];
const dy:[i32;4] = [0,0,1,-1];
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let mut pacific_queue = VecDeque::new();
        let mut atlantic_queue = VecDeque::new();
        for i in 0..m{
            pacific_queue.push_back((i,0));
            atlantic_queue.push_back((i,n-1));
        }
        for i in 0..n{
            pacific_queue.push_back((0,i));
            atlantic_queue.push_back((m-1,i))
        }
        let mut pacific_visited = vec![vec![false; n]; m];
        let mut atlantic_visited = vec![vec![false; n]; m];
        bfs(&heights, &mut pacific_queue, &mut pacific_visited);
        bfs(&heights, &mut atlantic_queue, &mut atlantic_visited);
        let mut res = vec![];
        for i in 0..m {
            for j in 0..n{
                if pacific_visited[i][j]&&atlantic_visited[i][j]{
                    let i = i as i32;
                    let j = j as i32;
                    res.push(vec![i,j]);
                }
            }
        }
        res
    }
}
fn bfs(heights: &Vec<Vec<i32>>,queue:&mut VecDeque<(usize,usize)>, visited:&mut Vec<Vec<bool>>){
    while let Some((x,y)) = queue.pop_front(){
        visited[x][y]=true;
        for k in 0..4 as usize{
            let nx = x as i32+dx[k];
            let ny = y as i32 +dy[k];
            if 0<=nx && nx< heights.len() as i32 && 0<=ny && ny<heights[0].len() as i32{
                let nx = nx as usize;
                let ny = ny as usize;
                let x = x as usize;
                let y = y as usize;
              
                if visited[nx][ny]||heights[nx][ny]<heights[x][y]{
                    continue
                }
                
                queue.push_back((nx,ny));
            }
        }
    }
}