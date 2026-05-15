use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Calculation {
    pub input: String,
    pub result: f64,
}

pub struct History {
    pub calculations: VecDeque<Calculation>,
    max_size: usize,
}

impl History {
    pub fn new(max_size: usize) -> Self {
        Self {
            calculations: VecDeque::new(),
            max_size,
        }
    }

    pub fn add_calculation(&mut self, input: String, result: f64) {
        let calc = Calculation { input, result };
        self.calculations.push_front(calc);
        if self.calculations.len() > self.max_size {
            self.calculations.pop_back();
        }
    }

    pub fn get_calculations(&self) -> &VecDeque<Calculation> {
        &self.calculations
    }

    pub fn clear(&mut self) {
        self.calculations.clear();
    }
}
