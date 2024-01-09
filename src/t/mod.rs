//! Module responsible for all items needed to perform T-tests.

use statrs::distribution::StudentsT;
use statrs::distribution::ContinuousCDF;
use crate::utils::{Tails, Conclusion, mean, variance}; 

/// A struct for storing the resulting test statistic and p-value from T-tests.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct TResult {
    pub test_type: &'static str,
    pub statistic: f64,
    pub df: f64,
    pub p: f64,
}

/// Derives observed mean, sample size, and standard deviation from a `Vec<Number>` data,
/// then calls `t::test_dataless`.
/// 
/// # Examples
/// 
/// The following is an example of how you can perform a lower-tailed 1-sample T-test.
/// To perform an upper-tailed or 2-sided 1-sample T-test, simply replace `Tails::LOWER` 
/// with `Tails::UPPER` or `Tails::BOTH` respectively.
/// 
/// ```
/// use hyte::t;
/// use hyte::utils::Tails;
///
/// fn main() {
///     let data = vec![2.5, 2.9, 3.1, 2.6, 2.7, 2.8, 3.0, 3.2];
///     let unwrapped_results = t::test(data, 3, Tails::LOWER, true);
///
///     assert_ne!(unwrapped_results, None);
///
///     let results = unwrapped_results.unwrap();
///     println!("{:#?}", results);
///
///     assert_eq!(results.test_type, "(1-Sample) One-Sided T-Test for Mean (Lower-Tailed)");
///     assert_eq!(results.statistic, -1.7320508075688763);
///     assert_eq!(results.df, 7.0);
///     assert_eq!(results.p, 0.06343518346183559);
/// }
/// ```
pub fn test<Number: Into<f64> + Copy, Number2: Into<f64> + Copy>(data: Vec<Number>, expected_mean: Number2, tail: Tails, print_output: bool) -> Option<TResult> {
    if data.len() == 0 { return None };
    let observed_mean = mean(&data).unwrap();
    let sample_size = data.len() as u32;
    let sd = variance(&data).unwrap().sqrt();
    test_dataless(observed_mean, expected_mean.into(), sample_size, sd, tail, print_output)
}

/// Calculates T-score and p-value, given relevant numerical summaries. 
/// It returns a `TResult` instance with T-score and p-value as fields `statistic` and `p` respectively.
/// 
/// # Examples
/// 
/// The following is an example of how you can perform a lower-tailed 1-sample T-test.
/// To perform an upper-tailed or 2-sided 1-sample T-test, simply replace `Tails::LOWER` 
/// with `Tails::UPPER` or `Tails::BOTH` respectively.
/// 
/// ```
/// use hyte::t;
/// use hyte::utils::Tails;
///
/// fn main() {
///     let unwrapped_results = t::test_dataless(1.2, 1.0, 30, 0.5, Tails::LOWER, true);
///
///     assert_ne!(unwrapped_results, None);
///
///     let results = unwrapped_results.unwrap();
///     println!("{:#?}", results);
///
///     assert_eq!(results.test_type, "(1-Sample) One-Sided T-Test for Mean (Lower-Tailed)");
///     assert_eq!(results.statistic, 2.1908902300206643);
///     assert_eq!(results.df, 29.0);
///     assert_eq!(results.p, 0.9816776580257508);
/// }
/// ```
pub fn test_dataless<Number: Into<f64> + Copy>(observed_mean: Number, expected_mean: Number, sample_size: u32, pop_sd: Number, tail: Tails, print_output: bool) -> Option<TResult> {
    if sample_size == 0 { panic!("\n[HYTE-Panic] Sample size must be greater than 0!\n") };
    if pop_sd.into() < 0.0 { panic!("\n[HYTE-Panic] Standard deviation must not be a negative number!\n") };
    
    let statistic: f64 = (observed_mean.into() - expected_mean.into()) / (pop_sd.into() / (sample_size as f64).sqrt());
    let df = (sample_size - 1) as f64;
    let t_distribution = StudentsT::new(0.0, 1.0, df).unwrap();
    let p: f64;
    let test_type: &'static str;

    match tail {
        Tails::LOWER => {
            p = t_distribution.cdf(statistic);
            test_type = "(1-Sample) One-Sided T-Test for Mean (Lower-Tailed)";
        },
        Tails::UPPER => {
            p = 1.0 - t_distribution.cdf(statistic);
            test_type = "(1-Sample) One-Sided T-Test for Mean (Upper-Tailed)";
        },
        Tails::BOTH => {
            p = 2.0 * t_distribution.cdf(-statistic.abs());
            test_type = "(1-Sample) Two-Sided T-Test for Mean"
        },
    }

    let results = TResult {
        test_type,
        statistic,
        df,
        p,
    };

    if print_output {
        match tail {
            Tails::LOWER | Tails::UPPER => println!("\n----------------------- HYTE -----------------------\n\n{}\n\nZ test statistic = {:.2}\np-value = {:.3e}\nDegrees of freedom = {}\n\n----------------------- HYTE -----------------------\n", results.test_type, results.statistic, results.p, results.df),
            Tails::BOTH => println!("\n--------------- HYTE ---------------\n\n{}\n\nZ test statistic = {:.2}\np-value = {:.3e}\nDegrees of freedom = {}\n\n--------------- HYTE ---------------\n", results.test_type, results.statistic, results.p, results.df),
        }
    }

    Some(results)
}

