// look is one of the disk_schedule algorithm
// where the disk head position move end to
// the last requested position of either direction
// and again goes to the last requested position of another
// direction

use disk_schedule::look::look;

fn main() {
    let requests = vec![82, 170, 43, 140, 24, 16, 190];
    let head_start = 50;

    println!("LOOK Disk Scheduling Algorithm");
    println!("Request Queue: {:?}", requests);
    println!("Initial Head Position: {}", head_start);
    println!();

    // first direction to scan left
    let (sequence_up, seek_up) = look(&requests, head_start, true);
    println!("Direction: Left");
    println!("Seek Sequence: {:?}", sequence_up);
    println!("Total Seek Distance: {}\n", seek_up);

    // first direction to scan right
    let (sequence_down, seek_down) = look(&requests, head_start, false);
    println!("Direction: Right");
    println!("Seek Sequence: {:?}", sequence_down);
    println!("Total Seek Distance: {}", seek_down);
}
