pub mod discovering_rust {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn variables_and_mutability() {
        let a = 38;
        let b = 4;
        let sum = add(a, b);

        println!("{} + {} = {}", a, b, sum);
    }
}