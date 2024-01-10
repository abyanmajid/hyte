//! Module responsible for all items needed to perform Z-tests.

use statrs::distribution::Normal;
use statrs::distribution::ContinuousCDF;
use crate::utils::{Tails, Conclusion, mean, variance};

/// A struct for storing the resulting test statistic and p-value from Z-tests.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct ZResult {
    pub test_type: &'static str,
    pub statistic: f64,
    pub p: f64,
}

/// Derives observed mean, sample size, and standard deviation from a `Vec<Number>` data,
/// then calls `z::test_dataless`.
/// 
/// # Examples
/// 
/// The following is an example of how you can perform a lower-tailed 1-sample Z-test.
/// To perform an upper-tailed or 2-sided 1-sample Z-test, simply replace `Tails::LOWER` 
/// with `Tails::UPPER` or `Tails::BOTH` respectively.
/// 
/// ```
/// use hyte::z;
/// use hyte::utils::Tails;
/// 
/// fn main() {
///     let data = vec![1, 2, 3, 4, 5];
///     let unwrapped_results = z::test(data, 3.5, Tails::LOWER, true);
///
///     assert_ne!(unwrapped_results, None);
///  
///     let results = unwrapped_results.unwrap();
///     println!("{:#?}", results);
///
///     assert_eq!(results.test_type, "One-Sided Z-Test for Mean (Lower-Tailed)");
///     assert_eq!(results.statistic, -0.7071067811865475);
///     assert_eq!(results.p, 0.2397500610934768);
/// }
/// ````
/// 
pub fn test<Number: Into<f64> + Copy, Number2: Into<f64> + Copy>(data: Vec<Number>, expected_mean: Number2, tail: Tails, print_output: bool) -> Option<ZResult> {
    if data.len() == 0 { return None };
    let observed_mean = mean(&data).unwrap();
    let sample_size = data.len() as u32;
    let sd = variance(&data).unwrap().sqrt();
    test_dataless(observed_mean, expected_mean.into(), sample_size, sd, tail, print_output)
}

/// Calculates Z-score and p-value, given relevant numerical summaries. 
/// It returns an `Option<ZResult>`, which if successful, can unwrap a
/// `ZResult` with Z-score and p-value as fields `statistic` and `p` respectively.
/// 
/// # Examples
/// 
/// The following is an example of how you can perform a lower-tailed 1-sample Z-test.
/// To perform an upper-tailed or 2-sided 1-sample Z-test, simply replace `Tails::LOWER` 
/// with `Tails::UPPER` or `Tails::BOTH` respectively.
/// 
/// ```
/// use hyte::z;
/// use hyte::utils::Tails;
///
/// fn main() {
///     let unwrapped_results = z::test_dataless(1.2, 1.0, 30, 0.5, Tails::LOWER, true);
/// 
///     assert_ne!(unwrapped_results, None);
///    
///     let results = unwrapped_results.unwrap();
///
///     assert_eq!(results.test_type, "One-Sided Z-Test for Mean (Lower-Tailed)");
///     assert_eq!(results.statistic, 2.1908902300206643);
///     assert_eq!(results.p, 0.9857701315424667);
///
///     println!("{:#?}", results);
/// }
/// ```
pub fn test_dataless<Number: Into<f64> + Copy>(observed_mean: Number, expected_mean: Number, sample_size: u32, pop_sd: Number, tail: Tails, print_output: bool) -> Option<ZResult> {
    if sample_size == 0 { panic!("\n[HYTE-Panic] Sample size must be greater than 0!\n") };
    if pop_sd.into() < 0.0 { panic!("\n[HYTE-Panic] Standard deviation must not be a negative number!\n") };
    
    let statistic: f64 = (observed_mean.into() - expected_mean.into()) / (pop_sd.into() / (sample_size as f64).sqrt());
    let standard_normal = Normal::new(0.0, 1.0).unwrap();
    let p: f64;
    let test_type: &'static str;

    match tail {
        Tails::LOWER => {
            p = standard_normal.cdf(statistic);
            test_type = "One-Sided Z-Test for Mean (Lower-Tailed)";
        },
        Tails::UPPER => {
            p = 1.0 - standard_normal.cdf(statistic);
            test_type = "One-Sided Z-Test for Mean (Upper-Tailed)";
        },
        Tails::BOTH => {
            p = 2.0 * standard_normal.cdf(-statistic.abs());
            test_type = "Two-Sided Z-Test for Mean";
        },
    }

    let results = ZResult {
        test_type,
        statistic,
        p,
    };

    if print_output {
        match tail {
            Tails::LOWER => println!("\n----------------- HYTE -----------------\n\n{}\n\nZ test statistic = {:.2}\np-value = {:.3e}\n\n----------------- HYTE -----------------\n", results.test_type, results.statistic, results.p),
            Tails::UPPER => println!("\n----------------- HYTE -----------------\n\n{}\n\nZ test statistic = {:.2}\np-value = {:.3e}\n\n----------------- HYTE -----------------\n", results.test_type, results.statistic, results.p),
            Tails::BOTH => println!("\n---------- HYTE ----------\n\n{}\n\nZ test statistic = {:.2}\np-value = {:.3e}\n\n---------- HYTE ----------\n", results.test_type, results.statistic, results.p),
        }
    }

    Some(results)
}

impl ZResult {
    /// Concludes if a `ZResult` instance should be rejected given a specified significance value, by returning a `Conclusion` variant `Reject` or `DoNotReject`
    pub fn conclude(&self, significance_level: f64, print_output: bool) -> Conclusion {
        if self.p < significance_level {
            if print_output {println!("\n---------------------------- HYTE ----------------------------\n\nStatistical Conclusion\n\np-value = ({:.3e})\nsignificance level = ({:.3e})\n\np-value < s. l, therefore reject H_0\n\nThere is sufficient evidence to reject the null hypothesis.\n\n---------------------------- HYTE ----------------------------\n", self.p, significance_level);};
            Conclusion::Reject
        } else {
            if print_output {println!("\n---------------------------- HYTE ----------------------------\n\nStatistical Conclusion\n\np-value = ({:.3e})\nsignificance level = ({:.3e})\n\np-value > s. l, therefore do not reject H_0\n\nThere is insufficient evidence to reject the null hypothesis.\n\n---------------------------- HYTE ----------------------------\n", self.p, significance_level);};
            Conclusion::DoNotReject
        }
    }

    /// Concludes if a `ZResult` instance should be rejected using a significance level of 0.05 by convention, by returning a `Conclusion` variant `Reject` or `DoNotReject`
    pub fn conclude_by_convention(&self, print_output: bool) -> Conclusion {
        if self.p < 0.05 {
            if print_output {println!("\n---------------------------- HYTE ----------------------------\n\nStatistical Conclusion\n\np-value = ({:.3e})\nsignificance level = 0.5\n\np-value < s. l, therefore reject H_0\n\nThere is sufficient evidence to reject the null hypothesis.\n\n---------------------------- HYTE ----------------------------\n", self.p);};
            Conclusion::Reject
        } else {
            if print_output {println!("\n---------------------------- HYTE ----------------------------\n\nStatistical Conclusion\n\np-value = ({:.3e})\nsignificance level = 0.5\n\np-value > s. l, therefore do not reject H_0\n\nThere is insufficient evidence to reject the null hypothesis.\n\n---------------------------- HYTE ----------------------------\n", self.p);};
            Conclusion::DoNotReject
        }
    }
}