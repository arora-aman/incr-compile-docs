#[inline]
pub fn strcat(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

pub fn total_strlen(strs: Vec<&str>) -> usize {
    let mut total = 0;

    for s in strs {
        total += s.len();
    }

    total
}
