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

    pub fn arrays_ranges_and_looping() {
        let values = [8, 30, 1, 3];
        let mut sum = 0;

        // range based for loop
        // for i in 0..4 {
        //     sum = add(sum, values[i]);
        // }

        // iterator based for loop
        for i in values {
            sum = add(sum, i);
        }

        println!("sum = {}", sum);
    }
}