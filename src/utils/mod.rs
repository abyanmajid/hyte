use std::collections::HashMap;

pub enum Matrix<Number> {
    TwoDimensional(Vec<Vec<Number>>),
    OneDimensional(Vec<Number>),
}

pub enum Tails {
    LOWER,
    UPPER,
    BOTH,
}

pub fn mean(numbers: &[f64]) -> Option<f64> {
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len();

    // Check to prevent division by zero
    if count > 0 {
        Some(sum / count as f64)
    } else {
        None
    }
}

pub fn mode(numbers: &[i32]) -> Vec<i32> {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    let max_occurrences = occurrences.values().cloned().max().unwrap_or(0);
    occurrences.into_iter()
        .filter(|&(_, count)| count == max_occurrences)
        .map(|(val, _)| val)
        .collect()
}

pub fn median(numbers: &mut [i32]) -> f64 {
    numbers.sort_unstable();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] as f64 + numbers[mid] as f64) / 2.0
    } else {
        numbers[mid] as f64
    }
}

pub fn range(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        None
    } else {
        Some(numbers.iter().max().unwrap() - numbers.iter().min().unwrap())
    }
}