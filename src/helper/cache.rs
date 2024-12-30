use std::time::{Duration, Instant};

const DEFAULT_CACHE_TTL: Duration = Duration::from_secs(30);

pub struct Cache<T: Clone>
{
    pub data: T,
    pub created_at: Instant,
    pub ttl: Duration
}

impl<T> Cache<T>
where
    T: Clone
{
    pub fn new(data: T, ttl: Option<Duration>) -> Self {
        Self {
            data,
            created_at: Instant::now(),
            ttl: ttl.unwrap_or(DEFAULT_CACHE_TTL)
        }
    }
    
    pub fn validate(&self) -> bool {
        self.created_at.elapsed() < self.ttl
    }
}