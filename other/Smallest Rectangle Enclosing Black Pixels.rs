// Smallest Rectangle Enclosing Black Pixels
// m x n のバイナリマトリックスで、接続された黒いピクセル（1）を囲む最小の軸整列矩形の面積を求める
// O(mn)未満の時間計算量で解く必要があるため、バイナリサーチを使用

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`Solution`構造体が定義されています

impl Solution {
    pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        let m = image.len() as i32;
        let n = image[0].len() as i32;
        
        // バイナリサーチで最小行（top）を見つける
        // 行0からxまでで、黒いピクセルが存在する最小の行を見つける
        let top = Self::search_rows(&image, 0, x, true);
        
        // バイナリサーチで最大行（bottom）を見つける
        // 行xからm-1までで、黒いピクセルが存在する最大の行を見つける
        let bottom = Self::search_rows(&image, x, m - 1, false);
        
        // バイナリサーチで最小列（left）を見つける
        // 列0からyまでで、黒いピクセルが存在する最小の列を見つける
        let left = Self::search_cols(&image, 0, y, true);
        
        // バイナリサーチで最大列（right）を見つける
        // 列yからn-1までで、黒いピクセルが存在する最大の列を見つける
        let right = Self::search_cols(&image, y, n - 1, false);
        
        // 矩形の面積を計算
        (bottom - top + 1) * (right - left + 1)
    }
    
    // 行方向のバイナリサーチ
    // search_first: trueの場合は最小行を、falseの場合は最大行を見つける
    fn search_rows(image: &Vec<Vec<char>>, mut left: i32, mut right: i32, search_first: bool) -> i32 {
        while left < right {
            let mid = if search_first {
                left + (right - left) / 2
            } else {
                left + (right - left + 1) / 2  // 最大値を探す場合は右側に寄せる
            };
            
            if Self::has_black_in_row(image, mid) {
                if search_first {
                    right = mid;  // 最小行を探す場合、midより前を探す
                } else {
                    left = mid;   // 最大行を探す場合、mid以降を探す
                }
            } else {
                if search_first {
                    left = mid + 1;  // 最小行を探す場合、midより後を探す
                } else {
                    right = mid - 1; // 最大行を探す場合、midより前を探す
                }
            }
        }
        left
    }
    
    // 列方向のバイナリサーチ
    // search_first: trueの場合は最小列を、falseの場合は最大列を見つける
    fn search_cols(image: &Vec<Vec<char>>, mut left: i32, mut right: i32, search_first: bool) -> i32 {
        while left < right {
            let mid = if search_first {
                left + (right - left) / 2
            } else {
                left + (right - left + 1) / 2  // 最大値を探す場合は右側に寄せる
            };
            
            if Self::has_black_in_col(image, mid) {
                if search_first {
                    right = mid;  // 最小列を探す場合、midより前を探す
                } else {
                    left = mid;   // 最大列を探す場合、mid以降を探す
                }
            } else {
                if search_first {
                    left = mid + 1;  // 最小列を探す場合、midより後を探す
                } else {
                    right = mid - 1; // 最大列を探す場合、midより前を探す
                }
            }
        }
        left
    }
    
    // 指定された行に黒いピクセルが存在するかどうかを確認
    fn has_black_in_row(image: &Vec<Vec<char>>, row: i32) -> bool {
        let row = row as usize;
        image[row].iter().any(|&pixel| pixel == '1')
    }
    
    // 指定された列に黒いピクセルが存在するかどうかを確認
    fn has_black_in_col(image: &Vec<Vec<char>>, col: i32) -> bool {
        let col = col as usize;
        image.iter().any(|row| row[col] == '1')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let image = vec![
            vec!['0', '0', '1', '0'],
            vec!['0', '1', '1', '0'],
            vec!['0', '1', '0', '0'],
        ];
        assert_eq!(Solution::min_area(image, 0, 2), 6);
    }

    #[test]
    fn test_example2() {
        let image = vec![vec!['1']];
        assert_eq!(Solution::min_area(image, 0, 0), 1);
    }

    #[test]
    fn test_single_row() {
        let image = vec![vec!['0', '1', '1', '0']];
        assert_eq!(Solution::min_area(image, 0, 1), 1);
    }

    #[test]
    fn test_single_col() {
        let image = vec![
            vec!['0'],
            vec!['1'],
            vec!['1'],
            vec!['0'],
        ];
        assert_eq!(Solution::min_area(image, 1, 0), 1);
    }

    #[test]
    fn test_full_rectangle() {
        let image = vec![
            vec!['1', '1', '1'],
            vec!['1', '1', '1'],
            vec!['1', '1', '1'],
        ];
        assert_eq!(Solution::min_area(image, 1, 1), 9);
    }
}

