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
}