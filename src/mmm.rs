use std::collections::HashMap;

pub fn mean(items: &[i32]) -> f64 {
    let mut mean: f64 = 0.0;

    for item in items.iter() {
        mean += *item as f64;
    }

    mean = mean / items.len() as f64;
    mean
}
