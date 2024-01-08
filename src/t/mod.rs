use statrs::distribution::StudentsT;
use statrs::distribution::ContinuousCDF;
use crate::utils::{Tails, mean, variance}; 

pub struct T {
    pub test_type: &'static str,
    pub statistic: f64,
    pub df: f64,
    pub p: f64,
}

impl T {
    pub fn test<Number: Into<f64> + Copy, Number2: Into<f64> + Copy>(data: Vec<Number>, expected_mean: Number2, tail: Tails, print_output: bool) -> T {
        let observed_mean = mean(&data).unwrap();
        let sample_size = data.len() as u32;
        let sd = variance(&data).unwrap().sqrt();
        Self::test_dataless(observed_mean, expected_mean.into(), sample_size, sd, tail, print_output)
    }

    pub fn test_dataless<Number: Into<f64> + Copy>(observed_mean: Number, expected_mean: Number, sample_size: u32, pop_sd: Number, tail: Tails, print_output: bool) -> T {
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

        let results = T {
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

        results
    }

    pub fn test_two_samples<Number: Into<f64> + Copy>(data1: Vec<Number>, data2: Vec<Number>, print_output: bool) -> T {
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
        
        let results = T {
            test_type: "(2-Sample) T-Test for Mean",
            statistic,
            df,
            p,
        };

        if print_output {println!("\n---------- HYTE ----------\n\n{}\n\nZ test statistic = {:.2}\np-value = {:.3e}\nDegrees of freedom = {:.2}\n\n---------- HYTE ----------\n", results.test_type, results.statistic, results.p, results.df)};

        results
    }
}