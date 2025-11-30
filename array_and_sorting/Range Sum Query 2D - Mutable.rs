// Range Sum Query 2D - Mutable
// 2D行列の要素を更新し、矩形範囲[row1, col1]から[row2, col2]の合計を効率的に計算する
// 2D Binary Indexed Tree (Fenwick Tree)を使用して、更新とクエリをO(log m * log n)で処理

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`NumMatrix`構造体が定義されています

struct NumMatrix {
    rows: usize,
    cols: usize,
    // 元の行列の値を保持
    matrix: Vec<Vec<i32>>,
    // 2D Binary Indexed Tree
    bit: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let rows = matrix.len();
        let cols = if rows > 0 { matrix[0].len() } else { 0 };
        
        let mut num_matrix = NumMatrix {
            rows,
            cols,
            matrix: vec![vec![0; cols]; rows],
            bit: vec![vec![0; cols + 1]; rows + 1],
        };
        
        // 元の行列の値をコピーし、BITを構築
        for i in 0..rows {
            for j in 0..cols {
                num_matrix.update(i as i32, j as i32, matrix[i][j]);
            }
        }
        
        num_matrix
    }
    
    // セル(row, col)の値をvalに更新
    fn update(&mut self, row: i32, col: i32, val: i32) {
        let row = row as usize;
        let col = col as usize;
        
        let diff = val - self.matrix[row][col];
        self.matrix[row][col] = val;
        
        // BITを更新（1-indexedなので+1）
        self.update_bit(row + 1, col + 1, diff);
    }
    
    // BITを更新する（1-indexed）
    // (i, j)の位置にdiffを加算
    fn update_bit(&mut self, mut i: usize, mut j: usize, diff: i32) {
        while i <= self.rows {
            let mut j_temp = j;
            while j_temp <= self.cols {
                self.bit[i][j_temp] += diff;
                j_temp += j_temp & j_temp.wrapping_neg(); // lowbitを加算
            }
            i += i & i.wrapping_neg(); // lowbitを加算
        }
    }
    
    // (0, 0)から(row, col)までの矩形の合計を取得（1-indexed）
    fn query_bit(&self, mut i: usize, mut j: usize) -> i32 {
        let mut sum = 0;
        while i > 0 {
            let mut j_temp = j;
            while j_temp > 0 {
                sum += self.bit[i][j_temp];
                j_temp -= j_temp & j_temp.wrapping_neg(); // lowbitを減算
            }
            i -= i & i.wrapping_neg(); // lowbitを減算
        }
        sum
    }
    
    // 矩形範囲[row1, col1]から[row2, col2]の合計を取得
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;
        
        // 包含-排除原理を使用
        // 全体の矩形 - 上の矩形 - 左の矩形 + 重複部分
        self.query_bit(row2 + 1, col2 + 1)           // (0,0)から(row2,col2)までの合計
            - self.query_bit(row1, col2 + 1)         // 上の矩形を引く
            - self.query_bit(row2 + 1, col1)         // 左の矩形を引く
            + self.query_bit(row1, col1)             // 重複部分を足す
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
        let mut num_matrix = NumMatrix::new(matrix);
        
        assert_eq!(num_matrix.sum_region(2, 1, 4, 3), 8);   // 初期状態
        num_matrix.update(3, 2, 2);                         // matrix[3][2]を0から2に更新
        assert_eq!(num_matrix.sum_region(2, 1, 4, 3), 10);  // 更新後
    }

    #[test]
    fn test_single_element() {
        let matrix = vec![vec![5]];
        let mut num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(0, 0, 0, 0), 5);
        num_matrix.update(0, 0, 10);
        assert_eq!(num_matrix.sum_region(0, 0, 0, 0), 10);
    }

    #[test]
    fn test_entire_matrix() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let mut num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(0, 0, 2, 2), 45); // 1+2+3+4+5+6+7+8+9 = 45
        num_matrix.update(1, 1, 10);
        assert_eq!(num_matrix.sum_region(0, 0, 2, 2), 50); // 45 - 5 + 10 = 50
    }

    #[test]
    fn test_single_row() {
        let matrix = vec![
            vec![1, 2, 3, 4, 5],
        ];
        let mut num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(0, 1, 0, 3), 9); // 2+3+4 = 9
        num_matrix.update(0, 2, 10);
        assert_eq!(num_matrix.sum_region(0, 1, 0, 3), 16); // 2+10+4 = 16
    }

    #[test]
    fn test_single_column() {
        let matrix = vec![
            vec![1],
            vec![2],
            vec![3],
            vec![4],
        ];
        let mut num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(1, 0, 3, 0), 9); // 2+3+4 = 9
        num_matrix.update(2, 0, 10);
        assert_eq!(num_matrix.sum_region(1, 0, 3, 0), 16); // 2+10+4 = 16
    }

    #[test]
    fn test_negative_numbers() {
        let matrix = vec![
            vec![-1, -2],
            vec![-3, -4],
        ];
        let mut num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(0, 0, 1, 1), -10);
        num_matrix.update(0, 0, 0);
        assert_eq!(num_matrix.sum_region(0, 0, 1, 1), -9); // 0 + (-2) + (-3) + (-4) = -9
    }

    #[test]
    fn test_multiple_updates() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let mut num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(0, 0, 2, 2), 45);
        num_matrix.update(0, 0, 10);
        assert_eq!(num_matrix.sum_region(0, 0, 2, 2), 54); // 45 - 1 + 10 = 54
        num_matrix.update(1, 1, 20);
        assert_eq!(num_matrix.sum_region(0, 0, 2, 2), 69); // 54 - 5 + 20 = 69
        num_matrix.update(2, 2, 30);
        assert_eq!(num_matrix.sum_region(0, 0, 2, 2), 90); // 69 - 9 + 30 = 90
    }

    #[test]
    fn test_partial_region() {
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ];
        let mut num_matrix = NumMatrix::new(matrix);
        assert_eq!(num_matrix.sum_region(1, 1, 2, 2), 34); // 6+7+10+11 = 34
        num_matrix.update(1, 1, 20);
        assert_eq!(num_matrix.sum_region(1, 1, 2, 2), 48); // 20+7+10+11 = 48
    }
}

