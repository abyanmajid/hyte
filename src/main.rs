use hyte::chisquare::ChiSquare;
use hyte::z::{Z, Tails};
use hyte::utils::Matrix;

fn try_chisquare() {
    let results1 = ChiSquare::test(
        "toi", 
        Matrix::TwoDimensional(vec![vec![762, 327, 468], vec![484, 239, 477]]), 
        None, 
        true);
    println!("test_type: {}, statistic: {}, df: {}, p-value: {}", results1.test_type, results1.statistic, results1.df, results1.p);
    // ChiSquare::test("toi", vec![vec![535, 8], vec![2269, 82], vec![937, 174], vec![14, 5]]);

    let results2 = ChiSquare::test(
        "gof",
        Matrix::OneDimensional(vec![30, 40, 30]),
        Some(vec![0.25, 0.5, 0.25]),
        true
    );
    println!("test_type: {}, statistic: {}, df: {}, p-value: {}", results2.test_type, results2.statistic, results2.df, results2.p);
}

fn try_z() {
    let results1 = Z::test(
        1.2, 
        1.0, 
        30, 
        0.5,
        Tails::LOWER,
        true,
    );
    println!("test_type: {}, statistic: {}, p-value: {}", results1.test_type, results1.statistic, results1.p);
    let results2 = Z::test(
        1.2, 
        1.0, 
        30, 
        0.5,
        Tails::UPPER,
        true,
    );
    println!("test_type: {}, statistic: {}, p-value: {}", results2.test_type, results2.statistic, results2.p);
    let results3 = Z::test(
        1.2, 
        1.0, 
        30, 
        0.5,
        Tails::BOTH,
        true,
    );
    println!("test_type: {}, statistic: {}, p-value: {}", results3.test_type, results3.statistic, results3.p);
}

fn main() {
    // try_chisquare();
    // try_z();
}