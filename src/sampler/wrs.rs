use rand::random;

/// a reservoir with only one output
pub struct Reservoir<T: Clone> {
    y: Option<T>, // the output sample
    pub weight_sum: f64, // sum of the weights
    pub m: u32,  // candidates count
}

impl<T: Clone> Reservoir<T> {
    pub fn new() -> Self {
        Reservoir {
            y: None,
            weight_sum: 0.0,
            m: 0,
        }
    }

    pub fn update(&mut self, x: T, w: f64) {
        self.weight_sum += w; 
        self.m += 1;
        if self.y.is_none() || random::<f64>() < w / self.weight_sum {
            // update
            self.y = Some(x);
        }
   }

    /// if the reservoir is empty, panic
    pub fn output_sample(&self) -> T {
        assert!(self.y.is_some());
        self.y.clone().unwrap()
    }
}