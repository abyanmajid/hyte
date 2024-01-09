//! Module responsible for all items needed to perform Pearson's Chi-squared tests.

use statrs::distribution::{ChiSquared, ContinuousCDF};
use crate::utils::{Matrix, Conclusion};

fn contains_negative<Number: Into<f64> + Clone>(matrix: &[Vec<Number>]) -> bool {
    matrix.iter().any(|row| row.iter().any(|num| num.clone().into() < 0.0))
}

fn has_different_rows<Number: Into<f64> + Copy>(matrix: &[Vec<Number>]) -> bool {
    if let Some(first_row_len) = matrix.first().map(|row| row.len()) {
        matrix.iter().any(|row| row.len() != first_row_len)
    } else {
        panic!("\n[HYTE-Panic] You must not pass in an empty matrix!\n")
    }
}

fn compute_totals<Number: Into<f64> + Copy>(matrix: &Vec<Vec<Number>>) -> Totals<f64> {
    let mut totals = Totals {
        column_totals: vec![0.0; matrix.first().map_or(0, Vec::len)],
        row_totals: vec![0.0; matrix.len()],
        grand_total: 0.0,
    };

    for (i, row) in matrix.into_iter().enumerate() {
        let mut row_total = 0.0;
        for (j, &num) in row.into_iter().enumerate() {
            let num_f64: f64 = num.into();
            row_total += num_f64;
            totals.column_totals[j] += num_f64;
        }
        totals.row_totals[i] = row_total;
        totals.grand_total += row_total;
    }

    totals
}

#[derive(Debug)]

struct Totals<Number: Into<f64> + Copy> {
    column_totals: Vec<Number>,
    row_totals: Vec<Number>,
    grand_total: Number
}

/// A struct for storing the resulting test statistic and p-value from Pearson's Chi-squared tests.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct ChiSquareResult {
    pub test_type: &'static str,
    pub statistic: f64,
    pub df: usize,
    pub p: f64,
}

/// Calculates Chi-square statistic and p-value for a Pearson's Chi-squared test. 
/// It returns a `ChiSquareResult` instance with the Chi-square statistic and p-value as fields `statistic` and `p` respectively.
/// 
/// # Important Notes
/// 
/// To perform test of independence, pass `"toi"` to `test_type`. Else, pass "`gof`" to perform a goodness of fit test.
/// 
/// ***-IMPORTANT NOTE-*** on `observed_matrix` parameter:
/// 
/// - To perform a test of independence `"toi"`, you must pass a `Matrix::TwoDimensional(Vec<Vec<Number>>)` variant to `observed_matrix`.
/// - To perform a goodness of fit test `"gof"`, you must apss a `Matrix::OneDimensional(Vec<Number>)` variant to `observed_matrix`.
/// 
/// ***-IMPORTANT NOTE-*** on `gof_probabilities` parameter:
/// 
/// - To perform a test of independence `"toi"`, you must pass an `Option::None` variant to `gof_probabiltiies`
/// - To perform a goodness of fit test `"gof"`, you must pass an `Option::Some(Vec<f64>)` variant to `gof_probabilities`
/// 
/// ## Test of Independence (Example)
/// 
/// The following is an example of how you can perform Pearson's Chi-squared test of independence
/// 
/// ```
/// use hyte::chisquare;
/// use hyte::utils::Matrix;
///
/// fn main() {
///     let observed_frequencies = Matrix::TwoDimensional(vec![vec![762, 327, 468], 
///                                                            vec![484, 239, 477]]);
///     let unwrapped_results = chisquare::test(
///         "toi", 
///         observed_frequencies, 
///         None, 
///         true
///     );
///
///     assert_ne!(unwrapped_results, None);
///
///     let results = unwrapped_results.unwrap();
///     println!("{:#?}", results);
///
///     assert_eq!(results.test_type, "Pearson's Chi-squared Test of Independence");
///     assert_eq!(results.statistic, 30.070149095754672);
///     assert_eq!(results.df, 2);
///     assert_eq!(results.p, 0.00000029535891832299654);
/// }
/// ```
/// 
/// ## Goodness of Fit (Example)
/// 
/// The following is an example of how you can perform Pearson's Chi-squared goodness of fit test.
/// 
/// ```
/// use hyte::chisquare;
/// use hyte::utils::Matrix;
///
/// fn main() {
///     let unwrapped_results = chisquare::test(
///         "gof",
///         Matrix::OneDimensional(vec![30, 40, 30]),
///         Some(vec![0.25, 0.5, 0.25]),
///         true
///     );
///
///     assert_ne!(unwrapped_results, None);
///
///     let results = unwrapped_results.unwrap();
///     println!("{:#?}", results);
///
///     assert_eq!(results.test_type, "Pearson's Chi-squared Goodness Of Fit");
///     assert_eq!(results.statistic, 4.0);
///     assert_eq!(results.df, 2);
///     assert_eq!(results.p, 0.1353352832366128);
/// }
/// ````
/// 
pub fn test<Number: Into<f64> + Copy>(
    test_type: &str, 
    observed_matrix: Matrix<Number>, 
    gof_probabilities: Option<Vec<f64>>, 
    print_output: bool
) -> Option<ChiSquareResult> {
    match (test_type, observed_matrix) {
        ("toi", Matrix::TwoDimensional(matrix)) => {
            if matrix.len() == 0 { panic!("\n[HYTE-Panic] You must not pass in an empty matrix!\n"); };
            if contains_negative(&matrix) {
                panic!("\n[HYTE-Panic] You must not pass in a matrix with a negative number!\n");
            } else if has_different_rows(&matrix) {
                panic!("\n[HYTE-Panic] You must not pass in a matrix with rows of different lengths!\n");
            }
            toi(matrix, print_output)
        },
        ("gof", Matrix::OneDimensional(matrix)) => {
            if matrix.len() == 0 { panic!("\n[HYTE-Panic] You must not pass in an empty matrix!\n"); };
            if matrix.iter().any(|&num| num.into() < 0.0) {
                panic!("\n[HYTE-Panic] You must not pass in a vector with a negative number!\n");
            }
            if let Some(probabilities) = gof_probabilities.as_ref() {
                if probabilities.iter().any(|&prob| prob < 0.0 || prob > 1.0) {
                    panic!("\n[HYTE-Panic] You must not pass in a vector with a negative number!\n");
                };
                if probabilities.len() != matrix.len() {
                    panic!(
                        "[HYTE-Panic] The lengths of your observed vector ({}) and expected probabilities vector ({}) do not match!",
                        matrix.len(),
                        probabilities.len()
                    );
                };
            } else {
                panic!("[HYTE-Panic] Expected probabilities must be provided for the goodness of fit test.");
            }
            gof(matrix, gof_probabilities, print_output)
        },
        _ => panic!("\n[HYTE-Panic] Test type for a ChiSquare test must be \"toi\" or \"gof\"!\n"),
    }
}

