#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_array() {
        let data = vec![2.0, 4.0, 6.0, 8.0];
        let result = parallel_average(&data, 2);
        assert_eq!(result.total_avg, 5.0);
    }

    #[test]
    fn test_edge_cases() {
        let empty: Vec<f64> = vec![];
        let single = vec![42.0];
        assert_eq!(parallel_average(&single, 1).total_avg, 42.0);
    }