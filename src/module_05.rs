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