pub mod ownership_and_the_borrow_checker {
    #[derive(Debug)]
    struct Accumulator {
        sum: i32,
    }

    impl Accumulator {
        fn new(sum: i32) -> Self {
            Self { sum }
        }

        fn get(self) -> i32 {
            self.sum
        }

        fn add(self, increment: i32) -> Self {
            Self {
                sum: self.sum + increment
            }
        }
    }

    pub fn memory_management_models() {
        let mut acc = Accumulator::new(0);

        for i in 3..10 {
            acc = acc.add(i);
            // uncomment the following line to see the error
            // println!("acc = {}", acc.get());
        }

        println!("acc = {}", acc.get());
    }
}