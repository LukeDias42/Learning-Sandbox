use std::time::{SystemTime, UNIX_EPOCH};

pub struct SplitMix {
    state: u64,
}

impl SplitMix {
    pub fn new() -> SplitMix {
        let since_the_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards")
            .as_millis();
        SplitMix {
            state: since_the_epoch as u64,
        }
    }

    pub fn build(initial_state: u64) -> SplitMix {
        SplitMix {
            state: initial_state,
        }
    }

    pub fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9_u64);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb_u64);
        z ^ (z >> 31)
    }

    pub fn next_float(&mut self) -> f64 {
        (self.next() as f64) / 2_f64.powi(64)
    }

    pub fn rand(&mut self, max: u64) -> u64 {
        (self.next_float() * max as f64) as u64
    }
}
