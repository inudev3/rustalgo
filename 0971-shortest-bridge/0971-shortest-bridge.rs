use std::collections::VecDeque;
const dx:[i32;4] = [0,0,-1,1];
const dy :[i32;4] = [1,-1,0,0];

impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut secondqueue = VecDeque::new();

        'label: for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    dfs(&mut grid, i,j, &mut secondqueue);
                    break 'label;
                }
            }
        }


        let mut dist = 0;
        while !secondqueue.is_empty() {
            let len = secondqueue.len();
            for _ in 0..len { // process all nodes in the current level
                let (x, y) = secondqueue.pop_front().unwrap();
                for k in 0..4 as usize{
                    let nx = x as i32 + dx[k];
                    let ny = y as i32 + dy[k];
                    if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if grid[nx][ny] == 1 {
                            return dist;
                        } else if grid[nx][ny] == 0 {
                            grid[nx][ny] = 2;
                            secondqueue.push_back((nx, ny));
                        }
                    }
                }
            }
            dist += 1; // increment the distance after one level is processed
        }
        dist
    }
}
 fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, queue: &mut VecDeque<(usize, usize)>) {
        let n = grid.len();
        if x >= n || y >= n || grid[x][y] != 1 {
            return;
        }
        grid[x][y] = 2;
        queue.push_back((x, y));
        for k in 0..4 as usize {
            let nx = x as i32 + dx[k];
            let ny = y as i32 + dy[k];
            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                dfs(grid, nx as usize, ny as usize, queue);
            }
        }
    }

