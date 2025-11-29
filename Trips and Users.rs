use std::collections::HashMap;

impl Solution {
    // トリップとユーザーのデータを受け取り、キャンセル率を計算する
    // trips: (id, client_id, driver_id, status, request_at)
    //   status: "completed", "cancelled_by_driver", "cancelled_by_client"
    // users: (users_id, banned)
    //   banned: "Yes" or "No"
    pub fn trips_and_users(
        trips: Vec<(i32, i32, i32, String, String)>,
        users: Vec<(i32, String)>
    ) -> Vec<(String, f64)> {
        // ユーザーIDからbanned状態へのマッピングを作成
        let mut user_banned: HashMap<i32, bool> = HashMap::new();
        for (users_id, banned) in users {
            user_banned.insert(users_id, banned == "Yes");
        }
        
        // 日付ごとにトリップをグループ化
        let mut daily_trips: HashMap<String, Vec<(i32, i32, String)>> = HashMap::new();
        for (id, client_id, driver_id, status, request_at) in trips {
            if request_at >= "2013-10-01".to_string() && request_at <= "2013-10-03".to_string() {
                daily_trips.entry(request_at)
                    .or_insert_with(Vec::new)
                    .push((client_id, driver_id, status));
            }
        }
        
        // 日付ごとにキャンセル率を計算
        let mut result: Vec<(String, f64)> = Vec::new();
        let mut dates: Vec<String> = daily_trips.keys().cloned().collect();
        dates.sort();
        
        for date in dates {
            let trips_for_date = &daily_trips[&date];
            
            let mut total_unbanned = 0;
            let mut cancelled_unbanned = 0;
            
            for (client_id, driver_id, status) in trips_for_date {
                // クライアントとドライバーの両方がbannedされていないかチェック
                let client_banned = user_banned.get(client_id).copied().unwrap_or(true);
                let driver_banned = user_banned.get(driver_id).copied().unwrap_or(true);
                
                if !client_banned && !driver_banned {
                    total_unbanned += 1;
                    if status == "cancelled_by_driver" || status == "cancelled_by_client" {
                        cancelled_unbanned += 1;
                    }
                }
            }
            
            // キャンセル率を計算（2桁の小数点で丸める）
            let cancellation_rate = if total_unbanned > 0 {
                let rate = cancelled_unbanned as f64 / total_unbanned as f64;
                (rate * 100.0).round() / 100.0
            } else {
                0.0
            };
            
            result.push((date, cancellation_rate));
        }
        
        result
    }
}

