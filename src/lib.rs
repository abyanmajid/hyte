pub mod utils;
pub mod chisquare;
pub mod z;
pub mod t;

pub fn help() {
    let version = env!("CARGO_PKG_VERSION");
    println!(r#"
 _             _        
| |__   _   _ | |_  ___ 
| '_ \ | | | || __|/ _ \
| | | || |_| || |_|  __/
|_| |_| \__, | \__|\___|
        |___/          

❤  Welcome to Hyte v{version} ❤

Documentation: 
Source: https://www.github.com/abyanmajid/hyte

➤  Z-Test for Mean

    ➜  (FN) hyte::z::Z::test
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

    ➜  (FN) hyte::z::Z::test_dataless
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

➤  T-Test for Mean

    ➜  (FN) hyte::t::T::test
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

    ➜  (FN) hyte::t::T::test_dataless
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

➤  Pearson's Chi-squared Test

    ➜  (FN) hyte::chisquare::ChiSquare::test
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