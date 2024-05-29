#![allow(unused)]

mod module_02;
mod module_03;
mod module_04;

use module_02::discovering_rust;
use module_03::functional_flavored_oop;
use module_04::ownership_and_the_borrow_checker;

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
    ownership_and_the_borrow_checker::memory_management_models();
}
