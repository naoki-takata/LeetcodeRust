impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let chars: Vec<char> = num.chars().collect();
        let n = chars.len();
        
        // 最初の2つの数字の長さを試す
        // 最初の数字: [0..i)
        // 2番目の数字: [i..i+j)
        for i in 1..=n / 2 {
            for j in 1..=(n - i) / 2 {
                // 先頭0をチェック
                if i > 1 && chars[0] == '0' {
                    continue;
                }
                if j > 1 && chars[i] == '0' {
                    continue;
                }
                
                // 最初の2つの数字を取得
                let first = Self::parse_number(&chars[0..i]);
                let second = Self::parse_number(&chars[i..i + j]);
                
                // 残りの文字列で加法的な数列を形成できるかチェック
                if Self::is_valid_sequence(&chars, i + j, first, second) {
                    return true;
                }
            }
        }
        
        false
    }
    
    fn parse_number(chars: &[char]) -> u128 {
        chars.iter()
            .fold(0u128, |acc, &ch| {
                acc * 10 + (ch as u128 - '0' as u128)
            })
    }
    
    fn is_valid_sequence(
        chars: &[char],
        start: usize,
        prev: u128,
        curr: u128,
    ) -> bool {
        // 残りの文字列がない場合、少なくとも3つの数字が必要なので無効
        if start == chars.len() {
            return false;
        }
        
        // 次の数字は prev + curr
        let next = prev + curr;
        let next_str = next.to_string();
        let next_chars: Vec<char> = next_str.chars().collect();
        
        // 残りの文字列が次の数字の長さに足りない場合、無効
        if start + next_chars.len() > chars.len() {
            return false;
        }
        
        // 先頭0をチェック（"0"自体はOK、それ以外で先頭が0の場合は無効）
        if next_chars.len() > 1 && chars[start] == '0' {
            return false;
        }
        
        // 残りの文字列が次の数字で始まるかチェック
        for (i, &ch) in next_chars.iter().enumerate() {
            if chars[start + i] != ch {
                return false;
            }
        }
        
        // 次の数字が一致した場合
        let new_start = start + next_chars.len();
        if new_start == chars.len() {
            // すべての文字を使い切った場合、成功（少なくとも3つの数字が確定）
            return true;
        }
        
        // 再帰的に次の数字をチェック
        Self::is_valid_sequence(chars, new_start, curr, next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::is_additive_number("112358".to_string()),
            true
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::is_additive_number("199100199".to_string()),
            true
        );
    }

    #[test]
    fn test_leading_zero() {
        assert_eq!(
            Solution::is_additive_number("1023".to_string()),
            false
        );
    }

    #[test]
    fn test_leading_zero2() {
        assert_eq!(
            Solution::is_additive_number("1203".to_string()),
            false
        );
    }

    #[test]
    fn test_short_string() {
        assert_eq!(
            Solution::is_additive_number("12".to_string()),
            false
        );
    }

    #[test]
    fn test_three_digits() {
        assert_eq!(
            Solution::is_additive_number("123".to_string()),
            true
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(
            Solution::is_additive_number("1991001991".to_string()),
            false
        );
    }
}

