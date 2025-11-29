use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        // グラフの構築: 文字 -> その文字の後に来る文字のリスト
        let mut graph: HashMap<char, Vec<char>> = HashMap::new();
        // 各文字の入次数をカウント
        let mut in_degree: HashMap<char, i32> = HashMap::new();
        
        // すべての文字を初期化（出現する文字をすべて記録）
        for word in &words {
            for ch in word.chars() {
                graph.entry(ch).or_insert_with(Vec::new);
                in_degree.entry(ch).or_insert(0);
            }
        }
        
        // 隣接する単語のペアを比較して順序関係を構築
        for i in 0..words.len() - 1 {
            let word1 = &words[i];
            let word2 = &words[i + 1];
            
            // 例: "abc" が "ab" の前に来る場合、無効（短い単語が長い単語の前に来ることはできない）
            if word1.len() > word2.len() && word1.starts_with(word2) {
                return String::new();
            }
            
            // 最初に異なる文字を見つける
            let min_len = word1.len().min(word2.len());
            for j in 0..min_len {
                let ch1 = word1.chars().nth(j).unwrap();
                let ch2 = word2.chars().nth(j).unwrap();
                
                if ch1 != ch2 {
                    // ch1 は ch2 の前に来る
                    graph.entry(ch1).or_insert_with(Vec::new).push(ch2);
                    *in_degree.entry(ch2).or_insert(0) += 1;
                    break; // 最初の異なる文字のみが重要
                }
            }
        }
        
        // Kahn's algorithm（トポロジカルソート）
        let mut queue: VecDeque<char> = VecDeque::new();
        
        // 入次数が0の文字をキューに追加
        for (&ch, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(ch);
            }
        }
        
        let mut result = String::new();
        
        while let Some(ch) = queue.pop_front() {
            result.push(ch);
            
            // この文字の後に来る文字の入次数を減らす
            if let Some(neighbors) = graph.get(&ch) {
                for &neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(&neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
        
        // すべての文字が結果に含まれているかチェック
        // 含まれていない場合、サイクルが存在する（矛盾がある）
        if result.len() != in_degree.len() {
            return String::new();
        }
        
        result
    }
}

