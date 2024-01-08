use statrs::distribution::StudentsT;
use statrs::distribution::ContinuousCDF;
use crate::utils::Tails;    

pub struct T {
    pub test_type: &'static str,
    pub statistic: f64,
    pub p: f64,
}

impl T {
    pub fn test_dataless(observed_mean: f64, expected_mean: f64, sample_size: u32, pop_sd: f64, df: usize, tail: Tails, print_output: bool) -> T {
        let statistic: f64 = (observed_mean - expected_mean) / (pop_sd / (sample_size as f64).sqrt());
        let t_distribution = StudentsT::new(0.0, 1.0, df as f64).unwrap();
        let p: f64;
        let test_type: &'static str;

        match tail {
            Tails::LOWER => {
                p = t_distribution.cdf(statistic);
                test_type = "One-Sided T-Test for Mean (Lower-Tailed)";
            },
            Tails::UPPER => {
                p = 1.0 - t_distribution.cdf(statistic);
                test_type = "One-Sided T-Test for Mean (Upper-Tailed)";
            },
            Tails::BOTH => {
                p = 2.0 * t_distribution.cdf(-statistic.abs());
                test_type = "Two-Sided T-Test for Mean"
            },
        }

        let results = T {
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

        results
    }

    pub fn test_two_samples() {}
}