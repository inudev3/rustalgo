use std::collections::{HashMap, BTreeMap};
use std::ops::Bound;
struct TimeMap {
    timemap: HashMap<String, BTreeMap<i32, String>>,
}


/**
  * `&self` means the method takes an immutable reference
  * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        Self{
            timemap:HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.timemap.entry(key).or_insert(BTreeMap::new()).insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.timemap.get(&key).map_or_else(
            || "".to_string(),
            |treemap| {
                match treemap.range(..=timestamp).next_back() {
                    Some((_, value)) => value.clone(),
                    None => "".to_string(),
                }
            },
        )
    }

}