pub fn is_numeric(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}