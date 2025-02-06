# Rust Mutable and Immutable Reference Error

This repository demonstrates a common error in Rust related to mutable and immutable references.  The core issue is attempting to modify a value through an immutable reference while a mutable reference to the same value already exists.  This is prevented by the Rust compiler to ensure data safety and prevent race conditions.

The `bug.rs` file contains code that will trigger a compile-time error, highlighting the problem. The `bugSolution.rs` file offers a solution by managing references to avoid conflicts.