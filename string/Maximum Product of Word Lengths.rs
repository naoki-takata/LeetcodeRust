// Maximum Product of Word Lengths
// 共通の文字を持たない2つの単語の長さの積の最大値を求める
// ビットマスクを使用して各単語の文字セットを効率的に表現

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let n = words.len();
        if n < 2 {
            return 0;
        }
        
        // 各単語の文字セットをビットマスクで表現
        // 26文字なので、u32で十分（32ビット）
        let mut masks = Vec::with_capacity(n);
        let mut lengths = Vec::with_capacity(n);
        
        for word in &words {
            let mut mask = 0u32;
            for ch in word.chars() {
                // 'a'から'z'を0から25のビット位置にマッピング
                let bit_pos = (ch as u8 - b'a') as u32;
                mask |= 1 << bit_pos;
            }
            masks.push(mask);
            lengths.push(word.len() as i32);
        }
        
        // すべての単語ペアをチェック
        let mut max_product = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                // ビットマスクのANDが0なら、共通文字なし
                if masks[i] & masks[j] == 0 {
                    let product = lengths[i] * lengths[j];
                    max_product = max_product.max(product);
                }
            }
        }
        
        max_product
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let words = vec![
            "abcw".to_string(),
            "baz".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "xtfn".to_string(),
            "abcdef".to_string(),
        ];
        assert_eq!(Solution::max_product(words), 16);
    }

    #[test]
    fn test_example2() {
        let words = vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "d".to_string(),
            "cd".to_string(),
            "bcd".to_string(),
            "abcd".to_string(),
        ];
        assert_eq!(Solution::max_product(words), 4);
    }

    #[test]
    fn test_example3() {
        let words = vec![
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "aaaa".to_string(),
        ];
        assert_eq!(Solution::max_product(words), 0);
    }

    #[test]
    fn test_two_words_no_common() {
        let words = vec!["abc".to_string(), "def".to_string()];
        assert_eq!(Solution::max_product(words), 9);
    }

    #[test]
    fn test_two_words_with_common() {
        let words = vec!["abc".to_string(), "bcd".to_string()];
        assert_eq!(Solution::max_product(words), 0);
    }
}

