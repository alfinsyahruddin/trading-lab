use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::Instant;

/// Global rate limiter for Sectors.app API calls.
///
/// Implements the Generic Cell Rate Algorithm (GCRA) token bucket with burst capacity,
/// enforcing a single global rate limit across all Sectors API endpoints (screener,
/// daily transactions, foreign flow) and across all concurrent backtest executions.
///
/// If an HTTP 429 response is encountered by any request, the rate limiter can be paused
/// globally via [`notify_429`], blocking all subsequent calls across all endpoints and backtests
/// for the duration of the cooldown window (typically 60 seconds).
#[derive(Debug)]
pub struct SectorsRateLimiter {
    capacity: u32,
    window: Duration,
    emission_interval: Duration,
    state: Mutex<RateLimiterState>,
}

#[derive(Debug)]
struct RateLimiterState {
    tat: Option<Instant>,
    paused_until: Option<Instant>,
}

impl SectorsRateLimiter {
    /// Creates a new [`SectorsRateLimiter`] allowing up to `capacity` requests per `window`.
    pub fn new(capacity: u32, window: Duration) -> Self {
        let safe_capacity = capacity.max(1);
        let safe_window = if window.is_zero() {
            Duration::from_secs(60)
        } else {
            window
        };
        let emission_interval = safe_window / safe_capacity;

        Self {
            capacity: safe_capacity,
            window: safe_window,
            emission_interval,
            state: Mutex::new(RateLimiterState {
                tat: None,
                paused_until: None,
            }),
        }
    }

    /// Returns the maximum burst capacity.
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Returns the rate limiting window duration.
    pub fn window(&self) -> Duration {
        self.window
    }

    /// Returns the emission interval (time allocated per request).
    pub fn emission_interval(&self) -> Duration {
        self.emission_interval
    }

    /// Acquires permission to dispatch a request to Sectors.app.
    ///
    /// Suspends asynchronously if rate limits or a 429 cooldown are active,
    /// waking up when it is safe to proceed.
    pub async fn acquire(&self) {
        let delay = {
            let mut state = self.state.lock().await;
            let now = Instant::now();

            let base = match state.paused_until {
                Some(paused) if paused > now => paused,
                Some(_) => {
                    state.paused_until = None;
                    now
                }
                None => now,
            };

            let tat = state.tat.unwrap_or(base);
            let new_tat = tat.max(base) + self.emission_interval;
            let burst_offset = self.emission_interval * self.capacity;

            let target_instant = if new_tat > base + burst_offset {
                new_tat - burst_offset
            } else {
                base
            };

            state.tat = Some(new_tat);
            target_instant
                .checked_duration_since(now)
                .unwrap_or(Duration::ZERO)
        };

        if !delay.is_zero() {
            tokio::time::sleep(delay).await;
        }

        // Re-check in case an HTTP 429 was reported while this task was sleeping
        self.wait_if_paused().await;
    }

    /// Signals that an HTTP 429 Too Many Requests response was received.
    ///
    /// Pauses all future and in-flight API requests across all endpoints and backtests
    /// for `pause_duration` (typically 60s), matching Sectors.app quota reset behavior.
    pub async fn notify_429(&self, pause_duration: Duration) {
        let mut state = self.state.lock().await;
        let now = Instant::now();
        let new_paused_until = now + pause_duration;

        let resume_at = match state.paused_until {
            Some(existing) if existing > new_paused_until => existing,
            _ => new_paused_until,
        };

        eprintln!(
            "[SectorsRateLimiter] HTTP 429 received: pausing all Sectors API calls globally for {:?} until {:?}",
            pause_duration, resume_at
        );

        state.paused_until = Some(resume_at);
        state.tat = Some(match state.tat {
            Some(tat) if tat > resume_at => tat,
            _ => resume_at,
        });
    }

    /// Checks if a global 429 cooldown is active and sleeps until it expires.
    async fn wait_if_paused(&self) {
        loop {
            let pause_delay = {
                let mut state = self.state.lock().await;
                let now = Instant::now();
                match state.paused_until {
                    Some(paused) if paused > now => paused.checked_duration_since(now),
                    Some(_) => {
                        state.paused_until = None;
                        None
                    }
                    None => None,
                }
            };

            if let Some(dur) = pause_delay {
                tokio::time::sleep(dur).await;
            } else {
                break;
            }
        }
    }
}

