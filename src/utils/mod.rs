//! Module for data structures and additional utilities which the `z`, `t`, and `chisquare` modules depend on.

/// An enum for representing multidimensional vectors
pub enum Matrix<Number> {
    TwoDimensional(Vec<Vec<Number>>),
    OneDimensional(Vec<Number>),
}

/// An enum for specifying if a 1-sample test is lower-tailed, upper-tailed, or 2-sided.
pub enum Tails {
    LOWER,
    UPPER,
    BOTH,
}

/// An enum for concluding if a test result should be rejected.
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Conclusion {
    Reject,
    DoNotReject,
}

/// Computes the average of a vector/array of numbers `$[Number]`
/// 
/// # Examples
/// ```
/// use hyte::utils::mean;
///
/// fn main() {
///     let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
///     let mean = mean(&data).unwrap();
///     println!("mean: {:?}", mean);
///
///     assert_eq!(mean, 5.5);
/// }
/// ```
pub fn mean<Number: Into<f64> + Copy>(numbers: &[Number]) -> Option<f64> {
    let mut sum: f64 = 0.0;
    for &num in numbers {
        sum += num.into();
    }
    let count = numbers.len();

    // Check to prevent division by zero
    if count > 0 {
        Some(sum / count as f64)
    } else {
        None
    }
}

/// Computes the variance of a vector/array of numbers `$[Number]`
/// 
/// # Examples
/// ```
/// use hyte::utils::variance;
///
/// fn main() {
///     let data = vec![1, 2, 3, 4, 5, 6];
///     let variance = variance(&data).unwrap();
///     println!("variance: {:?}", variance);
///
///     assert_eq!(variance, 3.5);
/// }
/// ```
pub fn variance<Number: Into<f64> + Copy>(data: &[Number]) -> Option<f64> {
    let len = data.len();
    if len == 0 {
        return None;
    }

    let mean = mean(data);
    let variance = data.iter()
                       .map(|&value| {
                           let diff = value.into() - mean.unwrap();
                           diff * diff
                       })
                       .sum::<f64>() / (len - 1) as f64;

    Some(variance)
}