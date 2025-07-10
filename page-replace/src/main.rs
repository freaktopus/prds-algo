// Page replacement : It is method to to replace the page
// from the main memory when the memory is full
// and there is request of page access which is not
// in the main memory

use page_replace::lru_algorithm::LRUCache;

fn main() {
    let page_sequence = vec![7, 0, 1, 2, 0, 3, 0, 4];
    let size = 3;

    let mut lru = LRUCache::new(size);
    let mut faults = 0;

    println!("LRU Page Replacement ({} Frames)", size);
    println!("Pages: {:?}\n", page_sequence);
    println!(
        "{:<8} {:<10} {:<15} {:<10}",
        "Step", "Page", "Action", "Frames (LRU → MRU)"
    );
    println!("{}", "-".repeat(55));

    for (step, &page) in page_sequence.iter().enumerate() {
        let (is_fault, evicted) = lru.access_page(page);
        let action = if is_fault {
            match evicted {
                Some(p) => format!("MISS, Evict {}", p),
                None => "MISS".to_string(),
            }
        } else {
            "HIT".to_string()
        };
        let state = lru.current_state();
        println!("{:<8} {:<10} {:<15} {:?}", step + 1, page, action, state);
        if is_fault {
            faults += 1;
        }
    }

    let hits = page_sequence.len() - faults;
    println!("\nSummary:");
    println!("Total Requests : {}", page_sequence.len());
    println!("Page Faults    : {}", faults);
    println!("Page Hits      : {}", hits);
    println!(
        "Fault Rate     : {:.2}%",
        (faults as f64 / page_sequence.len() as f64) * 100.0
    );
}