impl Default for SectorsRateLimiter {
    fn default() -> Self {
        Self::new(25, Duration::from_secs(60))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Instant as StdInstant;

    #[tokio::test]
    async fn should_allow_burst_up_to_capacity_immediately() {
        let limiter = SectorsRateLimiter::new(5, Duration::from_secs(10));
        let start = StdInstant::now();

        for _ in 0..5 {
            limiter.acquire().await;
        }

        let elapsed = start.elapsed();
        assert!(
            elapsed < Duration::from_millis(50),
            "Burst requests took too long: {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn should_throttle_requests_beyond_burst_capacity() {
        // 2 requests per 100ms -> emission interval = 50ms
        let limiter = SectorsRateLimiter::new(2, Duration::from_millis(100));

        // First 2: immediate burst
        limiter.acquire().await;
        limiter.acquire().await;

        // 3rd: should wait ~50ms
        let start = StdInstant::now();
        limiter.acquire().await;
        let elapsed = start.elapsed();

        assert!(
            elapsed >= Duration::from_millis(40),
            "Expected 3rd request to be throttled for ~50ms, elapsed: {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn should_refill_tokens_after_idle_period() {
        // 2 requests per 60ms -> emission interval = 30ms
        let limiter = SectorsRateLimiter::new(2, Duration::from_millis(60));

        limiter.acquire().await;
        limiter.acquire().await;

        // Sleep to let tokens refill
        tokio::time::sleep(Duration::from_millis(70)).await;

        let start = StdInstant::now();
        limiter.acquire().await;
        limiter.acquire().await;
        let elapsed = start.elapsed();

        assert!(
            elapsed < Duration::from_millis(20),
            "Refilled burst took too long: {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn should_pause_all_callers_globally_on_429() {
        let limiter = Arc::new(SectorsRateLimiter::new(10, Duration::from_secs(10)));
        let pause_duration = Duration::from_millis(120);

        limiter.notify_429(pause_duration).await;

        let start = StdInstant::now();
        let l1 = limiter.clone();
        let l2 = limiter.clone();

        let h1 = tokio::spawn(async move {
            l1.acquire().await;
        });
        let h2 = tokio::spawn(async move {
            l2.acquire().await;
        });

        h1.await.unwrap();
        h2.await.unwrap();

        let elapsed = start.elapsed();
        assert!(
            elapsed >= Duration::from_millis(100),
            "Expected tasks to wait for 429 pause (~120ms), elapsed: {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn should_coordinate_rate_limiting_across_multiple_tasks_and_endpoints() {
        // 4 requests per 100ms -> emission interval = 25ms
        let limiter = Arc::new(SectorsRateLimiter::new(4, Duration::from_millis(100)));
        let mut handles = Vec::new();

        let start = StdInstant::now();

        // Spawn 8 tasks across 2 simulated "endpoints"
        for i in 0..8 {
            let l = limiter.clone();
            handles.push(tokio::spawn(async move {
                l.acquire().await;
                i
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        let elapsed = start.elapsed();
        // 4 in burst (0ms) + 4 spaced by 25ms = at least 75-100ms
        assert!(
            elapsed >= Duration::from_millis(70),
            "Expected 8 requests to take at least ~75ms, elapsed: {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn should_coordinate_across_multiple_concurrent_backtest_runs() {
        // 6 requests per 120ms -> emission interval = 20ms
        let limiter = Arc::new(SectorsRateLimiter::new(6, Duration::from_millis(120)));
        let start = StdInstant::now();

        // Simulate 3 concurrent backtests, each making 4 requests across different endpoints
        let mut backtest_handles = Vec::new();
        for backtest_id in 0..3 {
            let l = limiter.clone();
            backtest_handles.push(tokio::spawn(async move {
                // Screener
                l.acquire().await;
                // Foreign flow (2 calls)
                l.acquire().await;
                l.acquire().await;
                // Daily transactions
                l.acquire().await;
                backtest_id
            }));
        }

        for h in backtest_handles {
            h.await.unwrap();
        }

        let elapsed = start.elapsed();
        // Total 12 requests: 6 in initial burst + 6 queued spaced by 20ms = ~120ms
        assert!(
            elapsed >= Duration::from_millis(100),
            "Expected 12 requests across 3 backtests to be throttled to ~120ms, elapsed: {:?}",
            elapsed
        );
    }

    #[test]
    fn should_handle_zero_capacity_and_zero_window_gracefully() {
        let limiter = SectorsRateLimiter::new(0, Duration::ZERO);
        assert_eq!(limiter.capacity(), 1);
        assert_eq!(limiter.window(), Duration::from_secs(60));
    }
}
