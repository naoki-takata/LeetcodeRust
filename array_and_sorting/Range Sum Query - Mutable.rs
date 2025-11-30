// Range Sum Query - Mutable
// 配列の要素を更新し、範囲[left, right]の合計を効率的に計算する
// Segment Treeを使用して、更新とクエリをO(log n)で処理

// 注意: LeetCode提出時は以下の行を削除またはコメントアウトしてください
// LeetCode環境では既に`NumArray`構造体が定義されています

struct NumArray {
    n: usize,
    tree: Vec<i32>,
    nums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        // Segment Treeのサイズは元の配列の4倍（安全のため）
        let mut tree = vec![0; 4 * n];
        let nums_clone = nums.clone();
        
        let mut num_array = NumArray {
            n,
            tree,
            nums: nums_clone,
        };
        
        // Segment Treeを構築
        num_array.build(0, 0, n - 1);
        num_array
    }
    
    // Segment Treeを構築する（再帰的）
    // node: 現在のノードのインデックス
    // start, end: 現在のノードがカバーする範囲
    fn build(&mut self, node: usize, start: usize, end: usize) {
        if start == end {
            // リーフノード
            self.tree[node] = self.nums[start];
        } else {
            let mid = (start + end) / 2;
            let left_child = 2 * node + 1;
            let right_child = 2 * node + 2;
            
            // 左の子を構築
            self.build(left_child, start, mid);
            // 右の子を構築
            self.build(right_child, mid + 1, end);
            
            // 現在のノードの値は左右の子の和
            self.tree[node] = self.tree[left_child] + self.tree[right_child];
        }
    }
    
    // インデックスindexの値をvalに更新
    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let diff = val - self.nums[index];
        self.nums[index] = val;
        self.update_tree(0, 0, self.n - 1, index, diff);
    }
    
    // Segment Treeを更新する（再帰的）
    // node: 現在のノードのインデックス
    // start, end: 現在のノードがカバーする範囲
    // index: 更新する配列のインデックス
    // diff: 値の変化量
    fn update_tree(&mut self, node: usize, start: usize, end: usize, index: usize, diff: i32) {
        if start > index || end < index {
            // 現在のノードの範囲にindexが含まれていない
            return;
        }
        
        // 現在のノードを更新
        self.tree[node] += diff;
        
        if start != end {
            // リーフノードでない場合、子ノードも更新
            let mid = (start + end) / 2;
            let left_child = 2 * node + 1;
            let right_child = 2 * node + 2;
            
            self.update_tree(left_child, start, mid, index, diff);
            self.update_tree(right_child, mid + 1, end, index, diff);
        }
    }
    
    // 範囲[left, right]の合計を取得
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.query(0, 0, self.n - 1, left as usize, right as usize)
    }
    
    // Segment Treeから範囲クエリを実行する（再帰的）
    // node: 現在のノードのインデックス
    // start, end: 現在のノードがカバーする範囲
    // left, right: クエリの範囲
    fn query(&self, node: usize, start: usize, end: usize, left: usize, right: usize) -> i32 {
        if right < start || left > end {
            // 現在のノードの範囲がクエリ範囲と重複していない
            return 0;
        }
        
        if left <= start && end <= right {
            // 現在のノードの範囲がクエリ範囲に完全に含まれている
            return self.tree[node];
        }
        
        // 現在のノードの範囲がクエリ範囲と部分的に重複している
        let mid = (start + end) / 2;
        let left_child = 2 * node + 1;
        let right_child = 2 * node + 2;
        
        let left_sum = self.query(left_child, start, mid, left, right);
        let right_sum = self.query(right_child, mid + 1, end, left, right);
        
        left_sum + right_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut num_array = NumArray::new(vec![1, 3, 5]);
        assert_eq!(num_array.sum_range(0, 2), 9);  // 1 + 3 + 5 = 9
        num_array.update(1, 2);  // nums = [1, 2, 5]
        assert_eq!(num_array.sum_range(0, 2), 8);  // 1 + 2 + 5 = 8
    }

    #[test]
    fn test_single_element() {
        let mut num_array = NumArray::new(vec![5]);
        assert_eq!(num_array.sum_range(0, 0), 5);
        num_array.update(0, 10);
        assert_eq!(num_array.sum_range(0, 0), 10);
    }

    #[test]
    fn test_multiple_updates() {
        let mut num_array = NumArray::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(num_array.sum_range(0, 4), 15);
        num_array.update(2, 10);
        assert_eq!(num_array.sum_range(0, 4), 22);  // 1 + 2 + 10 + 4 + 5 = 22
        assert_eq!(num_array.sum_range(1, 3), 16);  // 2 + 10 + 4 = 16
        num_array.update(0, 0);
        assert_eq!(num_array.sum_range(0, 4), 21);  // 0 + 2 + 10 + 4 + 5 = 21
    }

    #[test]
    fn test_negative_numbers() {
        let mut num_array = NumArray::new(vec![-1, -2, -3, -4, -5]);
        assert_eq!(num_array.sum_range(0, 4), -15);
        num_array.update(2, 0);
        assert_eq!(num_array.sum_range(0, 4), -12);  // -1 + -2 + 0 + -4 + -5 = -12
    }

    #[test]
    fn test_mixed_numbers() {
        let mut num_array = NumArray::new(vec![1, -2, 3, -4, 5]);
        assert_eq!(num_array.sum_range(0, 4), 3);
        num_array.update(1, 2);
        assert_eq!(num_array.sum_range(0, 4), 7);  // 1 + 2 + 3 + -4 + 5 = 7
    }

    #[test]
    fn test_same_index() {
        let mut num_array = NumArray::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(num_array.sum_range(2, 2), 3);
        num_array.update(2, 10);
        assert_eq!(num_array.sum_range(2, 2), 10);
    }

    #[test]
    fn test_partial_range() {
        let mut num_array = NumArray::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(num_array.sum_range(1, 3), 9);  // 2 + 3 + 4 = 9
        num_array.update(2, 10);
        assert_eq!(num_array.sum_range(1, 3), 16);  // 2 + 10 + 4 = 16
    }
}

