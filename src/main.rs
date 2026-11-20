/// The sum of `a` and `b`.
fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn main() {
    println!("2 + 3 = {}", add(2, 3));
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-4, 1), -3);
    }
}
