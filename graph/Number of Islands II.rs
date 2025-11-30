// Number of Islands II
// 2Dグリッドに土地を順次追加していき、各操作後の島の数を返す
// Union-Find（Disjoint Set Union）データ構造を使用して効率的に処理

impl Solution {
    pub fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
        let m = m as usize;
        let n = n as usize;
        let total_cells = m * n;
        
        // Union-Findデータ構造
        let mut parent: Vec<i32> = (0..total_cells as i32).collect();
        let mut rank: Vec<i32> = vec![0; total_cells];
        
        // グリッドの状態を追跡（土地かどうか）
        let mut grid: Vec<Vec<bool>> = vec![vec![false; n]; m];
        
        // 島の数を追跡
        let mut island_count = 0;
        let mut result = Vec::new();
        
        // 4方向への移動（上、右、下、左）
        let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        
        for position in positions {
            let row = position[0] as usize;
            let col = position[1] as usize;
            
            // 既に土地の場合はスキップ
            if grid[row][col] {
                result.push(island_count);
                continue;
            }
            
            // 新しい土地を追加
            grid[row][col] = true;
            island_count += 1;
            
            let current_idx = (row * n + col) as i32;
            
            // 隣接する4方向の土地とマージ
            for (dr, dc) in &directions {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;
                
                // 境界チェック
                if new_row < 0 || new_row >= m as i32 || new_col < 0 || new_col >= n as i32 {
                    continue;
                }
                
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                
                // 隣接セルが土地の場合、マージを試みる
                if grid[new_row][new_col] {
                    let neighbor_idx = (new_row * n + new_col) as i32;
                    
                    // マージに成功した場合、島の数を減らす
                    if Self::union(&mut parent, &mut rank, current_idx, neighbor_idx) {
                        island_count -= 1;
                    }
                }
            }
            
            result.push(island_count);
        }
        
        result
    }
    
    // Union-Find: 親を見つける（パス圧縮付き）
    fn find(parent: &mut Vec<i32>, x: i32) -> i32 {
        if parent[x as usize] != x {
            parent[x as usize] = Self::find(parent, parent[x as usize]);
        }
        parent[x as usize]
    }
    
    // Union-Find: 2つの集合を統合（ランクによる統合）
    // 統合に成功した場合true、既に同じ集合に属している場合falseを返す
    fn union(parent: &mut Vec<i32>, rank: &mut Vec<i32>, x: i32, y: i32) -> bool {
        let root_x = Self::find(parent, x);
        let root_y = Self::find(parent, y);
        
        // 既に同じ集合に属している場合
        if root_x == root_y {
            return false;
        }
        
        // ランクに基づいて統合
        if rank[root_x as usize] < rank[root_y as usize] {
            parent[root_x as usize] = root_y;
        } else if rank[root_x as usize] > rank[root_y as usize] {
            parent[root_y as usize] = root_x;
        } else {
            parent[root_y as usize] = root_x;
            rank[root_x as usize] += 1;
        }
        
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let m = 3;
        let n = 3;
        let positions = vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 2],
            vec![2, 1],
        ];
        let result = Solution::num_islands2(m, n, positions);
        assert_eq!(result, vec![1, 1, 2, 3]);
    }

    #[test]
    fn test_example2() {
        let m = 1;
        let n = 1;
        let positions = vec![vec![0, 0]];
        let result = Solution::num_islands2(m, n, positions);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_single_row() {
        let m = 1;
        let n = 3;
        let positions = vec![
            vec![0, 0],
            vec![0, 1],
            vec![0, 2],
        ];
        let result = Solution::num_islands2(m, n, positions);
        assert_eq!(result, vec![1, 1, 1]); // 全て連結される
    }

    #[test]
    fn test_single_column() {
        let m = 3;
        let n = 1;
        let positions = vec![
            vec![0, 0],
            vec![1, 0],
            vec![2, 0],
        ];
        let result = Solution::num_islands2(m, n, positions);
        assert_eq!(result, vec![1, 1, 1]); // 全て連結される
    }

    #[test]
    fn test_disconnected_islands() {
        let m = 3;
        let n = 3;
        let positions = vec![
            vec![0, 0],
            vec![0, 2],
            vec![2, 0],
            vec![2, 2],
        ];
        let result = Solution::num_islands2(m, n, positions);
        assert_eq!(result, vec![1, 2, 3, 4]); // 全て独立した島
    }

    #[test]
    fn test_merge_islands() {
        let m = 3;
        let n = 3;
        let positions = vec![
            vec![0, 0],
            vec![0, 2],
            vec![1, 1], // これで2つの島が連結される
        ];
        let result = Solution::num_islands2(m, n, positions);
        assert_eq!(result, vec![1, 2, 1]); // 最後に1つに統合
    }

    #[test]
    fn test_duplicate_position() {
        let m = 2;
        let n = 2;
        let positions = vec![
            vec![0, 0],
            vec![0, 0], // 重複
            vec![0, 1],
        ];
        let result = Solution::num_islands2(m, n, positions);
        assert_eq!(result, vec![1, 1, 1]); // 重複は無視される
    }

    #[test]
    fn test_large_grid() {
        let m = 10;
        let n = 10;
        let positions = vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 1],
        ];
        let result = Solution::num_islands2(m, n, positions);
        assert_eq!(result, vec![1, 1, 1, 1]); // 2x2の島が形成される
    }
}

