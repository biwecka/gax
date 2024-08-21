// pub mod crossover;
// pub mod mutation;

pub fn pt1(y: f32, u: f32, t: f32) -> f32 {
    if t + 1.0 != 1.0 {
        y + ((u - y) / t)
    } else {
        u
    }
}
