# Property-Based Randomized Testing

problem: check the correctness of a complex function

extreme solutions:
    - "classical" testing: hand-picked input/output pairs
        - labor-intensive
        - incomplete
        - not fun
    - formal methods:
        - steep learning curve (proofs, abstract math)
        - lack of support & simple tools
        - did I mention proofs & math?

middle ground: property-based randomized testing
    - requires some math and creativity
    - is a probabilistic method
    - has good support, automation
    - scales well

details:
- property: a true statement about an input-output pair
- test: generate random input -> run function -> check properties
- repeat MANY times, essentially sampling the input domain
- main difficulty: find good properties, generate random values of a specific kind

notable implementations:
- QuickCheck (Haskell)
- proptest (Rust)
- hypothesis (Python)

examples:
- fn is_palindrome(input: String) -> bool
- fn add_int(a: i32, b: i32) -> i32
