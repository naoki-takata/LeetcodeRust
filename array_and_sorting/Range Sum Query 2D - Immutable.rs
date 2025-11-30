// Range Sum Query 2D - Immutable
// 2D行列の矩形範囲[row1, col1]から[row2, col2]の合計を効率的に計算する
// 2D累積和（2D prefix sum）を使用して、O(1)で範囲クエリを処理

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`NumMatrix`構造体が定義されています

struct NumMatrix {
    // dp[i+1][j+1]は(0,0)から(i,j)までの矩形の合計を表す
    dp: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let rows = matrix.len();
        let cols = if rows > 0 { matrix[0].len() } else { 0 };
        
        // dpは(rows+1) x (cols+1)のサイズで、インデックス0は0で初期化
        let mut dp = vec![vec![0; cols + 1]; rows + 1];
        
        // 2D累積和を構築
        // dp[i+1][j+1] = matrix[i][j] + dp[i][j+1] + dp[i+1][j] - dp[i][j]
        for i in 0..rows {
            for j in 0..cols {
                dp[i + 1][j + 1] = matrix[i][j] 
                    + dp[i][j + 1]      // 上の矩形
                    + dp[i + 1][j]      // 左の矩形
                    - dp[i][j];         // 重複部分を引く
            }
        }
        
        NumMatrix { dp }
    }
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;
        
        // 包含-排除原理を使用
        // 全体の矩形 - 上の矩形 - 左の矩形 + 重複部分
        self.dp[row2 + 1][col2 + 1]           // (0,0)から(row2,col2)までの合計
            - self.dp[row1][col2 + 1]         // 上の矩形を引く
            - self.dp[row2 + 1][col1]         // 左の矩形を引く
            + self.dp[row1][col1]             // 重複部分を足す
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let matrix = vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ];
        let num_matrix = NumMatrix::new(matrix);
        
        assert_eq!(num_matrix.sum_region(2, 1, 4, 3), 8);   // 赤い矩形
        assert_eq!(num_matrix.sum_region(1, 1, 2, 2), 11);  // 緑の矩形
        assert_eq!(num_matrix.sum_region(1, 2, 2, 4), 12);  // 青い矩形
    }

    #[test]
    fn test_single_element() {
        let matrix = vec![vec![5]];
        let num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(0, 0, 0, 0), 5);
    }

    #[test]
    fn test_entire_matrix() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(0, 0, 2, 2), 45); // 1+2+3+4+5+6+7+8+9 = 45
    }

    #[test]
    fn test_single_row() {
        let matrix = vec![
            vec![1, 2, 3, 4, 5],
        ];
        let num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(0, 1, 0, 3), 9); // 2+3+4 = 9
    }

    #[test]
    fn test_single_column() {
        let matrix = vec![
            vec![1],
            vec![2],
            vec![3],
            vec![4],
        ];
        let num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(1, 0, 3, 0), 9); // 2+3+4 = 9
    }

    #[test]
    fn test_negative_numbers() {
        let matrix = vec![
            vec![-1, -2],
            vec![-3, -4],
        ];
        let num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(0, 0, 1, 1), -10);
        assert_eq!(num_matrix.sum_region(0, 0, 0, 1), -3);
    }

    #[test]
    fn test_empty_matrix() {
        let matrix: Vec<Vec<i32>> = vec![];
        let num_matrix = NumMatrix::new(matrix);
        // 空行列の場合、sum_regionは0を返す（dpは全て0で初期化されているため）
        assert_eq!(num_matrix.sum_region(0, 0, 0, 0), 0);
    }

    #[test]
    fn test_large_matrix() {
        let mut matrix = vec![vec![0; 100]; 100];
        for i in 0..100 {
            for j in 0..100 {
                matrix[i][j] = (i * 100 + j) as i32;
            }
        }
        let num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(0, 0, 99, 99), 49995000); // 0から9999までの合計
    }
}

