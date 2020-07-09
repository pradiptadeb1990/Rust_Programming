# Raindrops

The task is to convert a number into a string that contains raindrop sounds corresponding to certain potential factors. 
A factor is a number that evenly divides into another number, leaving no remainder. 
The simplest way to test if a one number is a factor of another is to use the [modulo operation](https://en.wikipedia.org/wiki/Modulo_operation).

The rules of `raindrops` are that if a given number:

- has 3 as a factor, add 'Pling' to the result.
- has 5 as a factor, add 'Plang' to the result.
- has 7 as a factor, add 'Plong' to the result.
- _does not_ have any of 3, 5, or 7 as a factor, the result should be the digits of the number.

## Examples

- 28 has 7 as a factor, but not 3 or 5, so the result would be "Plong".
- 30 has both 3 and 5 as factors, but not 7, so the result would be "PlingPlang".
- 34 is not factored by 3, 5, or 7, so the result would be "34".

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