// These are functions that calculate the mean and the standard deviation of a distribution in a Vec<usize>

pub fn average(nums: &Vec<usize>, num_connections: usize) -> f64 {
    let sum: usize = nums.iter().sum();
    let mean = sum as f64 / num_connections as f64;
    mean
}

pub fn calculate_stdv(numbers: &Vec<usize>, num_connections: usize) -> f64 {
    let squares: f64 = numbers.iter().map(|&x| (x as f64 - average(numbers, num_connections)).powf(2.0)).sum();
    let variance = squares / num_connections as f64;
    let st_dv = variance.powf(0.5);
    st_dv
}