pub mod kahan;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_example() {
        let mut ks = kahan::KahanSum::default();
        ks.add(16777216_f32);
        assert_eq!(16777216., ks.get_sum());
        assert_eq!(0., ks.get_err());
        ks.add(1_f32);
        assert_eq!(16777216., ks.get_sum());
        assert_eq!(-1., ks.get_err());
        ks.add(1_f32);
        assert_eq!(16777218., ks.get_sum());
        assert_eq!(0., ks.get_err());
    }
}
