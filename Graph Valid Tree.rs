impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        // 木であるための必要条件: n個のノードに対してn-1本のエッジが必要
        if edges.len() != (n - 1) as usize {
            return false;
        }
        
        // Union-Findデータ構造を使用してサイクルを検出
        let mut parent: Vec<i32> = (0..n).collect();
        let mut rank: Vec<i32> = vec![0; n as usize];
        
        // 各エッジを処理
        for edge in edges {
            let u = edge[0];
            let v = edge[1];
            
            // 同じ親を持つ場合、サイクルが存在する
            if !Self::union(&mut parent, &mut rank, u, v) {
                return false;
            }
        }
        
        // すべてのノードが同じ親を持つかチェック（連結性の確認）
        let root = Self::find(&mut parent, 0);
        for i in 1..n {
            if Self::find(&mut parent, i) != root {
                return false;
            }
        }
        
        true
    }
    
    // Union-Find: 親を見つける（パス圧縮付き）
    fn find(parent: &mut Vec<i32>, x: i32) -> i32 {
        if parent[x as usize] != x {
            parent[x as usize] = Self::find(parent, parent[x as usize]);
        }
        parent[x as usize]
    }
    
    // Union-Find: 2つの集合を統合（ランクによる統合）
    fn union(parent: &mut Vec<i32>, rank: &mut Vec<i32>, x: i32, y: i32) -> bool {
        let root_x = Self::find(parent, x);
        let root_y = Self::find(parent, y);
        
        // 同じ親を持つ場合、サイクルが存在する
        if root_x == root_y {
            return false;
        }
        
        // ランクに基づいて統合
        if rank[root_x as usize] < rank[root_y as usize] {
            parent[root_x as usize] = root_y;
        } else if rank[root_x as usize] > rank[root_y as usize] {
            parent[root_y as usize] = root_x;
        } else {
            parent[root_y as usize] = root_x;
            rank[root_x as usize] += 1;
        }
        
        true
    }
}

