// Structure to represent the PT2 control circuit
#[derive(Clone)]
pub struct PT2 {
    /// Time Constant T1:
    /// This parameter defines how quickly the system reacts to changes. A
    /// smaller t1 makes the system respond faster, while a larger t1 causes a
    /// slower response. It is associated with the speed of the first-order
    /// dynamics.
    t1: f64,

    /// Time Constant T2:
    /// This determines how the system handles the higher-order dynamics (the
    /// second-order term). Like t1, a smaller t2 increases the system’s
    /// responsiveness to changes, while a larger t2 slows down the response.
    /// t2 adds a delay effect and can create a longer settling time.
    t2: f64,

    /// Damping Factor:
    /// The damping factor ζ controls how oscillations decay over time. It
    /// affects the stability and overshoot of the system:
    /// - ζ < 1 (Underdamped): The system will oscillate, and the amplitude of
    ///   these oscillations decreases gradually over time.
    /// - ζ = 1 (Critically Damped): The system responds as quickly as possible
    ///   without oscillating. This is often a desirable state in control
    ///   systems because it provides a fast, stable response.
    /// - ζ > 1 (Overdamped): The system responds slowly without oscillating
    ///   and may take longer to settle compared to a critically damped system.
    zeta: f64,

    /// Gain:
    /// The gain k amplifies the system's response to the input. A higher k
    /// increases the amplitude of the output, making the system more sensitive
    /// to inputs, but too high of a gain may also increase the chance of
    /// instability or overshooting.
    k: f64,

    /// Current Output:
    /// This is the system’s current output. The system’s goal is to adjust y
    /// in response to the input based on the dynamics controlled by the time
    /// constants, damping factor, and gain.
    y: f64,

    /// First derivative of the output (dy/dt)
    dy: f64,

    /// Second derivative of the output (d²y/dt²)
    d2y: f64,

    /// Time step for simulation:
    /// The time step dt defines the granularity of the simulation. Smaller
    /// time steps provide more accurate approximations of the system's
    /// dynamics but require more computational power. Larger time steps may
    /// result in less accurate simulations but faster computation.
    dt: f64,
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
