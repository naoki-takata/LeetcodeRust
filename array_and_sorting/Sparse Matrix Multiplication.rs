// Sparse Matrix Multiplication
// 2つのスパース行列 mat1 (m x k) と mat2 (k x n) の乗算結果を返す
// スパース行列の特性を活かして、0でない要素のみを処理することで効率化

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`Solution`構造体が定義されています

impl Solution {
    pub fn multiply(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat1.len();
        let k = mat1[0].len();
        let n = mat2[0].len();
        
        // 結果行列を初期化 (m x n)
        let mut result = vec![vec![0; n]; m];
        
        // mat1の各行について
        for i in 0..m {
            // mat1の各行の各要素について
            for j in 0..k {
                // 0でない要素のみを処理（スパース行列の最適化）
                if mat1[i][j] != 0 {
                    // mat2の対応する行の各列と掛け合わせる
                    for l in 0..n {
                        result[i][l] += mat1[i][j] * mat2[j][l];
                    }
                }
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mat1 = vec![
            vec![1, 0, 0],
            vec![-1, 0, 3],
        ];
        let mat2 = vec![
            vec![7, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 1],
        ];
        let expected = vec![
            vec![7, 0, 0],
            vec![-7, 0, 3],
        ];
        assert_eq!(Solution::multiply(mat1, mat2), expected);
    }

    #[test]
    fn test_example2() {
        let mat1 = vec![vec![0]];
        let mat2 = vec![vec![0]];
        let expected = vec![vec![0]];
        assert_eq!(Solution::multiply(mat1, mat2), expected);
    }

    #[test]
    fn test_single_element_positive() {
        let mat1 = vec![vec![2]];
        let mat2 = vec![vec![3]];
        let expected = vec![vec![6]];
        assert_eq!(Solution::multiply(mat1, mat2), expected);
    }

    #[test]
    fn test_single_element_negative() {
        let mat1 = vec![vec![-2]];
        let mat2 = vec![vec![3]];
        let expected = vec![vec![-6]];
        assert_eq!(Solution::multiply(mat1, mat2), expected);
    }

    #[test]
    fn test_2x2_matrices() {
        let mat1 = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        let mat2 = vec![
            vec![5, 6],
            vec![7, 8],
        ];
        let expected = vec![
            vec![19, 22],  // 1*5 + 2*7 = 19, 1*6 + 2*8 = 22
            vec![43, 50],  // 3*5 + 4*7 = 43, 3*6 + 4*8 = 50
        ];
        assert_eq!(Solution::multiply(mat1, mat2), expected);
    }

    #[test]
    fn test_sparse_matrices() {
        let mat1 = vec![
            vec![0, 1, 0],
            vec![2, 0, 0],
        ];
        let mat2 = vec![
            vec![0, 0],
            vec![3, 0],
            vec![0, 4],
        ];
        let expected = vec![
            vec![3, 0],  // 0*0 + 1*3 + 0*0 = 3, 0*0 + 1*0 + 0*4 = 0
            vec![0, 0],  // 2*0 + 0*3 + 0*0 = 0, 2*0 + 0*0 + 0*4 = 0
        ];
        assert_eq!(Solution::multiply(mat1, mat2), expected);
    }

    #[test]
    fn test_all_zeros() {
        let mat1 = vec![
            vec![0, 0],
            vec![0, 0],
        ];
        let mat2 = vec![
            vec![0, 0],
            vec![0, 0],
        ];
        let expected = vec![
            vec![0, 0],
            vec![0, 0],
        ];
        assert_eq!(Solution::multiply(mat1, mat2), expected);
    }
}

