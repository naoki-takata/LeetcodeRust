impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        let mut start_times: Vec<i32> = intervals.iter().map(|v| v[0]).collect();
        let mut end_times: Vec<i32> = intervals.iter().map(|v| v[1]).collect();
        
        start_times.sort();
        end_times.sort();
        
        let mut rooms = 0;
        let mut max_rooms = 0;
        let mut start_idx = 0;
        let mut end_idx = 0;
        
        while start_idx < n {
            if start_times[start_idx] < end_times[end_idx] {
                // 新しい会議が開始される
                rooms += 1;
                start_idx += 1;
                max_rooms = max_rooms.max(rooms);
            } else {
                // 会議が終了する
                rooms -= 1;
                end_idx += 1;
            }
        }
        
        max_rooms
    }
}

