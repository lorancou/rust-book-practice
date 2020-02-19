// https://doc.rust-lang.org/book/ch11-03-test-organization.html

// Test Organization

pub fn add_two(a: i32) -> i32 { // Public
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 { // Private
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
