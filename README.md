# [Hyte](https://github.com/abyanmajid/hyte) 🦀 &middot; [![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/abyanmajid/hyte/blob/main/LICENSE)

***Hyte*** is a ***Hy***pothesis ***te***sting library crate for Rust with support for Chi-square, Z, and T-tests.

WORK IN PROGRESS

## Installation

Include the following in your `Cargo.toml` file.

```toml
[dependencies]
hyte = "0.1.0"
```

## Quickstart

The following are collapsible contents, each containing snippets to help you get started.

<details>
  <summary>Performing Z-tests</summary>
  <br>

  Work in Progress
  
</details>

<details>
  <summary>Performing T-tests</summary>
  <br>

  Work in Progress

</details>

<details>
  <summary>Performing Pearson's Chi-squared tests</summary>
  <br>

  Work in Progress
  
</details>

<details>
  <summary>Concluding a test</summary>

  <h4>Concluding with a custom significance level using <code>conclude</code></h4>

  Every instance of a test result such as `Z`, `T`, and `ChiSquare` have a method `conclude` which you can call to return a `Conclusion` variant (one of `Reject` or `DoNotReject`). The `conclude` method takes in two parameters:

  - significance_level: `f64`
  - print_output: `bool`
  
  ```rust
  use hyte::z::Z;

  fn main() {
      let results = Z::test(vec![1, 2, 3, 4, 5], 3.5, Tails::LOWER, true);

      // args: significance_level: f64, print_output: bool
      // returns: a `Conclusion` variant - either `Reject` or `DoNotReject`
      let conclusion = results.conclude(0.1, true);
  }
  ```

  `conclude` checks if the p-value assigned to `self.p` exceeds the significance level. If `self.p < significance_level`, then `conclude` will return the `Reject` variant. Otherwise, it will return the `DoNotReject` variant.

  <h4>Concluding conventionally with <code>conclude_by_convention</code></h4>

  `conclude_by_convention` is an alternative to `conclude`. It assumes a significance level of 0.05, which is widely regarded as an appropriate default in statistics.

  ```rust
  use hyte::z::Z;

  fn main() {
      let results = Z::test(vec![1, 2, 3, 4, 5], 3.5, Tails::LOWER, true);

      // args: print_output: bool
      // returns: a `Conclusion` variant - either `Reject` or `DoNotReject`
      let conclusion = results.conclude_by_convention(true);
  }
  ```

</details>

## Getting help

The documentation for this crate can be found at [Work In Progress](https://github.com/abyanmajid/hyte/blob/main/LICENSE). Alternatively, you can print out a short manual by calling the `help` function.

```rust
use hyte::help;

fn main() {
    help();
}
```
