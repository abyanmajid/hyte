# [Hyte](https://github.com/abyanmajid/hyte) 🦀

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/abyanmajid/hyte/blob/main/LICENSE) [![Version](https://img.shields.io/badge/crates.io-v0.1.1-orange.svg)](https://crates.io/crates/hyte) ![example workflow](https://github.com/abyanmajid/hyte/actions/workflows/ci.yml/badge.svg)

***Hyte*** is a ***Hy***pothesis ***te***sting library crate for Rust with support for Z, T, and Pearson's Chi-squared tests.

[Documentation](https://docs.rs/hyte/0.1.1/hyte/) 📃 | [crates.io](https://crates.io/crates/hyte) 📦 | [Source](https://github.com/abyanmajid/hyte/) 🌿

## Installation

Run the following to include `hyte` as a dependency.

```toml
cargo add hyte
```

## Quickstart

The following are collapsible contents, each containing snippets to help you get started.

<details>
  <summary>Performing Z-tests</summary>

  <h3>1-sample Z-test</h3>

  You can perform a 1-sample Z-test using `z::test`, a function that takes in the following arguments:

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

  <h3>1-sample Z-test given numerical summaries</h3>

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

  <h3>1-sample T-test</h3>

  You can perform a 1-sample T-test using `t::test`, a function that takes in the following arguments:

  - data: `Vec<Number>`
  - expected_mean: `Number`
  - tail: `Tails::LOWER`, `Tails::UPPER`, or `Tails::BOTH`
  - print_output: `bool`

  where `Number` is a generic that accepts integers and floats. Here is an example of a how you can perform a lower-tailed 1-sample T-test:

  ```rust
  use hyte::t;
  use hyte::utils::Tails;
  
  fn main() {
      let data = vec![2.5, 2.9, 3.1, 2.6, 2.7, 2.8, 3.0, 3.2];
      let results = t::test(data, 3, Tails::LOWER, true).unwrap();
  }
  ```

  <h3>1-sample T-test given numerical summaries</h3>

  You can alternatively perform T-tests using the `t::test_dataless` function which takes in numerical summaries including observed mean, sample size, and population standard deviation, all in replacement of data. The `t::test_dataless` function takes the following arguments:

  - observed_mean: `Number`
  - expected_mean: `Number`
  - sample_size: `u32`
  - pop_sd: `Number`
  - tail: `Tails::LOWER`, `Tails::UPPER`, or `Tails::BOTH`
  - print_output: `bool`

  Here is an example:
  
  ```rust
  use hyte::t;
  use hyte::utils::Tails;
  
  fn main() {
      let results = t::test_dataless(1.2, 1.0, 30, 0.5, Tails::LOWER, true).unwrap();
  }
  ```

  <h3>2-sample T-test</h3>

  Hyte provides the `t::test_two_samples` function for performing a 2-sample T-test. It takes in the following arguments:

  - data1: `Vec<Number>`
  - data2: `Vec<Number>`
  - print_output: `bool`

  Here's an example:

  ```rust
  use hyte::t;

  fn main() {
      let group1 = vec![20, 22, 19, 20, 21, 20, 19, 21, 22, 18];
      let group2 = vec![22, 24, 23, 24, 25, 23, 24, 23, 22, 24];
      let results = t::test_two_samples(group1, group2, true).unwrap();
  }
  ```

</details>

<details>
  <summary>Performing Pearson's Chi-squared tests</summary>
  <br>
  
  The `chisquare` module only contains one funtion `chisquare::test` which can be used to perform both Pearson's Chi-squared test of independence and goodness of fit. It takes on the following arguments:

  - test_type: `&str`
  - observed_matrix: `Matrix<Number>`
  - gof_probabilities: `Option<Vec<f64>>`
  - print_output: `bool`

  where `Matrix<Number>` is an enum with two variants: `Matrix::TwoDimensional(Vec<Vec<Number>>)` and `Matrix::OneDimensional(Vec<Number>)`.

  <h3>Test of independence</h3>

  To perform a test of independence, you must pass in:
  
  - `"toi"` to `test_type`
  - `Option::None` variant to `gof_probabilities`
  - `Matrix::TwoDimensional(Vec<Vec<Number>>)` to `observed_matrix`

  Here's an example:
  ```rust
  use hyte::chisquare;
  use hyte::utils::Matrix;
  
  fn main() {
      let observed_frequencies = Matrix::TwoDimensional(vec![vec![762, 327, 468], 
                                                             vec![484, 239, 477]]);
      let results = chisquare::test(
          "toi", 
          observed_frequencies, 
          None, 
          true
      ).unwrap();
  }
  ```

  <h3>Goodness Of Fit</h3>

  To perform a goodness of fit test, you must pass in:
  
  - `"gof"` to `test_type`
  - `Option::Some(f64)` variant to `gof_probabilities`
  - `Matrix::OneDimensional(Vec<Number>)` to `observed_matrix`

  Here's an example:
  
  ```rust
  use hyte::chisquare;
  use hyte::utils::Matrix;
  
  fn main() {
      let results = chisquare::test(
          "gof",
          Matrix::OneDimensional(vec![30, 40, 30]),
          Some(vec![0.25, 0.5, 0.25]),
          true
      ).unwrap();
  }
  ```
  
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

The documentation for this crate can be found at [docs.rs/hyte](https://docs.rs/hyte). Alternatively, you can print a short manual to the standard output by calling the `help` function.

```rust
use hyte::help;

fn main() {
    help();
}
```
