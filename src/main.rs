use hyte::chisquare::ChiSquare;
use hyte::z::Z;
use hyte::t::T;
use hyte::utils::{Matrix, Tails};

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
    
    
    // ------------------------------------------ DATALESS ------------------------------------------
    
    let results1 = Z::test_dataless(
        1.2, 
        1.0, 
        30, 
        0.5,
        Tails::LOWER,
        true,
    );
    println!("test_type: {}, statistic: {}, p-value: {}", results1.test_type, results1.statistic, results1.p);
    
    let results2 = Z::test_dataless(
        1.2, 
        1.0, 
        30, 
        0.5,
        Tails::UPPER,
        true,
    );
    println!("test_type: {}, statistic: {}, p-value: {}", results2.test_type, results2.statistic, results2.p);
    
    let results3 = Z::test_dataless(
        1.2, 
        1.0, 
        30, 
        0.5,
        Tails::BOTH,
        true,
    );
    println!("test_type: {}, statistic: {}, p-value: {}", results3.test_type, results3.statistic, results3.p);
}

fn try_t() {
    
    // ------------------------------------------ DATALESS ------------------------------------------

    let results1 = T::test_dataless(
        1.2, 
        1.0, 
        30, 
        0.5,
        29,
        Tails::LOWER,
        true,
    );
    println!("test_type: {}, statistic: {}, p-value: {}", results1.test_type, results1.statistic, results1.p);
    
    let results2 = T::test_dataless(
        1.2, 
        1.0, 
        30, 
        0.5,
        29,
        Tails::UPPER,
        true,
    );
    println!("test_type: {}, statistic: {}, p-value: {}", results2.test_type, results2.statistic, results2.p);
    
    let results3 = T::test_dataless(
        1.2, 
        1.0, 
        30, 
        0.5,
        29,
        Tails::BOTH,
        true,
    );
    println!("test_type: {}, statistic: {}, p-value: {}", results3.test_type, results3.statistic, results3.p);
    
    let data1 = vec![20, 22, 19, 20, 21, 20, 19, 21, 22, 18];
    let data2 = vec![22, 24, 23, 24, 25, 23, 24, 23, 22, 24];
    let results4 = T::test_two_samples(data1, data2, true);
    println!("test_type: {}, statistic: {}, p-value: {}", results4.test_type, results4.statistic, results4.p);

}

fn main() {
    // try_chisquare();
    // try_z();
    try_t();
}