impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        // 各文字の出現回数をカウント
        let mut char_count = std::collections::HashMap::new();
        for ch in s.chars() {
            *char_count.entry(ch).or_insert(0) += 1;
        }
        
        // 奇数回出現する文字の数をカウント
        let odd_count = char_count.values()
            .filter(|&count| count % 2 == 1)
            .count();
        
        // 回文を形成できる条件：
        // - 文字列の長さが偶数の場合：奇数回出現する文字が0個
        // - 文字列の長さが奇数の場合：奇数回出現する文字が1個
        odd_count <= 1
    }

    pub fn generate_palindromes(s: String) -> Vec<String> {
        // 各文字の出現回数をカウント
        let mut char_count = std::collections::HashMap::new();
        for ch in s.chars() {
            *char_count.entry(ch).or_insert(0) += 1;
        }
        
        // 奇数回出現する文字を探す
        let mut odd_char: Option<char> = None;
        let mut half_chars = Vec::new();
        
        for (&ch, &count) in char_count.iter() {
            if count % 2 == 1 {
                if odd_char.is_some() {
                    // 奇数回出現する文字が2つ以上ある場合は回文順列不可
                    return Vec::new();
                }
                odd_char = Some(ch);
            }
            // 各文字を半分の回数だけ追加
            for _ in 0..count / 2 {
                half_chars.push(ch);
            }
        }
        
        // 半分の文字列が空の場合（例：入力が "a" の場合）
        if half_chars.is_empty() {
            // 奇数回出現する文字があれば、それだけを含む回文順列を返す
            if let Some(ch) = odd_char {
                return vec![ch.to_string()];
            }
            return Vec::new();
        }
        
        // 重複を避けるためにソート
        half_chars.sort();
        
        // すべての順列を生成
        let mut result = Vec::new();
        let mut used = vec![false; half_chars.len()];
        let mut current = Vec::new();
        
        Self::backtrack(&half_chars, &mut used, &mut current, &mut result, odd_char);
        
        result
    }
    
    fn backtrack(
        half_chars: &[char],
        used: &mut [bool],
        current: &mut Vec<char>,
        result: &mut Vec<String>,
        odd_char: Option<char>,
    ) {
        // すべての文字を使い切った場合
        if current.len() == half_chars.len() {
            // 回文を構築
            let mut palindrome: Vec<char> = current.clone();
            
            // 奇数回出現する文字があれば中央に追加
            if let Some(ch) = odd_char {
                palindrome.push(ch);
            }
            
            // 逆順を追加して回文を完成
            for i in (0..current.len()).rev() {
                palindrome.push(current[i]);
            }
            
            result.push(palindrome.into_iter().collect());
            return;
        }
        
        // 次の文字を選択
        for i in 0..half_chars.len() {
            // 既に使用済みの場合はスキップ
            if used[i] {
                continue;
            }
            
            // 重複を避ける：同じ文字が連続して選択される場合のみ処理
            // 前の文字と同じで、前の文字が未使用の場合はスキップ
            if i > 0 && half_chars[i] == half_chars[i - 1] && !used[i - 1] {
                continue;
            }
            
            used[i] = true;
            current.push(half_chars[i]);
            
            Self::backtrack(half_chars, used, current, result, odd_char);
            
            current.pop();
            used[i] = false;
        }
    }
}

