use std::collections::HashMap;

fn mean(data: &[i32]) -> Option<f32> {
    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    match count {
        0 => None,
        _ => Some(sum / count as f32),
    }
}

fn std_deviation(data: &[i32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|x| (*x as f32 - data_mean).powi(2))
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

fn z_score(data: &[i32], index: usize) -> Option<f32> {
    match (mean(data), std_deviation(data), data.get(index)) {
        (Some(data_mean), Some(data_std_deviation), Some(x)) => {
            let diff = *x as f32 - data_mean;
            Some(diff / data_std_deviation)
        }
        _ => None,
    }
}

fn calc_mean() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];
    println!("Mean of the data is {:?}", mean(&data));
}

fn calc_mode() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let frequencies = data.iter().fold(HashMap::new(), |mut freqs, &value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });

    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value);

    println!("Mode of the data is {:?}", mode);
}

fn calc_mean_std_and_zscore() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let data_mean = mean(&data);
    println!("Mean is {:?}", data_mean);

    let data_std_deviation = std_deviation(&data);
    println!("Standard deviation is {:?}", data_std_deviation);

    let index = 4;
    let data_zscore = z_score(&data, index);
    println!(
        "Z-score of data at index {} (with value {}) is {:?}",
        index, data[index], data_zscore
    );
}

fn main() {
    calc_mean();
    calc_mode();
    calc_mean_std_and_zscore();
}
