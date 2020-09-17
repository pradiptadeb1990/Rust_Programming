# Nth Prime

Given a number n, determine what the nth prime is.

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that
the 6th prime is 13.

If your language provides methods in the standard library to deal with prime
numbers, pretend they don't exist and implement them yourself.

Remember that while people commonly count with 1-based indexing (i.e. "the 6th prime is 13"), many programming languages, including Rust, use 0-based indexing (i.e. `primes[5] == 13`). Use 0-based indexing for your implementation.

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

To run all ignored tests without editing the tests source file, use:

```bash
$ cargo test -- --ignored
```

To run a specific test, for example `some_test`, you can use:

```bash
$ cargo test some_test
```

If the specific test is ignored use:

```bash
$ cargo test some_test -- --ignored
```

## Further improvements

To format the code solution, inside the solution directory use

```bash
cargo fmt
```

To see, if the code solution contains some common ineffective use cases, inside the solution directory use

```bash
cargo clippy --all-targets
```
