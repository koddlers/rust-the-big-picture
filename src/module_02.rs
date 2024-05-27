pub mod discovering_rust {
    pub fn variables_and_mutability() {
        let a = 38;
        let b = 4;
        let mut sum = a;
        sum += b;

        println!("{} + {} = {}", a, b, sum);
    }
}