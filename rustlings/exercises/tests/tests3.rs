// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// Execute `rustlings hint tests3` for hints :)


pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("Hello! Carol")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was {}",result);
    }
}

