use hyte::{ChiSquare, Matrix};

fn main() {
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