fn toi<Number: Into<f64> + Copy>(matrix: Vec<Vec<Number>>, print_output: bool) -> Option<ChiSquareResult> {
    let totals = compute_totals(&matrix);

    let mut statistic: f64 = 0.0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            let num_f64: f64 = num.into(); // Convert Number to f64
            let expected_frequency = (totals.row_totals[i] * totals.column_totals[j]) / totals.grand_total;
            statistic += ((num_f64 - expected_frequency).powi(2)) / expected_frequency;
        }
    };

    let df = (&matrix.len() - 1) * (&matrix[0].len() - 1);
    let chi = ChiSquared::new(df as f64).unwrap();
    let p = 1.0 - chi.cdf(statistic);

    let results = ChiSquareResult {
        test_type: "Pearson's Chi-squared Test of Independence",
        statistic,
        df,
        p,
    };
    if print_output {println!("\n------------------ HYTE ------------------\n\n{}\n\nX^2 test statistic = {:.2}\np-value = {:.3e}\nDegrees of freedom = {}\n\n------------------ HYTE ------------------\n", results.test_type, results.statistic, results.p, results.df);}
    
    Some(results)
}

fn gof<Number: Into<f64> + Copy>(matrix: Vec<Number>, gof_probabilities: Option<Vec<f64>>, print_output: bool) -> Option<ChiSquareResult> {
    let mut total: f64 = 0.0;
    
    for &num in matrix.iter() {
        total += num.into();
    };
    
    let mut expected_frequencies: Vec<f64> = Vec::new();
    let mut statistic: f64 = 0.0;
    
    for chance in gof_probabilities.unwrap() {
        expected_frequencies.push(chance * total);
    };

    for (i, &num) in matrix.iter().enumerate() {
        let num_f64: f64 = num.into();
        statistic += ((num_f64 - expected_frequencies[i]).powi(2)) / expected_frequencies[i];
    };

    let df = &matrix.len() - 1;
    let chi = ChiSquared::new(df as f64).unwrap();
    let p = 1.0 - chi.cdf(statistic);

    let results = ChiSquareResult {
        test_type: "Pearson's Chi-squared Goodness Of Fit",
        statistic,
        df,
        p,
    };
    if print_output {println!("\n---------------- HYTE ----------------\n\n{}\n\nX^2 test statistic = {:.2}\np-value = {:.3e}\nDegrees of freedom = {}\n\n---------------- HYTE ----------------\n", results.test_type, results.statistic, results.p, results.df);}
    
    Some(results)
}

impl ChiSquareResult {
    /// Concludes if a `ChiSquareResult` instance should be rejected given a specified significance value, by returning a `Conclusion` variant `Reject` or `DoNotReject`
    pub fn conclude(&self, significance_level: f64, print_output: bool) -> Conclusion {
        if self.p < significance_level {
            if print_output {println!("\n---------------------------- HYTE ----------------------------\n\nStatistical Conclusion\n\np-value = ({:.3e})\nsignificance level = ({:.3e})\n\np-value < s. l, therefore reject H_0\n\nThere is sufficient evidence to reject the null hypothesis.\n\n---------------------------- HYTE ----------------------------\n", self.p, significance_level);};
            Conclusion::Reject
        } else {
            if print_output {println!("\n---------------------------- HYTE ----------------------------\n\nStatistical Conclusion\n\np-value = ({:.3e})\nsignificance level = ({:.3e})\n\np-value > s. l, therefore do not reject H_0\n\nThere is insufficient evidence to reject the null hypothesis.\n\n---------------------------- HYTE ----------------------------\n", self.p, significance_level);};
            Conclusion::DoNotReject
        }
    }

    /// Concludes if a `ChiSquareResult` instance should be rejected using a significance level of 0.05 by convention, by returning a `Conclusion` variant `Reject` or `DoNotReject`
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