/// Calculates T-score and p-value for a 2-sample T-test, given two groups of data each of type Vec<Number>. 
/// It returns a `TResult` instance with T-score and p-value as fields `statistic` and `p` respectively.
/// 
/// # Examples
/// 
/// The following is an example of how you can perform a 2-sample T-test.
/// 
/// ```
/// use hyte::t;
///
/// fn main() {
///     let group1 = vec![20, 22, 19, 20, 21, 20, 19, 21, 22, 18];
///     let group2 = vec![22, 24, 23, 24, 25, 23, 24, 23, 22, 24];
///     let unwrapped_results = t::test_two_samples(group1, group2, true);
///
///     assert_ne!(unwrapped_results, None);
///
///     let results = unwrapped_results.unwrap();
///     println!("{:#?}", results);
///
///     assert_eq!(results.test_type, "(2-Sample) T-Test for Mean");
///     assert_eq!(results.statistic, -6.196773353931866);
///     assert_eq!(results.df, 16.513761467889907);
///     assert_eq!(results.p, 0.000011111614734799414);
/// }
/// ```
pub fn test_two_samples<Number: Into<f64> + Copy>(data1: Vec<Number>, data2: Vec<Number>, print_output: bool) -> Option<TResult> {
    if data1.len() == 0 {
        return None
    } else if data2.len() == 0 {
        return None
    };
    let data1_mean = mean(&data1).unwrap();
    let data2_mean = mean(&data2).unwrap();
    let data1_variance = variance(&data1).unwrap();
    let data2_variance = variance(&data2).unwrap();
    let n1 = data1.len() as f64;
    let n2 = data2.len() as f64;

    let numerator = ((data1_variance / n1) + (data2_variance / n2)).powi(2);
    let denominator = (data1_variance.powi(2) / (n1 * n1 * (n1 - 1.0))) + (data2_variance.powi(2) / (n2 * n2 * (n2 - 1.0)));
    let df = numerator / denominator;
    
    let statistic = (data1_mean - data2_mean) / ((data1_variance / n1) + (data2_variance / n2)).sqrt();

    let t_distribution = StudentsT::new(0.0, 1.0, df as f64).unwrap();
    let p = 2.0 * t_distribution.cdf(-statistic.abs());   
    
    let results = TResult {
        test_type: "(2-Sample) T-Test for Mean",
        statistic,
        df,
        p,
    };

    if print_output {println!("\n---------- HYTE ----------\n\n{}\n\nZ test statistic = {:.2}\np-value = {:.3e}\nDegrees of freedom = {:.2}\n\n---------- HYTE ----------\n", results.test_type, results.statistic, results.p, results.df)};

    Some(results)
}

impl TResult {
    /// Concludes if a `TResult` instance should be rejected given a specified significance value, by returning a `Conclusion` variant `Reject` or `DoNotReject`
    pub fn conclude(&self, significance_level: f64, print_output: bool) -> Conclusion {
        if self.p < significance_level {
            if print_output {println!("\n---------------------------- HYTE ----------------------------\n\nStatistical Conclusion\n\np-value = ({:.3e})\nsignificance level = ({:.3e})\n\np-value < s. l, therefore reject H_0\n\nThere is sufficient evidence to reject the null hypothesis.\n\n---------------------------- HYTE ----------------------------\n", self.p, significance_level);};
            Conclusion::Reject
        } else {
            if print_output {println!("\n---------------------------- HYTE ----------------------------\n\nStatistical Conclusion\n\np-value = ({:.3e})\nsignificance level = ({:.3e})\n\np-value > s. l, therefore do not reject H_0\n\nThere is insufficient evidence to reject the null hypothesis.\n\n---------------------------- HYTE ----------------------------\n", self.p, significance_level);};
            Conclusion::DoNotReject
        }
    }
    /// Concludes if a `TResult` instance should be rejected using a significance level of 0.05 by convention, by returning a `Conclusion` variant `Reject` or `DoNotReject`
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