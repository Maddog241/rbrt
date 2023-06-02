use rand::random;

pub struct Reservoir<T> {
    pub y: T, // the output sample
    pub weight_sum: f64, // sum of the weights
    pub m: u32,  // candidates count
}

impl<T> Reservoir<T> {
    pub fn new(initial_sample: T, initial_weight: f64) -> Self {
        Reservoir {
            y: initial_sample,
            weight_sum: initial_weight,
            m: 1,
        }
    }

    pub fn update(&mut self, x: T, w: f64) {
        self.weight_sum += w; 
        self.m += 1;

        if random::<f64>() < w / self.weight_sum {
            // update
            self.y = x;
        }
    }

    pub fn output_sample(self) -> T {
        self.y
    }
}