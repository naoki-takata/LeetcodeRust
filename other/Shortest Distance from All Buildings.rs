use std::collections::VecDeque;

impl Solution {
    pub fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        
        // すべての建物の位置を収集
        let mut buildings = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    buildings.push((i, j));
                }
            }
        }
        
        // 建物がない場合は-1を返す
        if buildings.is_empty() {
            return -1;
        }
        
        // 各空き地について、距離の合計と到達回数を記録
        let mut total_distance = vec![vec![0; n]; m];
        let mut reach_count = vec![vec![0; n]; m];
        
        // 4方向への移動（上、右、下、左）
        let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        
        // 各建物からBFSを実行
        for &(start_row, start_col) in &buildings {
            let mut queue = VecDeque::new();
            let mut visited = vec![vec![false; n]; m];
            let mut distance = vec![vec![0; n]; m];
            
            queue.push_back((start_row, start_col));
            visited[start_row][start_col] = true;
            
            while let Some((row, col)) = queue.pop_front() {
                // 4方向を探索
                for (dr, dc) in &directions {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;
                    
                    // 境界チェック
                    if new_row < 0 || new_row >= m as i32 || new_col < 0 || new_col >= n as i32 {
                        continue;
                    }
                    
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;
                    
                    // 訪問済み、または障害物/建物の場合はスキップ
                    if visited[new_row][new_col] || grid[new_row][new_col] != 0 {
                        continue;
                    }
                    
                    // 距離を更新
                    visited[new_row][new_col] = true;
                    distance[new_row][new_col] = distance[row][col] + 1;
                    total_distance[new_row][new_col] += distance[new_row][new_col];
                    reach_count[new_row][new_col] += 1;
                    
                    queue.push_back((new_row, new_col));
                }
            }
        }
        
        // すべての建物から到達可能な空き地の中で、最小の距離合計を探す
        let mut min_distance = i32::MAX;
        for i in 0..m {
            for j in 0..n {
                // 空き地で、すべての建物から到達可能な場合
                if grid[i][j] == 0 && reach_count[i][j] == buildings.len() {
                    min_distance = min_distance.min(total_distance[i][j]);
                }
            }
        }
        
        if min_distance == i32::MAX {
            -1
        } else {
            min_distance
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let grid = vec![
            vec![1, 0, 2, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
        ];
        assert_eq!(Solution::shortest_distance(grid), 7);
    }

    #[test]
    fn test_example2() {
        let grid = vec![
            vec![1, 0],
        ];
        assert_eq!(Solution::shortest_distance(grid), 1);
    }

    #[test]
    fn test_example3() {
        let grid = vec![
            vec![1],
        ];
        assert_eq!(Solution::shortest_distance(grid), -1);
    }

    #[test]
    fn test_no_reachable_land() {
        let grid = vec![
            vec![1, 2, 1],
        ];
        // 建物はあるが、空き地がないため-1
        assert_eq!(Solution::shortest_distance(grid), -1);
    }

    #[test]
    fn test_single_building() {
        let grid = vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 0],
        ];
        // 建物が1つだけなので、距離は0（既に建物がある場所に建てる必要はないが、
        // 問題の制約により空き地に建てる必要がある）
        // 実際には、建物が1つの場合、その建物自体が答えになるが、
        // 問題では空き地に建てる必要があるため、最も近い空き地を選ぶ
        // この場合、距離1の空き地が4つあるので、最小距離は1
        assert_eq!(Solution::shortest_distance(grid), 1);
    }
}

