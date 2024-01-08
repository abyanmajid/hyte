use statrs::distribution::{ChiSquared, ContinuousCDF};

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

pub enum Matrix<Number> {
    TwoDimensional(Vec<Vec<Number>>),
    OneDimensional(Vec<Number>),
}

#[derive(Debug)]
struct Totals<Number: Into<f64> + Copy> {
    column_totals: Vec<Number>,
    row_totals: Vec<Number>,
    grand_total: Number
}

pub struct ChiSquare {
    pub test_type: &'static str,
    pub statistic: f64,
    pub df: usize,
    pub p: f64,
}

impl ChiSquare {
    pub fn test<Number: Into<f64> + Copy>(
        test_type: &str, 
        observed_matrix: Matrix<Number>, 
        gof_probabilities: Option<Vec<f64>>, 
        print_output: bool
    ) -> ChiSquare {
        match (test_type, observed_matrix) {
            ("toi", Matrix::TwoDimensional(matrix)) => {
                if contains_negative(&matrix) {
                    panic!("\n[HYTE-Panic] You must not pass in a matrix with a negative number!\n");
                } else if has_different_rows(&matrix) {
                    panic!("\n[HYTE-Panic] You must not pass in a matrix with rows of different lengths!\n");
                }
                Self::toi(matrix, print_output)
            },
            ("gof", Matrix::OneDimensional(matrix)) => {
                if matrix.iter().any(|&num| num.into() < 0.0) {
                    panic!("\n[HYTE-Panic] You must not pass in a vector with a negative number!\n");
                }
                if let Some(probabilities) = gof_probabilities.as_ref() {
                    if probabilities.len() != matrix.len() {
                        panic!(
                            "[HYTE-Panic] The lengths of your observed vector ({}) and expected probabilities vector ({}) do not match!",
                            matrix.len(),
                            probabilities.len()
                        );
                    }
                } else {
                    panic!("[HYTE-Panic] Expected probabilities must be provided for the goodness of fit test.");
                }
                Self::gof(matrix, gof_probabilities, print_output)
            },
            _ => panic!("\n[HYTE-Panic] Test type for a ChiSquare test must be \"toi\" or \"gof\"!\n"),
        }
    }
    
    fn toi<Number: Into<f64> + Copy>(matrix: Vec<Vec<Number>>, print_output: bool) -> ChiSquare {
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

        let results = ChiSquare {
            test_type: "Pearson's Chi-squared Test of Independence",
            statistic,
            df,
            p,
        };
        if print_output {println!("\n------------------ HYTE ------------------\n\n{}\n\nX^2 test statistic = {:.2}\np-value = {:.3e}\nDegrees of freedom = {}\n\n------------------ HYTE ------------------\n", results.test_type, results.statistic, results.p, results.df);}
        
        results
    }

    fn gof<Number: Into<f64> + Copy>(matrix: Vec<Number>, gof_probabilities: Option<Vec<f64>>, print_output: bool) -> ChiSquare {
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

        let results = ChiSquare {
            test_type: "Pearson's Chi-squared Goodness Of Fit",
            statistic,
            df,
            p,
        };
        if print_output {println!("\n---------------- HYTE ----------------\n\n{}\n\nX^2 test statistic = {:.2}\np-value = {:.3e}\nDegrees of freedom = {}\n\n---------------- HYTE ----------------\n", results.test_type, results.statistic, results.p, results.df);}
        
        results
    }
}