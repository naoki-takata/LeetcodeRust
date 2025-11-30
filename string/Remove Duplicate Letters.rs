use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        // 各文字の最後の出現位置を記録
        let mut last_occurrence: HashMap<char, usize> = HashMap::new();
        for (i, ch) in s.chars().enumerate() {
            last_occurrence.insert(ch, i);
        }
        
        let mut stack: Vec<char> = Vec::new();
        let mut seen: HashSet<char> = HashSet::new();
        
        for (i, ch) in s.chars().enumerate() {
            // 既にスタックに含まれている文字はスキップ
            if seen.contains(&ch) {
                continue;
            }
            
            // スタックのトップが現在の文字より大きく、かつ後で再び現れる場合はポップ
            while let Some(&top) = stack.last() {
                if top > ch && last_occurrence.get(&top).unwrap() > &i {
                    stack.pop();
                    seen.remove(&top);
                } else {
                    break;
                }
            }
            
            // 現在の文字をスタックに追加
            stack.push(ch);
            seen.insert(ch);
        }
        
        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::remove_duplicate_letters("bcabc".to_string()),
            "abc"
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_string()),
            "acdb"
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(
            Solution::remove_duplicate_letters("a".to_string()),
            "a"
        );
    }

    #[test]
    fn test_all_same() {
        assert_eq!(
            Solution::remove_duplicate_letters("aaaa".to_string()),
            "a"
        );
    }

    #[test]
    fn test_no_duplicates() {
        assert_eq!(
            Solution::remove_duplicate_letters("abc".to_string()),
            "abc"
        );
    }
}

