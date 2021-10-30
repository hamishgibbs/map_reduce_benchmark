use std::collections::HashMap;
use std::io;
use std::collections::hash_map::Entry;

fn main() {

}

/// Combine map step for all friends of all users.
pub fn combine(hvec: Vec<HashMap<Vec<i32>, Vec<i32>>>) -> HashMap<Vec<i32>, Vec<Vec<i32>>> {

    let mut map_combined: HashMap<Vec<i32>, Vec<Vec<i32>>> = HashMap::new();

    for h in hvec {
        for (k, v) in h.iter() {
            match map_combined.entry(k.to_vec()) {
                Entry::Vacant(e) => { e.insert(vec![v.to_vec()]);}
                Entry::Occupied(mut e) => { e.get_mut().push(v.to_vec());}
            }
        }
    }

    map_combined

}
