use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct LRUCache<K, V> {
    capacity: usize,
    entries: HashMap<K, V>,
    order: VecDeque<K>, // LRU 순서 추적
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
{
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
        }
    }

    // 항목 조회 (LRU 순서 갱신)
    fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(val) = self.entries.get(key) {
            self.order.retain(|k| k != key); // 기존 위치 제거
            self.order.push_front(key.clone()); // 앞으로 이동
            Some(val)
        } else {
            None
        }
    }

    // 항목 추가 (용량 초과 시 LRU 제거)
    fn insert(&mut self, key: K, value: V) {
        if self.entries.len() >= self.capacity {
            if let Some(oldest) = self.order.pop_back() {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(key.clone(), value);
        self.order.push_front(key);
    }
}

// 스레드 세이프 캐시 래퍼
#[derive(Clone)]
struct ThreadSafeCache<K, V> {
    inner: Arc<Mutex<LRUCache<K, V>>>,
}

impl<K, V> ThreadSafeCache<K, V>
where
    K: Eq + std::hash::Hash + Clone + Send + 'static,
    V: Send + 'static,
{
    fn new(capacity: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(LRUCache::new(capacity))),
        }
    }

    // 스레드 안전한 조회
    fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        let mut cache = self.inner.lock().unwrap(); // C의 pthread_mutex_lock과 유사
        cache.get(key).cloned()
    }

    // 스레드 안전한 삽입
    fn insert(&self, key: K, value: V) {
        let mut cache = self.inner.lock().unwrap();
        cache.insert(key, value);
    }
}

pub fn main() {
    // 공유 캐시 생성 (용량 100)
    let cache = ThreadSafeCache::new(100);

    // 멀티스레드 벤치마크
    let start = Instant::now();
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let cache_clone = cache.clone();
            thread::spawn(move || {
                for j in 0..100 {
                    cache_clone.insert(format!("key-{}-{}", i, j), j);
                    cache_clone.get(&format!("key-{}-{}", i, j));
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Multi-threaded time: {:.2}ms",
        start.elapsed().as_millis()
    );
}

#[test]
fn test_lru_cache() {
    let mut cache = LRUCache::new(2);
    cache.insert("a", 1);
    cache.insert("b", 2);
    assert_eq!(cache.get(&"a"), Some(&1));
    cache.insert("c", 3); // "b"가 제거됨
    assert_eq!(cache.get(&"b"), None);
}

#[test]
fn test_thread_safety() {
    let cache = ThreadSafeCache::new(10);
    let handle = thread::spawn(move || {
        cache.insert("key", 42);
    });
    handle.join().unwrap();
    assert_eq!(cache.get(&"key"), Some(42));
}