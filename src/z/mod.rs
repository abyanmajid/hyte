use statrs::distribution::Normal;
use statrs::distribution::ContinuousCDF;
use crate::utils::{Tails, mean, variance};

pub struct Z {
    pub test_type: &'static str,
    pub statistic: f64,
    pub p: f64,
}

impl Z {
    pub fn test<Number: Into<f64> + Copy, Number2: Into<f64> + Copy>(data: Vec<Number>, expected_mean: Number2, tail: Tails, print_output: bool) -> Z {
        let observed_mean = mean(&data).unwrap();
        let sample_size = data.len() as u32;
        let sd = variance(&data).unwrap().sqrt();
        Self::test_dataless(observed_mean, expected_mean.into(), sample_size, sd, tail, print_output)
    }

    pub fn test_dataless<Number: Into<f64> + Copy>(observed_mean: Number, expected_mean: Number, sample_size: u32, pop_sd: Number, tail: Tails, print_output: bool) -> Z {
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

        let results = Z {
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
}