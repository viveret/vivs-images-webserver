pub fn string_hashcode_java_style(path: &str) -> i32 {
    if path.is_empty() {
        return 0;
    }

    let mut hash: i32 = 0;

    for ch in path.chars() {
        // Java uses 31 as multiplier and allows integer overflow
        hash = hash.wrapping_mul(31).wrapping_add(ch as i32);
    }

    hash
}