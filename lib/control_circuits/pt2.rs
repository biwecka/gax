// Structure to represent the PT2 control circuit
pub struct PT2 {
    t1: f64,   // Time constant T1
    t2: f64,   // Time constant T2
    zeta: f64, // Damping factor
    k: f64,    // Gain
    y: f64,    // Output
    dy: f64,   // First derivative of output (dy/dt)
    d2y: f64,  // Second derivative of output (d²y/dt²)
    dt: f64,   // Time step for simulation
}

impl PT2 {
    // Constructor to initialize the PT2 controller
    pub fn new(t1: f64, t2: f64, zeta: f64, k: f64, dt: f64) -> Self {
        PT2 { t1, t2, zeta, k, y: 0.0, dy: 0.0, d2y: 0.0, dt }
    }

    // Method to simulate the PT2 response for a given input signal
    pub fn update(&mut self, input: f64) {
        // Compute the second derivative using the PT2 differential equation
        self.d2y =
            (self.k * input - self.y - 2.0 * self.zeta * self.t1 * self.dy)
                / self.t2;

        // Integrate to find the first derivative and the output (Euler integration)
        self.dy += self.d2y * self.dt;
        self.y += self.dy * self.dt;
    }

    // Get the current output of the controller
    pub fn get_output(&self) -> f64 {
        self.y
    }
}
