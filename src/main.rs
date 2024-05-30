#![allow(unused)]

mod module_02;
mod module_03;
mod module_04;
mod module_05;

use module_02::discovering_rust;
use module_03::functional_flavored_oop;
use module_04::ownership_and_the_borrow_checker;
use module_05::fearless_concurrency;
use module_05::fearless_concurrency_v2;

fn main() {
    // Module 02 - Discovering Rust
    // discovering_rust::variables_and_mutability();
    // discovering_rust::arrays_ranges_and_looping();
    // discovering_rust::array_slices_and_zero_cost_abstractions();
    // discovering_rust::vectors();

    // Module 03 - Functional Flavored Object-Oriented Programming
    // functional_flavored_oop::closures();
    // functional_flavored_oop::capturing_closures();
    // functional_flavored_oop::iterators();
    // functional_flavored_oop::iterator_chaining_and_lazy_execution();
    // functional_flavored_oop::standard_iterators();
    // functional_flavored_oop::structure_definition_and_initialization();

    // Module 04 - Ownership and the Borrow Checker
    // ownership_and_the_borrow_checker::memory_management_models();
    // ownership_and_the_borrow_checker::ownership_borrowing_and_moving_values();

    // Module 05 - Fearless Concurrency
    // fearless_concurrency::thread_lifecycle_and_a_concurrent_function_design();
    fearless_concurrency_v2::concurrent_function_implementation_first_attempt();
}
