use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

pub struct CounterWithTimeout {
    count: u64,
    last_reset: Instant,
    max_count: u64,
    max_duration: Duration,
}

impl CounterWithTimeout {
    fn new(max_count: u64, max_duration: Duration) -> Self {
        Self {
            count: 0,
            last_reset: Instant::now(),
            max_count,
            max_duration,
        }
    }


    /// 增加计数并检查是否触发
    pub fn check_and_add(&mut self, n: u64) -> bool {
        let time_exceeded = self.last_reset.elapsed() >= self.max_duration;
        let count_exceeded = self.count >= self.max_count;
        if time_exceeded || count_exceeded {
            self.reset();
            return true;
        }
        self.count += n;
        false
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.last_reset = Instant::now();
    }
}

//
// ----------- 全局单例部分 -----------
//

static GLOBAL_COUNTER: OnceLock<Mutex<CounterWithTimeout>> = OnceLock::new();

/// 获取全局计数器（线程安全）
pub fn global_counter() -> &'static Mutex<CounterWithTimeout> {
    GLOBAL_COUNTER.get_or_init(|| {
        Mutex::new(CounterWithTimeout::new(
            64 * 1024 * 1024,       // 最大计数阈值
            Duration::from_secs(5), // 最大时间阈值
        ))
    })
}
