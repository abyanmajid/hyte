//! # hyte
//! 
//! `hyte` is ***hy***pothesis ***te***sting library intended to make
//! conducting tests easier. In version v0.1.0, the following hypothesis tests are supported:
//! 
//! - 1-sample Z-test
//! - 1-sample T-test
//! - 2-sample T-test
//! - Pearson's Chi-squared test of independence
//! - Pearson's Chi-squared test of goodness of fit
//! 
//! Source: [GitHub](https://www.github.com/abyanmajid/hyte)
//! 
//! v0.1.0 by Abyan Majid (kinderheim511)

pub mod utils;
pub mod chisquare;
pub mod z;
pub mod t;

/// Prints a short manual for all utilities in `hyte` to the standard output.
/// 
/// # Examples
/// 
/// ```
/// use hyte::help;
/// 
/// fn main() {
///     help();
/// }
/// ````
pub fn help() {
    let version = env!("CARGO_PKG_VERSION");
    println!(r#"
 _             _        
| |__   _   _ | |_  ___ 
| '_ \ | | | || __|/ _ \
| | | || |_| || |_|  __/
|_| |_| \__, | \__|\___|
        |___/          

❤  Welcome to Hyte v{version}  ❤

Documentation: 
Source: https://www.github.com/abyanmajid/hyte

➤  Z-Test for Mean

    ➜  (FN) hyte::z::test
            <>
            Intended use: Perform Z-Test given data.
            <>
            args:
                1. data: Vec<Number>, where Number: Into<f64>; accepts integer or float
                2. expected_mean: Number: Into<f64>; accepts integer or float
                3. tail: Tails::LOWER or Tails::UPPER or Tails::BOTH
                4. print_output: bool
            returns: Instance of struct `Z`
                Fields:
                    - test_type: &'static str,
                    - statistic: f64,
                    - p: f64,

    ➜  (FN) hyte::z::test_dataless
            <>
            Intended use: Perform Z-Test given numerical summaries.
            <>
            args:
                1. observed_mean: Number: Into<f64>; accepts integer or float
                2. expected_mean: Number: Into<f64>; accepts integer or float
                3. sample_size: u32
                4. pop_sd: Number: Into<f64>; accepts integer or float
                5. tail: Tails::LOWER or Tails::UPPER or Tails::BOTH
                6. print_output: bool
            returns: Instance of struct `Z`
                Fields:
                    - test_type: &'static str,
                    - statistic: f64,
                    - p: f64,

    ➜  (METHOD) hyte::z::ZResult::conclude
            <>
            Intended use: Conclude if a `ZResult` should be rejected, given a specified s. l
            <>
            args:
                1. significance_level: f64,
                2. print_output: bool,
            returns: A `Conclusion` variant; one of `Reject` or `DoNotReject`

    ➜  (METHOD) hyte::z::ZResult::conclude_by_convention
            <>
            Intended use: Conclude if a `ZResult` should be rejected with s. l = 0.05
            <>
            args:
                1. significance_level: f64,
                2. print_output: bool,
            returns: A `Conclusion` variant; one of `Reject` or `DoNotReject`

➤  T-Test for Mean

    ➜  (FN) hyte::t::test
            <>
            Intended use: Perform T-Test given data.
            <>
            args:
                1. data: data: Vec<Number>, where Number: Into<f64>; accepts integer or float
                2. expected_mean: Number: Into<f64>; accepts integer or float
                3. tail: Tails::LOWER or Tails::UPPER or Tails::BOTH
                4. print_output: bool
            returns: Instance of struct `T`
                Fields:
                    - test_type: &'static str,
                    - statistic: f64,
                    - df: f64,
                    - p: f64,

    ➜  (FN) hyte::t::test_dataless
            <>
            Intended use: Perform T-Test given numerical summaries.
            <>
            args:
                1. observed_mean: Number: Into<f64>; accepts integer or float
                2. expected_mean: Number: Into<f64>; accepts integer or float
                3. sample_size: u32
                4. pop_sd: Number: Into<f64>; accepts integer or float
                5. tail: Tails::LOWER or Tails::UPPER or Tails::BOTH
                6. print_output: bool
            returns: Instance of struct `T`
                Fields:
                    - test_type: &'static str,
                    - statistic: f64,
                    - df: f64,
                    - p: f64,

    ➜  (METHOD) hyte::t::TResult::conclude
            <>
            Intended use: Conclude if a `TResult` should be rejected, given a specified s. l
            <>
            args:
                1. significance_level: f64,
                2. print_output: bool,
            returns: A `Conclusion` variant; one of `Reject` or `DoNotReject`

    ➜  (METHOD) hyte::t::TResult::conclude_by_convention
            <>
            Intended use: Conclude if a `TResult` should be rejected with s. l = 0.05
            <>
            args:
                1. significance_level: f64,
                2. print_output: bool,
            returns: A `Conclusion` variant; one of `Reject` or `DoNotReject`

➤  Pearson's Chi-squared Test

    ➜  (FN) hyte::chisquare::test
            <>
            Intended use: Perform Pearson's Chi-squared Test given data.
            <>
            args:
                1. test_type: &str; Expected: "toi" (Test of Independence) or "gof" (Goodness of Fit)
                2. observed_matrix: Matrix<Number>, where
                    i. Matrix<Number> is an enum with variants:
                        - TwoDimensional(Vec<Vec<Number>>),
                        - OneDimensional(Vec<Number>),
                    ii. Number: Into<f64>; accepts integer or float
                3. gof_probabilities: Option<Vec<f64>>; should be `None` iff test_type == "toi"
                4. print_output: bool
            returns: Instance of struct `ChiSquare`
                Fields:
                    - test_type: &'static str,
                    - statistic: f64,
                    - df: usize,
                    - p: f64,

    ➜  (METHOD) hyte::chisquare::ChiSquareResult::conclude
            <>
            Intended use: Conclude if a `ChiSquareResult` should be rejected, given a specified s. l
            <>
            args:
                1. significance_level: f64,
                2. print_output: bool,
            returns: A `Conclusion` variant; one of `Reject` or `DoNotReject`

    ➜  (METHOD) hyte::chisquare::ChiSquareResult::conclude_by_convention
            <>
            Intended use: Conclude if a `ChiSquareResult` should be rejected with s. l = 0.05
            <>
            args:
                1. significance_level: f64,
                2. print_output: bool,
            returns: A `Conclusion` variant; one of `Reject` or `DoNotReject`

➤  Additional Utilities

    ➜  (ENUM) hyte::utils::Matrix
            variants:
                - TwoDimensional(Vec<Vec<Number>>),
                - OneDimensional(Vec<Number>), 
            where Number: Into<f64>; accepts integer or float

    ➜  (ENUM) hyte::utils::Tails
            variants:
                - LOWER
                - UPPER
                - BOTH

    ➜  (ENUM) hyte::utils::Conclusion
            variants:
                - Reject
                - DoNotReject

    ➜  (FN) hyte::utils::mean
            <>
            Intended use: Calculate the mean of a set of numbers.
            <>
            args:
                1. numbers: &[Number], where
                    i. Number: Into<f64>; accepts integer or float
            returns: Option<f64>

    ➜  (FN) hyte::utils::variance
            <>
            Intended use: Calculate the mean of a set of numbers.
            <>
            args:
                1. numbers: &[Number], where
                    i. Number: Into<f64>; accepts integer or float
            returns: Option<f64>

"#);
}


#[cfg(test)]
mod z_testcases {
    use super::*;
    use crate::utils::{Tails, Conclusion};

    #[test]
    fn test_with_typical_data() {
        let data = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        let expected_mean = 4.0;
        let result = z::test(data, expected_mean, Tails::BOTH, false).unwrap();
        assert!(result.statistic >= -2.0 && result.statistic <= 2.0);
        assert!(result.p >= 0.0 && result.p <= 1.0);
    }

    #[test]
    fn test_with_empty_data() {
        let data: Vec<f64> = Vec::new();
        let expected_mean = 0.0;
        let result = z::test(data, expected_mean, Tails::BOTH, false);
        assert_eq!(result, None);
    }

    #[test]
    fn test_with_zero_variance() {
        let data = vec![3.0, 3.0, 3.0, 3.0, 3.0];
        let expected_mean = 3.0;
        let result = z::test(data, expected_mean, Tails::BOTH, false).unwrap();
        assert!(result.statistic.is_nan());
        assert!(result.p.is_nan());
    }

    #[test]
    fn test_dataless_normal_case() {
        let result = z::test_dataless(5.0, 4.5, 30, 1.0, Tails::UPPER, false).unwrap();
        assert_eq!((result.statistic * 100.0).round() / 100.0, 2.74);
        assert_eq!((result.p * 100.0).round(), 0.0);
    }

    #[test]
    #[should_panic]
    fn test_dataless_invalid_sample_size() {
        z::test_dataless(5.0, 4.5, 0, 1.0, Tails::UPPER, false).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_dataless_negative_standard_deviation() {
        z::test_dataless(5.0, 4.5, 30, -1.0, Tails::UPPER, false).unwrap();
    }

    #[test]
    fn conclude_with_reject() {
        let z_result = z::ZResult { test_type: "Test", statistic: 2.5, p: 0.01 };
        let conclusion = z_result.conclude(0.05, false);
        assert_eq!(conclusion, Conclusion::Reject);
    }

    #[test]
    fn conclude_with_do_not_reject() {
        let z_result = z::ZResult { test_type: "Test", statistic: 1.5, p: 0.10 };
        let conclusion = z_result.conclude(0.05, false);
        assert_eq!(conclusion, Conclusion::DoNotReject);
    }

    #[test]
    fn conclude_by_convention_with_reject() {
        let z_result = z::ZResult { test_type: "Test", statistic: 2.5, p: 0.01 };
        let conclusion = z_result.conclude_by_convention(false);
        assert_eq!(conclusion, Conclusion::Reject);
    }

    #[test]
    fn conclude_by_convention_with_do_not_reject() {
        let z_result = z::ZResult { test_type: "Test", statistic: 1.5, p: 0.10 };
        let conclusion = z_result.conclude_by_convention(false);
        assert_eq!(conclusion, Conclusion::DoNotReject);
    }
}

#[cfg(test)]
mod t_testcases {
    use super::*;
    use crate::utils::{Tails, Conclusion};

    // Test Cases for `test` function
    #[test]
    fn test_typical_case() {
        let data = vec![2.5, 2.9, 3.1, 2.6, 2.7, 2.8, 3.0, 3.2];
        let result = t::test(data, 3.0, Tails::LOWER, false);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.test_type, "(1-Sample) One-Sided T-Test for Mean (Lower-Tailed)");
    }

    #[test]
    fn test_empty_data() {
        let data = Vec::<f64>::new();
        let result = t::test(data, 3.0, Tails::LOWER, false);
        assert!(result.is_none());
    }

    // Test Cases for `test_dataless` function
    #[test]
    fn test_dataless_typical_case() {
        let result = t::test_dataless(2.5, 3.0, 30, 0.5, Tails::UPPER, false);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.test_type, "(1-Sample) One-Sided T-Test for Mean (Upper-Tailed)");
    }

    #[test]
    #[should_panic(expected = "[HYTE-Panic] Sample size must be greater than 0!")]
    fn test_dataless_zero_sample_size() {
        t::test_dataless(2.5, 3.0, 0, 0.5, Tails::UPPER, false);
    }

    #[test]
    #[should_panic(expected = "[HYTE-Panic] Standard deviation must not be a negative number!")]
    fn test_dataless_negative_standard_deviation() {
        t::test_dataless(2.5, 3.0, 30, -0.5, Tails::UPPER, false);
    }

    // Test Cases for `test_two_samples` function
    #[test]
    fn test_two_samples_typical_case() {
        let group1 = vec![20.0, 21.0, 22.0];
        let group2 = vec![23.0, 24.0, 25.0];
        let result = t::test_two_samples(group1, group2, false);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.test_type, "(2-Sample) T-Test for Mean");
    }

    #[test]
    fn test_two_samples_empty_group() {
        let group1 = vec![];
        let group2 = vec![23.0, 24.0, 25.0];
        let result = t::test_two_samples(group1, group2, false);
        assert!(result.is_none());
    }

    // Test Cases for `TResult` methods
    #[test]
    fn conclude_reject() {
        let result = t::TResult { test_type: "Test", statistic: 2.5, df: 29.0, p: 0.01 };
        assert_eq!(result.conclude(0.05, false), Conclusion::Reject);
    }

    #[test]
    fn conclude_do_not_reject() {
        let result = t::TResult { test_type: "Test", statistic: 1.5, df: 29.0, p: 0.10 };
        assert_eq!(result.conclude(0.05, false), Conclusion::DoNotReject);
    }

    #[test]
    fn conclude_by_convention_reject() {
        let result = t::TResult { test_type: "Test", statistic: 2.5, df: 29.0, p: 0.01 };
        assert_eq!(result.conclude_by_convention(false), Conclusion::Reject);
    }

    #[test]
    fn conclude_by_convention_do_not_reject() {
        let result = t::TResult { test_type: "Test", statistic: 1.5, df: 29.0, p: 0.10 };
        assert_eq!(result.conclude_by_convention(false), Conclusion::DoNotReject);
    }
}

