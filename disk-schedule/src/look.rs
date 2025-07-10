// things to note for look algorithm
// - inital position of head
// - direction where to move first to scan (left means decreasing order)
// - request position vector

pub fn look(requests: &[i32], head_start: i32, direction_left: bool) -> (Vec<i32>, i32) {
    let mut seek_sequence = Vec::new();
    let mut total_seek = 0;
    let mut head = head_start;

    // seperating request which are in left and right
    let mut left: Vec<i32> = requests.iter().cloned().filter(|&r| r < head).collect();
    let mut right: Vec<i32> = requests.iter().cloned().filter(|&r| r >= head).collect();

    left.sort();
    right.sort();

    if direction_left {
        // Move towards lower requests first
        for &req in left.iter().rev() {
            seek_sequence.push(req);
            total_seek += (head - req).abs();
            head = req;
        }
        // Then reverse direction and take right requests in ascending order
        for &req in &right {
            seek_sequence.push(req);
            total_seek += (head - req).abs();
            head = req;
        }
    } else {
        // Move towards higher requests first
        for &req in &right {
            seek_sequence.push(req);
            total_seek += (head - req).abs();
            head = req;
        }
        // Then reverse direction and take left requests in descending order
        for &req in left.iter().rev() {
            seek_sequence.push(req);
            total_seek += (head - req).abs();
            head = req;
        }
    }

    (seek_sequence, total_seek)
}
