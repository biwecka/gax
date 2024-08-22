use rand::rngs::ThreadRng;
use rand_distr::Distribution;

#[derive(Clone)]
pub struct DynamicBetaDistribution {
    start: usize,
    end: usize,

    pub std_deviation: f32,
}

impl DynamicBetaDistribution {
    pub fn new_inclusive(start: usize, end: usize, std_deviation: f32) -> Self {
        assert!(start < end);
        assert!(std_deviation <= 0.25);
        assert!(std_deviation > 0.);
        Self { start, end, std_deviation }
    }

    pub fn set_std_deviation(&mut self, std_deviation: f32) {
        // assert!(std_deviation <= 0.45);
        assert!(std_deviation <= 0.25);
        assert!(std_deviation > 0.);

        self.std_deviation = std_deviation;
    }

    pub fn sample(&self, expected_value: usize, rng: &mut ThreadRng) -> usize {
        assert!(expected_value >= self.start);
        assert!(expected_value <= self.end);

        // Map expected value from [start, end] to [0, 1]
        let exp_val = map_interval(self.start, self.end, expected_value);

        // Calculate parameters (alpha + beta) for the beta distribution
        let alpha = calc_alpha(exp_val, self.std_deviation).max(0.001);
        let beta = calc_beta(exp_val, alpha).max(0.001);

        // Create distribution
        let beta_dist = rand_distr::Beta::new(alpha, beta).unwrap();

        // Get random value
        let random_value = beta_dist.sample(rng);

        // Reverse mapping and return
        reverse_map_interval(self.start, self.end, random_value)
    }
}

fn calc_alpha(expected_value: f32, std_deviation: f32) -> f32 {
    let u = expected_value; // μ
    let o = std_deviation; // σ

    (((1. - u) * u * u) / (o * o)) - u
}

fn calc_beta(expected_value: f32, alpha: f32) -> f32 {
    let u = expected_value; // μ

    ((1. / u) - 1.) * alpha
}

fn map_interval(a: usize, b: usize, x: usize) -> f32 {
    assert!(a < b);

    if x == a {
        return 0.;
    } else if x == b {
        return 1.;
    }

    let offset = a;
    let diff = b - a;

    (x - offset) as f32 / diff as f32
}

fn reverse_map_interval(a: usize, b: usize, x: f32) -> usize {
    assert!(a < b);

    let offset = a;
    let diff = b - a;

    (x * (diff as f32)).round() as usize + offset
}
