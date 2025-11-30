// Range Sum Query - Immutable
// 配列の範囲[left, right]の合計を効率的に計算する
// 累積和（prefix sum）を使用して、O(1)で範囲クエリを処理

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`NumArray`構造体が定義されています

struct NumArray {
    prefix_sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix_sum = Vec::with_capacity(nums.len() + 1);
        prefix_sum.push(0); // インデックス0は0で初期化（範囲[0, i]の合計を簡単に計算するため）
        
        let mut sum = 0;
        for num in nums {
            sum += num;
            prefix_sum.push(sum);
        }
        
        NumArray { prefix_sum }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        // prefix_sum[right + 1]は[0, right]の合計
        // prefix_sum[left]は[0, left - 1]の合計
        // したがって、prefix_sum[right + 1] - prefix_sum[left]が[left, right]の合計
        self.prefix_sum[right as usize + 1] - self.prefix_sum[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(num_array.sum_range(0, 2), 1);  // (-2) + 0 + 3 = 1
        assert_eq!(num_array.sum_range(2, 5), -1); // 3 + (-5) + 2 + (-1) = -1
        assert_eq!(num_array.sum_range(0, 5), -3); // (-2) + 0 + 3 + (-5) + 2 + (-1) = -3
    }

    #[test]
    fn test_single_element() {
        let num_array = NumArray::new(vec![5]);
        assert_eq!(num_array.sum_range(0, 0), 5);
    }

    #[test]
    fn test_all_positive() {
        let num_array = NumArray::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(num_array.sum_range(0, 4), 15);
        assert_eq!(num_array.sum_range(1, 3), 9);
    }

    #[test]
    fn test_all_negative() {
        let num_array = NumArray::new(vec![-1, -2, -3, -4, -5]);
        assert_eq!(num_array.sum_range(0, 4), -15);
        assert_eq!(num_array.sum_range(1, 3), -9);
    }

    #[test]
    fn test_mixed() {
        let num_array = NumArray::new(vec![1, -2, 3, -4, 5]);
        assert_eq!(num_array.sum_range(0, 4), 3);
        assert_eq!(num_array.sum_range(0, 1), -1);
        assert_eq!(num_array.sum_range(2, 4), 4);
    }

    #[test]
    fn test_same_index() {
        let num_array = NumArray::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(num_array.sum_range(2, 2), 3);
    }

    #[test]
    fn test_large_range() {
        let nums: Vec<i32> = (1..=1000).collect();
        let num_array = NumArray::new(nums);
        assert_eq!(num_array.sum_range(0, 999), 500500); // 1から1000の合計
        assert_eq!(num_array.sum_range(0, 499), 125250); // 1から500の合計
    }
}

