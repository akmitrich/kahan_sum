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
}