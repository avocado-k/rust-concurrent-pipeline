use rust_concurrent_pipeline::thread_partitioning;
use rust_concurrent_pipeline::channel_processing;
use rust_concurrent_pipeline::shared_cache;


fn main() {
    // Day 1: 스레드 분할 평균 계산
    let data: Vec<f64> = (0..1_000_000).map(|x| x as f64).collect();
    let result = thread_partitioning::parallel_average(&data, 4);
    println!(
        "Day 1 - {}개 스레드 사용, 평균값: {:.2}",
        result.thread_counts, result.total_avg
    );

    // Day 2: 채널 기반 병렬 처리
    channel_processing::main();

    // Day 3: 스레드 세이프 캐시 시스템
    shared_cache::main();
}