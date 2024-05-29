pub mod ownership_and_the_borrow_checker {
    #[derive(Debug)]
    struct Accumulator {
        sum: i32,
    }

    impl Accumulator {
        fn new(sum: i32) -> Self {
            Self { sum }
        }

        // `&self` is an immutable borrow of the instance
        fn get(&self) -> i32 {
            self.sum
        }
        // `&mut self` is a mutable borrow of the instance
        fn add(&mut self, increment: i32) {
            self.sum += increment
        }

        fn combine(acc1: Self, acc2: Self) -> Self {
            Self::new(acc1.sum + acc2.sum)
        }
    }

    pub fn memory_management_models() {
        let mut acc = Accumulator::new(0);

        for i in 3..10 {
            acc.add(i);
            println!("acc = {}", acc.get());
        }

        println!("acc = {}", acc.get());
    }

    pub fn ownership_borrowing_and_moving_values() {
        let mut evens_acc = Accumulator::new(0);
        let mut odds_acc = Accumulator::new(0);

        for i in 3..10 {
            if i % 2 == 0 {
                evens_acc.add(i);
            } else {
                odds_acc.add(i);
            }
        }

        let acc = Accumulator::combine(evens_acc, odds_acc);
        println!("acc = {}", acc.get());
    }
}