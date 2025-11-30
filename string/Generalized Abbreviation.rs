// Generalized Abbreviation
// 文字列の任意の非重複・非隣接の部分文字列をその長さに置き換える
// 例："word" → ["4","3d","2r1","2rd","1o2","1o1d","1or1","1ord","w3","w2d","w1r1","w1rd","wo2","wo1d","wor1","word"]
//
// 解法：
// バックトラッキングを使用して、各位置で文字を保持するか、数値に置き換えるかを決定
// - 文字を保持する場合：現在の数値カウントがあれば追加してから文字を追加
// - 数値に置き換える場合：数値カウントを増やす（連続する文字をグループ化）

impl Solution {
    pub fn generate_abbreviations(word: String) -> Vec<String> {
        let chars: Vec<char> = word.chars().collect();
        let mut result = Vec::new();
        let mut current = String::new();
        
        Self::backtrack(&chars, 0, 0, &mut current, &mut result);
        
        result
    }
    
    fn backtrack(
        chars: &[char],
        index: usize,
        count: usize,  // 現在の連続する文字の数値カウント
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        // すべての文字を処理した場合
        if index == chars.len() {
            let mut abbr = current.clone();
            // 最後に数値カウントが残っている場合は追加
            if count > 0 {
                abbr.push_str(&count.to_string());
            }
            result.push(abbr);
            return;
        }
        
        // 現在の状態を保存（バックトラッキング用）
        let current_len = current.len();
        
        // 選択肢1: 現在の文字を数値に置き換える（カウントを増やす）
        Self::backtrack(chars, index + 1, count + 1, current, result);
        
        // 選択肢2: 現在の文字を保持する
        // 現在の数値カウントがあれば追加
        if count > 0 {
            current.push_str(&count.to_string());
        }
        current.push(chars[index]);
        
        Self::backtrack(chars, index + 1, 0, current, result);
        
        // バックトラッキング：currentを元の状態に戻す
        current.truncate(current_len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = Solution::generate_abbreviations("word".to_string());
        let mut expected = vec![
            "4", "3d", "2r1", "2rd", "1o2", "1o1d", "1or1", "1ord",
            "w3", "w2d", "w1r1", "w1rd", "wo2", "wo1d", "wor1", "word"
        ];
        
        // 順序は任意なので、ソートして比較
        let mut result_sorted: Vec<String> = result.into_iter().collect();
        result_sorted.sort();
        let mut expected_sorted: Vec<String> = expected.into_iter().map(|s| s.to_string()).collect();
        expected_sorted.sort();
        
        assert_eq!(result_sorted, expected_sorted);
    }

    #[test]
    fn test_example2() {
        let result = Solution::generate_abbreviations("a".to_string());
        let mut expected = vec!["1", "a"];
        
        // 順序は任意なので、ソートして比較
        let mut result_sorted: Vec<String> = result.into_iter().collect();
        result_sorted.sort();
        let mut expected_sorted: Vec<String> = expected.into_iter().map(|s| s.to_string()).collect();
        expected_sorted.sort();
        
        assert_eq!(result_sorted, expected_sorted);
    }

    #[test]
    fn test_short_string() {
        let result = Solution::generate_abbreviations("ab".to_string());
        let mut expected = vec!["2", "1b", "a1", "ab"];
        
        let mut result_sorted: Vec<String> = result.into_iter().collect();
        result_sorted.sort();
        let mut expected_sorted: Vec<String> = expected.into_iter().map(|s| s.to_string()).collect();
        expected_sorted.sort();
        
        assert_eq!(result_sorted, expected_sorted);
    }
}

