use std::ops::{Add, AddAssign};

#[allow(unused, dead_code)]
#[derive(Debug, Clone, Default)]
pub struct KahanSum {
    sum: f32,
    err: f32,
}

pub trait KahanSummator {
    fn kahan_sum(self) -> KahanSum;
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

impl From<f32> for KahanSum {
    fn from(initial: f32) -> Self {
        KahanSum {
            sum: initial,
            err: 0_f32,
        }
    }
}

impl AddAssign<f32> for KahanSum {
    fn add_assign(&mut self, rhs: f32) {
        self.add(rhs);
    }
}

impl Add<f32> for KahanSum {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        let mut instance = self;
        instance += rhs;
        instance
    }
}

impl<It> KahanSummator for It
where It: Iterator<Item = f32>
{
    fn kahan_sum(self) -> KahanSum {
        self.fold(Default::default(), |sum, item| sum + item)
    }
}