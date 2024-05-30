pub mod fearless_concurrency {
    fn add(a: i32, b: i32) -> i32 {
        let mut sum = a;
        let (count, increment) = if (b > 0) { (b, 1) } else { (-b, -1) };
        // equivalent to the following
        // let mut count;
        // let mut increment;
        // if (b > 0) {
        //     count = b;
        //     increment = 1
        // } else {
        //     count = -(-b);
        //     increment = -1;
        // }

        for _ in 0..count {
            sum += increment
        }

        sum
    }

    pub fn thread_lifecycle_and_a_concurrent_function_design() {
        let a = 38;
        let b = 4;
        println!("{} + {} = {}", a, b, add(a, b));
    }
}

pub mod fearless_concurrency_v2 {
    use std::thread;

    fn add(a: i32, b: i32) -> i32 {
        let mut sum = a;
        let (count, increment) = if (b > 0) { (b, 1) } else { (-b, -1) };
        let mut handles = vec![];

        for _ in 0..count {
            handles.push(
                thread::spawn(move || {
                    sum += increment;
                })
            );
        }

        for handle in handles {
            handle.join().unwrap();
        }

        sum
    }

    pub fn concurrent_function_implementation_first_attempt() {
        let a = 38;
        let b = 4;
        println!("{} + {} = {}", a, b, add(a, b));
    }
}

pub mod fearless_concurrency_v3 {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::thread;

    fn add(a: i32, b: i32) -> i32 {
        let sum = Arc::new(AtomicI32::new(a));
        let (count, increment) = if (b > 0) { (b, 1) } else { (-b, -1) };
        let mut handles = vec![];

        for _ in 0..count {
            let inner_sum = Arc::clone(&sum);
            handles.push(
                thread::spawn(move || {
                    inner_sum.fetch_add(increment, Ordering::SeqCst);
                })
            );
        }

        for handle in handles {
            handle.join().unwrap();
        }

        sum.load(Ordering::SeqCst)
    }

    pub fn concurrent_function_implementation() {
        let a = 38;
        let b = 4;
        println!("{} + {} = {}", a, b, add(a, b));
    }
}

pub mod fearless_concurrency_v4 {
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::Ordering;
    use std::thread;

    #[derive(Debug)]
    struct Accumulator {
        sum: i32,
        operation_count: i32,
    }

    impl Accumulator {
        fn new(sum: i32) -> Self {
            Self {
                sum,
                operation_count: 0,
            }
        }

        // `&self` is an immutable borrow of the instance
        fn get(&self) -> i32 {
            self.sum
        }
        // `&mut self` is a mutable borrow of the instance
        fn add(&mut self, increment: i32) {
            self.sum += increment;
            self.operation_count += 1;
        }

        fn get_sum(&self) -> i32 {
            self.sum
        }

        fn get_count(&self) -> i32 {
            self.operation_count
        }

        fn combine(acc1: Self, acc2: Self) -> Self {
            Self::new(acc1.sum + acc2.sum)
        }
    }

    fn add(a: i32, b: i32) -> i32 {
        let acc = Arc::new(Mutex::new(Accumulator::new(a)));
        let (count, increment) = if (b > 0) { (b, 1) } else { (-b, -1) };
        let mut handles = vec![];

        for _ in 0..count {
            let inner_acc = Arc::clone(&acc);
            handles.push(
                thread::spawn(move || {
                    let mut guarded_acc = inner_acc.lock().unwrap();
                    guarded_acc.add(increment);
                })
            );
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_acc = acc.lock().unwrap();
        final_acc.get_sum()
    }

    pub fn concurrent_function_implementation() {
        let a = 38;
        let b = 4;
        println!("{} + {} = {}", a, b, add(a, b));
    }
}