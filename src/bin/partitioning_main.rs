use rust_concurrent_pipeline::thread_partitioning;

fn main() {
    // 100만 개 데이터 생성 (C의 malloc 대신 Vec 사용)
    let data: Vec<f64> = (0..1_000_000).map(|x| x as f64).collect();

    // C의 pthread_create 대응 코드 실행
    let result = thread_partitioning::parallel_average(&data, 4);
    
    println!(
        "reuslt: {}개 스레드 사용, 평균값: {:.2}",
        result.thread_counts, result.total_avg
    );
}