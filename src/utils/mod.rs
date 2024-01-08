pub enum Matrix<Number> {
    TwoDimensional(Vec<Vec<Number>>),
    OneDimensional(Vec<Number>),
}

pub enum Tails {
    LOWER,
    UPPER,
    BOTH,
}

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