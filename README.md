# Rust Data Race Example

This repository demonstrates a subtle data race that can occur in Rust when dealing with mutable and immutable references to the same data.  The `bug.rs` file contains the erroneous code, while `bugSolution.rs` provides a corrected version.

The core issue is that after creating an immutable reference, modifying the underlying data through a mutable reference leads to undefined behavior if the immutable reference is later dereferenced.

This example highlights the importance of careful reference management and understanding Rust's borrowing rules to avoid data races and ensure program correctness.