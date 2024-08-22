use rand::rngs::ThreadRng;
use rand_distr::Distribution;

/// Random number generator based on the normal distribution. This generator,
/// ensures the returned random numbers are integers and within the specified
/// interval.
#[derive(Clone)]
#[allow(unused)]
pub struct NormalDistribution {
    start: usize,
    end: usize,

    std_deviation: f32,
}

#[allow(unused)]
impl NormalDistribution {
    pub fn new_inclusive(start: usize, end: usize, std_deviation: f32) -> Self {
        assert!(start < end);
        assert!(std_deviation >= 1.);
        Self { start, end, std_deviation }
    }

    pub fn set_std_deviation(&mut self, std_deviation: f32) {
        assert!(std_deviation >= 1.);
        self.std_deviation = std_deviation;
    }

    pub fn sample(&self, expected_value: usize, rng: &mut ThreadRng) -> usize {
        assert!(expected_value >= self.start);
        assert!(expected_value <= self.end);

        // Create distribution and sample value
        let dist =
            rand_distr::Normal::new(expected_value as f32, self.std_deviation)
                .unwrap();

        let mut raw = dist.sample(rng);
        let mut val = raw.round() as usize;
        while val < self.start || val > self.end {
            raw = dist.sample(rng);
            val = raw.round() as usize;
        }

        // Return
        val
    }
}
