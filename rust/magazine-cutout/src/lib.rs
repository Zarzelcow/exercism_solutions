// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();
    for word in magazine {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    for word in note {
        if let Some(count) = map.get_mut(word) {
            if *count == 0 {
                return false;
            }
            *count -= 1;
        } else {
            return false;
        }
    }
    return true;
}
