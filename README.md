# [Hyte](https://github.com/abyanmajid/hyte) 🦀 &middot; [![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/abyanmajid/hyte/blob/main/LICENSE)

***Hyte*** is a ***Hy***pothesis ***te***sting library crate for Rust with support for Chi-square, Z, and T-tests.

[Documentation](https://docs.rs/hyte/0.1.0/hyte/) | [Source](https://github.com/abyanmajid/hyte/) | [crates.io](https://crates.io/crates/hyte)

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

  <h3>Performing a 1-sample Z-test</h3>

  You can use `z::test`, a function that takes the following arguments:

  - data: `Vec<Number>`
  - expected_mean: `Number`
  - tail: `Tails::LOWER`, `Tails::UPPER`, or `Tails::BOTH`
  - print_output: `bool`

  where `Number` is a generic that accepts integers and floats. Here is an example of a how you can perform a lower-tailed 1-sample Z-test:

  ```rust
  use hyte::z;
  use hyte::utils::Tails;
   
  fn main() {
      let data = vec![1, 2, 3, 4, 5];
      let results = z::test(data, 3.5, Tails::LOWER, true).unwrap();
  }
  ```

  Should you need to perform upper-tailed or 2-sided Z-tests, simply pass the `Tails::UPPER` or `Tails::BOTH` variants to `tail`.

  <h3>Performing a 1-sample Z-test given numerical summaries</h3>

  You can alternatively perform Z-tests using the `z::test_dataless` function which takes in numerical summaries including observed mean, sample size, and population standard deviation, all in replacement of data. The `z::test_dataless` function takes the following arguments:

  - observed_mean: `Number`
  - expected_mean: `Number`
  - sample_size: `u32`
  - pop_sd: `Number`
  - tail: `Tails::LOWER`, `Tails::UPPER`, or `Tails::BOTH`
  - print_output: `bool`

  Here is an example:
  
  ```rust
  use hyte::z;
  use hyte::utils::Tails;
  
  fn main() {
      let results = z::test_dataless(1.2, 1.0, 30, 0.5, Tails::LOWER, true).unwrap();
  }
  ```
  
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

  <h3>Concluding with a custom significance level using <code>conclude</code></h3>

  Every instance of a test result such as `ZResult`, `TResult`, and `ChiSquareResult` have a method `conclude` which returns a `Conclusion` variant (one of `Reject` or `DoNotReject`). The `conclude` method takes in two parameters:

  - significance_level: `f64`
  - print_output: `bool`
  
  ```rust
  use hyte::z;
  use hyte::utils::Tails;

  fn main() {
      let results = z::test(vec![1, 2, 3, 4, 5], 3.5, Tails::LOWER, true).unwrap();
      let conclusion = results.conclude(0.1, true);
  }
  ```

  `conclude` checks if the p-value assigned to `self.p` exceeds the significance level. If `self.p < significance_level`, then `conclude` will return the `Reject` variant. Otherwise, it will return the `DoNotReject` variant.

  <h3>Concluding conventionally with <code>conclude_by_convention</code></h3>

  `conclude_by_convention` is an alternative to `conclude`. It assumes a significance level of 0.05, which is widely regarded as an appropriate default in statistics.

  ```rust
  use hyte::z;
  use hyte::utils::Tails;

  fn main() {
      let results = z::test(vec![1, 2, 3, 4, 5], 3.5, Tails::LOWER, true).unwrap();
      let conclusion = results.conclude_by_convention(true);
  }
  ```

</details>

## Getting help

The documentation for this crate can be found at [Work In Progress](https://github.com/abyanmajid/hyte/blob/main/LICENSE). Alternatively, you can print a short manual to the standard output by calling the `help` function.

```rust
use hyte::help;

fn main() {
    help();
}
```
