# Task

Input: list of integers

Output: ranges

Fold a list of given numbers into ranges when they are consecutive.

```rust

    fn main() {
        let long = [1, 4, 5, 2, 3, 9, 8, 11, 0]; // Result: [(0, 5), (8, 9), (11, 11)]
        let short = [1, 4, 3, 2]; // Result: [(1, 4)]

        range(&long);
        range(&short);
    }

    fn range(input: &[u32]) -> Vec<(u32, u32)> {
        unimplemented!()
    }

```