#[cfg(test)]
mod chisquare_testcases {
    use super::*;
    use crate::utils::{Matrix, Conclusion};

    // Test Cases for `test` function
    #[test]
    fn test_toi_typical_case() {
        let matrix = Matrix::TwoDimensional(vec![vec![10, 20], vec![20, 10]]);
        let result = chisquare::test("toi", matrix, None, false);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.test_type, "Pearson's Chi-squared Test of Independence");
    }

    #[test]
    fn test_gof_typical_case() {
        let matrix = Matrix::OneDimensional(vec![10, 20, 30]);
        let gof_probabilities = Some(vec![0.2, 0.3, 0.5]);
        let result = chisquare::test("gof", matrix, gof_probabilities, false);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.test_type, "Pearson's Chi-squared Goodness Of Fit");
    }

    #[test]
    #[should_panic(expected = "[HYTE-Panic] You must not pass in an empty matrix!")]
    fn test_empty_matrix() {
        let matrix: Matrix<f64> = utils::Matrix::TwoDimensional(vec![]);
        chisquare::test("toi", matrix, None, false);
    }

    #[test]
    #[should_panic(expected = "[HYTE-Panic] You must not pass in a matrix with a negative number!")]
    fn test_negative_number_in_matrix() {
        let matrix = Matrix::TwoDimensional(vec![vec![-1, 20], vec![20, 10]]);
        chisquare::test("toi", matrix, None, false);
    }

    #[test]
    #[should_panic(expected = "[HYTE-Panic] You must not pass in a matrix with rows of different lengths!")]
    fn test_unequal_row_lengths() {
        let matrix = Matrix::TwoDimensional(vec![vec![10, 20], vec![30]]);
        chisquare::test("toi", matrix, None, false);
    }

    // Test Cases for `ChiSquareResult` methods
    #[test]
    fn conclude_reject() {
        let result = chisquare::ChiSquareResult { test_type: "Test", statistic: 10.5, df: 2, p: 0.01 };
        assert_eq!(result.conclude(0.05, false), Conclusion::Reject);
    }

    #[test]
    fn conclude_do_not_reject() {
        let result = chisquare::ChiSquareResult { test_type: "Test", statistic: 2.5, df: 2, p: 0.10 };
        assert_eq!(result.conclude(0.05, false), Conclusion::DoNotReject);
    }

    #[test]
    fn conclude_by_convention_reject() {
        let result = chisquare::ChiSquareResult { test_type: "Test", statistic: 10.5, df: 2, p: 0.01 };
        assert_eq!(result.conclude_by_convention(false), Conclusion::Reject);
    }

    #[test]
    fn conclude_by_convention_do_not_reject() {
        let result = chisquare::ChiSquareResult { test_type: "Test", statistic: 2.5, df: 2, p: 0.10 };
        assert_eq!(result.conclude_by_convention(false), Conclusion::DoNotReject);
    }
}