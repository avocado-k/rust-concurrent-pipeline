// # Thread Partitioning Module

use std::thread;

#[derive(Debug)]
pub struct PartitionResult {
    pub total_avg: f64,
    pub thread_counts: usize,
}

pub fn parallel_average(data: &[f64], num_threads: usize) -> PartitionResult {
    let chunk_size = data.len() / num_threads;
    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            data.len()
        } else {
            start + chunk_size
        };

        let chunk = data[start..end].to_vec();
        handles.push(thread::spawn(move || {
            chunk.iter().sum::<f64>() / chunk.len() as f64
        }));
    }

    let total: f64 = handles.into_iter().map(|h| h.join().unwrap()).sum();

    PartitionResult {
        total_avg: total / num_threads as f64,
        thread_counts: num_threads,
    }
}
