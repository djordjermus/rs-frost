use std::time::{SystemTime, UNIX_EPOCH};

pub struct Random
{
    seed : u64
}

impl Random
{
    /// Creates a new instance of 'Random'
    pub fn new() -> Self
    {
        return Self {
            seed: 
                 SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Error while reading current time")
                .as_nanos() as u64
        };
    }
    pub fn get_seed(&self) -> u64
    {
        return self.seed;
    }
    pub fn set_seed(&mut self, seed : u64)
    {
        self.seed = seed;
    }
    
    #[inline(always)]
    pub fn next_seed(&mut self) -> u64
    {
        // Cache return and current seed
        let ret : u64 = self.seed;
        let mut seed : u64 = ret;
        
        // Calculate new seed
        seed = seed ^ (seed << 12);
        seed = seed ^ (seed >> 25);
        seed = seed ^ (seed << 27);

        // Set new seed and 
        self.seed = seed;
        return ret;
    }

    /// Generates a random u64 in range [from, from + range)
    pub fn next_u64(&mut self, from: u64, range: u64) -> u64
    {
        return from + (self.next_seed() % range);
    }

    /// Generates a random i64 in range [from, from + range)
    pub fn next_i64(&mut self, from: i64, range: i64) -> i64
    {
        return from + (self.next_seed() % range as u64) as i64;
    }

    /// Generates a random f64 in range [from, from + range)
    pub fn next_f64(&mut self, from: f64, range: f64) -> f64
    {
        return from + (self.next_seed() as f64 * 0.0000000000000001f64 % range);
    }
}