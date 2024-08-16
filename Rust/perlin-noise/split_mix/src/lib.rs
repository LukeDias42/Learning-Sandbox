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

    pub fn new_from(initial_state: u64) -> SplitMix {
        SplitMix {
            state: initial_state,
        }
    }

    pub fn next_rand(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9_u64);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb_u64);
        z ^ (z >> 31)
    }

    pub fn next_float(&mut self) -> f64 {
        (self.next_rand() as f64) / 2_f64.powi(64)
    }

    pub fn rand(&mut self, max: u64) -> u64 {
        (self.next_float() * max as f64) as u64
    }

    pub fn rand_range(&mut self, min: u64, max: u64) -> u64 {
        let r = self.rand(max - min);
        r + min
    }

    pub fn shuffle_vec<T>(&mut self, vec: &mut [T])
    where
        T: Copy,
    {
        let len = vec.len() as u64;
        for i in 0..len {
            let j = self.rand_range(i, len);
            vec.swap(i as usize, j as usize);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_expected_values() {
        let mut sm = SplitMix::new_from(1234567);
        let mut results = Vec::new();
        for _ in 0..5 {
            results.push(sm.next_rand())
        }
        assert_eq!(
            results,
            vec![
                6457827717110365317,
                3203168211198807973,
                9817491932198370423,
                4593380528125082431,
                16408922859458223821
            ]
        );
    }

    #[test]
    fn correct_rand() {
        let mut sm = SplitMix::new_from(987654321);
        let mut results = vec![0, 0, 0, 0, 0];
        for _ in 0..100_000 {
            results[sm.rand(5) as usize] += 1
        }
        assert_eq!(results, vec![20027, 19892, 20073, 19978, 20030]);
    }
}

impl Default for SplitMix {
    fn default() -> Self {
        Self::new()
    }
}
