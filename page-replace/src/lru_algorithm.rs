// lru - track recently used pages and
// the one which was used least recently used
// meaning first used among the pages stayed in the
// main memory

// main idea to simulate this lru algorithm
// - track the last used/count/time of a page
// - change the page with least count/old time

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PageMetadata {
    page_number: i32,
    last_used: usize, // count not time
}

pub struct LRUCache {
    size: usize, // size of the frame
    time: usize,
    memory: HashMap<i32, PageMetadata>, // size when no of pages in the frame
}

impl LRUCache {
    pub fn new(size: usize) -> Self {
        LRUCache {
            size,
            time: 0,
            memory: HashMap::new(),
        }
    }

    // ask memory to access page
    pub fn access_page(&mut self, page: i32) -> (bool, Option<i32>) {
        self.time += 1;

        //  miss - not found in memory
        //  hit - found in memory
        //  evict - remove page
        match self.memory.get_mut(&page) {
            // page is present in main memory
            Some(meta) => {
                meta.last_used = self.time;
                (false, None)
            }
            None => {
                let mut evicted = None;
                if self.memory.len() == self.size {
                    let lru = self
                        .memory
                        .iter() // go through all hashmap
                        .min_by_key(|(_, meta)| meta.last_used) //find the least last_used count
                        .map(|(page, _)| *page) // extract the page number with least last_used
                        // count
                        .unwrap();
                    self.memory.remove(&lru);
                    evicted = Some(lru);
                }

                self.memory.insert(
                    page,
                    PageMetadata {
                        page_number: page,
                        last_used: self.time,
                    },
                );

                (true, evicted)
            }
        }
    }

    pub fn current_state(&self) -> Vec<i32> {
        let mut pages: Vec<_> = self.memory.values().collect();
        pages.sort_by_key(|meta| meta.last_used); // LRU to MRU
        pages.iter().map(|m| m.page_number).collect()
    }
}
