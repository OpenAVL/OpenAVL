pub fn sey_hello() {
    println!("Hello from OpenAVL!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        sey_hello();
    }
}
