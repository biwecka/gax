pub struct PT1 {
    t: f64,  // Time constant T
    k: f64,  // Gain
    y: f64,  // Output
    dt: f64, // Time step for simulation
}

impl PT1 {
    // Constructor to initialize the PT1 controller
    pub fn new(t: f64, k: f64, dt: f64) -> Self {
        PT1 { t, k, y: 0.0, dt }
    }

    // Method to simulate the PT1 response for a given input signal
    pub fn update(&mut self, input: f64) {
        // Compute the derivative of y using the PT1 differential equation
        let dy = (self.k * input - self.y) / self.t;

        // Integrate to find the output (Euler integration)
        self.y += dy * self.dt;
    }

    // Get the current output of the controller
    pub fn get_output(&self) -> f64 {
        self.y
    }
}
