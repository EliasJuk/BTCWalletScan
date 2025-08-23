use std::collections::HashMap;

pub fn get_ranges() -> HashMap<&'static str, (&'static str, &'static str)> {
    let mut map = HashMap::new();
    map.insert("68", ("80000000000000000", "fffffffffffffffff"));
    map.insert("70", ("200000000000000000", "3fffffffffffffffff"));
    map.insert("160", ("8000000000000000000000000000000000000000", "ffffffffffffffffffffffffffffffffffffffff"));
    map
}