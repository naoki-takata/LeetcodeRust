use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        
        // ノードが1つの場合
        if n == 1 {
            return vec![0];
        }
        
        // グラフを構築（隣接リスト）
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut degree = vec![0; n];
        
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            
            graph.entry(u).or_insert_with(Vec::new).push(v);
            graph.entry(v).or_insert_with(Vec::new).push(u);
            
            degree[u] += 1;
            degree[v] += 1;
        }
        
        // 葉ノード（次数1のノード）をキューに追加
        let mut queue = VecDeque::new();
        for i in 0..n {
            if degree[i] == 1 {
                queue.push_back(i);
            }
        }
        
        // 残りのノード数
        let mut remaining = n;
        
        // 葉ノードを繰り返し削除
        // 最後に1つまたは2つのノードが残るまで続ける
        while remaining > 2 {
            let size = queue.len();
            remaining -= size;
            
            // 現在の葉ノードをすべて処理
            for _ in 0..size {
                let leaf = queue.pop_front().unwrap();
                
                // 葉ノードの隣接ノードを処理
                if let Some(neighbors) = graph.get(&leaf) {
                    for &neighbor in neighbors {
                        degree[neighbor] -= 1;
                        
                        // 新しい葉ノードができたらキューに追加
                        if degree[neighbor] == 1 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
        
        // 残ったノードが最小高さツリーのルート
        queue.into_iter().map(|x| x as i32).collect()
    }
}

