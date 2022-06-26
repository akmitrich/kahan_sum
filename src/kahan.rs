#[allow(unused, dead_code)]

#[derive(Debug, Clone, Default)]
pub struct KahanSum {
    sum: f32,
    err: f32,
}

impl From<f32> for KahanSum {
    fn from(initial: f32) -> Self {
        KahanSum { sum: initial, err: 0_f32 }
    }
}

impl KahanSum {
    pub fn get_sum(&self) -> f32 {
        self.sum
    }

    pub fn get_err(&self) -> f32 {
        self.err
    }

    pub fn add(&mut self, value: f32) {
        let corrected = value - self.get_err();
        let new_sum = self.get_sum() + corrected;
        self.err = (new_sum - self.get_sum()) - corrected;
        self.sum = new_sum;
    }
}