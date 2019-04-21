# Exercise 05: Quine

Write a program that prints its own source code.

Note that `main.rs` cannot be empty because it must contain a definition of the `main` function.

## Run

```
$ ./run.sh
This statement is false.
```

## Test

```
$ ./test.sh
[FAILURE] Output and source code differ:
This statement is false.    | fn main() {
                            >     println!("This statement is false.");
                            > }
```
