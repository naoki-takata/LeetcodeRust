impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // ステップ1: すべての要素をXORすると、2つの一意の要素のXOR結果が得られる
        // 同じ要素はXORで0になるため、残るのは2つの一意の要素のXOR
        let xor_all = nums.iter().fold(0, |acc, &x| acc ^ x);
        
        // ステップ2: 最下位セットビットを見つける
        // これは2つの一意の要素が異なるビット位置を持つことを意味する
        let diff_bit = xor_all & (-xor_all);
        
        // ステップ3: そのビット位置に基づいて配列を2つのグループに分ける
        // グループ1: そのビットがセットされている要素
        // グループ2: そのビットがセットされていない要素
        let mut group1 = 0;
        let mut group2 = 0;
        
        for &num in &nums {
            if num & diff_bit != 0 {
                // グループ1: そのビットがセットされている要素をXOR
                group1 ^= num;
            } else {
                // グループ2: そのビットがセットされていない要素をXOR
                group2 ^= num;
            }
        }
        
        vec![group1, group2]
    }
}

