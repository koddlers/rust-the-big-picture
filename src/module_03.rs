pub mod functional_flavored_oop {
    pub fn closures() {
        let add = |a, b| a + b;

        let sum_range = |from, to| {
            let mut sum = 0;

            for i in from..to {
                // this closure captures the `add` variable from the
                // enclosing scope, which is another closure
                sum = add(sum, i);
            }

            // return value of this closure
            sum
        };

        println!("sum = {}", sum_range(3, 10));
    }

    pub fn capturing_closures() {
        let add = |a, b| a + b;

        // tuple destructuring
        let (from, to) = (3, 10);
        let sum_range = || {
            let mut sum = 0;

            // the variables `from` and `to` are also captured from the enclosing scope
            for i in from..to {
                // this closure captures the `add` variable from the
                // enclosing scope, which is another closure
                sum = add(sum, i);
            }

            // return value of this closure
            sum
        };

        println!("sum = {}", sum_range());
    }

    pub fn iterators() {
        let mut sum = 0;
        let add = |a, b| a + b;
        let numbers = 3..10;

        for i in numbers.into_iter() {
            sum = add(sum, i);
        }

        println!("sum = {}", sum);
    }

    pub fn iterator_chaining_and_lazy_execution() {
        let mut sum = 0;
        let add = |a, b| a + b;
        let numbers = (1..13)
            .inspect(|n| println!("Before filtering, n = {}", n))
            .filter(|n| n % 2 == 0)
            .inspect(|n| println!("After filtering, n = {}", n));

        for i in numbers {
            sum = add(sum, i);
        }

        println!("sum = {}", sum);
    }

    pub fn standard_iterators() {
        // version - 1: using `fold()`
        // let sum = (1..13)
        //     .filter(|n| n % 2 == 0)
        //     .fold(0, |tally, n| tally + n);

        // to visualize what really happens in closure inside the `fold()`
        // let sum = (1..13)
        //     .filter(|n| n % 2 == 0)
        //     .fold(0, |tally, n| {
        //         println!("tally = {}, n = {}", tally, n);
        //         tally + n
        //     });

        // version - 2: using `sum()::<T>`
        // let sum = (1..13)
        //     .filter(|n| n % 2 == 0)
        //     .sum::<i32>();

        // version - 3: using `sum()`
        let sum: i32 = (1..13)
            .filter(|n| n % 2 == 0)
            .sum();

        println!("sum = {}", sum);
    }
}